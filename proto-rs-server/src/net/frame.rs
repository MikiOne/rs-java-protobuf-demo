use std::io::{Read, Write};

use bytes::{Buf, BufMut, BytesMut};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::debug;

use crate::pb::student::Student;
use crate::student::student;

/// 帧头长度
pub const LEN_LEN: usize = 4;
/// 最大帧大小
pub const MAX_FRAME: usize = 2 * 1024 * 1024 * 1024;
/// 压缩阈值，小于等于该值则不压缩
pub const COMPRESSION_LIMIT: usize = 1436;
/// 压缩标志位（最高位为1代表压缩）
pub const COMPRESSION_BIT: usize = 1 << 31;

pub trait FrameCodec
where
    Self: Message + Sized + Default,
{
    fn encode_frame(&self, buf: &mut BytesMut) -> anyhow::Result<()> {
        let len = self.encoded_len();

        // 如果大于最大帧，则拒绝处理
        if len > MAX_FRAME {
            return Err(anyhow::anyhow!("frame too large"));
        }

        // 将长度写入buf
        buf.put_u32(len as u32);
        // 如果长度大于压缩阈值，则压缩
        if len > COMPRESSION_LIMIT {
            // 将self读入buf1中
            let mut buf1 = Vec::with_capacity(len);
            self.encode(&mut buf1)?;

            // 将buf分离出payload
            let mut payload = buf.split_off(LEN_LEN);
            buf.clear();

            // 使用gzip压缩buf1，在构建GzEncoder时使用payload.writer()，默认压缩级别为6
            let mut gz = GzEncoder::new(payload.writer(), Compression::default());
            gz.write_all(&buf1)?;
            payload = gz.finish()?.into_inner();
            println!("origin len: {}, compressed: {}", len, payload.len());

            // 写入压缩标志位机payload size
            // 注意：这里的payload size是指payload的长度，而不是整个buf的长度
            buf.put_u32((payload.len() | COMPRESSION_BIT) as u32);

            // 将payload unsplit到buf
            buf.unsplit(payload);
        } else {
            // 将self写入buf
            self.encode(buf)?;
        }
        Ok(())
    }

    fn decode_frame(buf: &mut BytesMut) -> anyhow::Result<Self> {
        // 先取 4 字节，从中拿出长度和 compression bit
        let header = buf.get_u32();

        let (len, compressed) = decode_header(header as usize);
        debug!("Got a frame: msg len {}, compressed {}", len, compressed);

        if compressed {
            // 读到buf1中
            let mut buf1 = Vec::with_capacity(len * 2);
            let mut decoder = GzDecoder::new(&buf[..len]);
            decoder.read_to_end(&mut buf1)?;

            buf.advance(len);
            Ok(Self::decode(&buf1[..buf1.len()])?)
        } else {
            // buf.advance(LEN_LEN);
            Ok(Self::decode(buf)?)
        }
    }
}

/// 从帧头中解析出长度和压缩标志位
pub fn decode_header(header: usize) -> (usize, bool) {
    let len = header & !COMPRESSION_BIT;
    let compressed = header & COMPRESSION_BIT == COMPRESSION_BIT;
    (len, compressed)
}

impl FrameCodec for Student {}

// 异步读取帧
pub async fn read_frame<S>(mut stream: S, buf: &mut BytesMut) -> anyhow::Result<()>
where
    S: AsyncRead + Unpin + Send,
{
    let header = stream.read_u32().await?;
    let (len, _compressed) = decode_header(header as usize);
    debug!("Got a frame: msg len {}", len);

    buf.reserve(LEN_LEN + len);     // 假设len = 13
    println!("read_frame: buf len: {}", buf.len()); // 0

    buf.put_u32(header);
    println!("read_frame: buf len: {}", buf.len()); // 4

    unsafe {
        buf.advance_mut(len);
    }
    println!("read_frame: buf len: {}", buf.len()); // 13
    stream.read_exact(&mut buf[LEN_LEN..]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use tokio::io::ReadBuf;

    use crate::pb::student::Student;

    use super::*;

    struct DummyStream {
        buf: BytesMut,
    }

    impl AsyncRead for DummyStream {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            // 看看 ReadBuf 需要多大的数据
            let size = buf.capacity();

            // split 出这么大的数据
            let data = self.get_mut().buf.split_to(size);

            // 拷贝给 ReadBuf
            buf.put_slice(&data);

            // 直接完工
            Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn read_frame_should_work() -> anyhow::Result<()> {
        let mut student = Student::default();
        student.id = 101;
        student.first_name = "hello".to_string();

        let mut buf = BytesMut::with_capacity(1024);
        student.encode_frame(&mut buf)?;
        let size = buf.len();
        let other = buf.split_off(buf.len());
        drop(other);

        let mut stream = DummyStream { buf };
        let mut buf_stream = BytesMut::with_capacity(1024);
        read_frame(&mut stream, &mut buf_stream).await.unwrap();

        let student_stream = Student::decode_frame(&mut buf_stream)?;
        assert_eq!(student, student_stream);
        Ok(())
    }


    #[test]
    fn student_encode_decode_should_work() -> anyhow::Result<()> {
        let mut student = Student::default();
        student.id = 101;
        student.first_name = "hello".to_string();

        let mut buf = BytesMut::with_capacity(1024);
        student.encode_frame(&mut buf)?;

        let student2 = Student::decode_frame(&mut buf)?;
        assert_eq!(student, student2);
        Ok(())
    }

    // encode_decode的压缩测试，压缩阈值设置为1436，即大于1436字节的消息才会被压缩
    #[test]
    fn student_compression_encode_decode_should_work() -> anyhow::Result<()> {
        let name = vec![0u8; COMPRESSION_LIMIT + 1];
        let mut student = Student::default();
        student.id = 101;
        student.first_name = String::from_utf8(name).unwrap();

        let mut buf = BytesMut::new();
        student.encode_frame(&mut buf)?;

        let student2 = Student::decode_frame(&mut buf)?;
        assert_eq!(student, student2);
        Ok(())
    }

    #[test]
    fn test_frame_codec() -> anyhow::Result<()> {
        let mut student = Student::default();
        student.id = 1;
        student.first_name = "hello".to_string();

        let mut buf = BytesMut::with_capacity(1024);
        println!("{:#x}", buf);
        student.encode_frame(&mut buf)?;
        println!("{:#x}", buf);

        // assert_eq!(student, Student::frame_decoder(&mut buf)?);
        Ok(())
    }
}

// pub trait FrameCodec
// where
//     Self: Message + Sized + Default,
// {
//     fn encode_frame(&mut self, buf: &mut BytesMut) -> anyhow::Result<()> {
//         let len = self.encoded_len();
//
//         // 如果大于最大帧，则拒绝处理
//         if len > MAX_FRAME {
//             return Err(anyhow::anyhow!("frame too large"));
//         }
//         // 将长度写入buf
//         buf.put_u32(len as u32);
//
//         // 如果长度大于压缩阈值，则压缩
//         if len > COMPRESSION_LIMIT {
//             // 将self读入buf1中
//             let mut buf1 = Vec::with_capacity(len);
//             self.encode(&mut buf1[..len])?;
//
//             // 将buf分离出payload
//             let mut payload = buf.split_off(LEN_LEN);
//             buf.clear();
//
//             // 使用gzip压缩buf1，在构建GzEncoder时使用payload.writer()，默认压缩级别为6
//             let mut gz = GzEncoder::new(payload.writer(), Compression::default());
//             gz.write_all(&buf1)?;
//             payload = gz.finish()?.into_inner();
//
//             // 写入压缩标志位机payload size
//             // 注意：这里的payload size是指payload的长度，而不是整个buf的长度
//             buf.put_u32((payload.len() | COMPRESSION_BIT) as u32);
//
//             // 将payload unsplit到buf
//             buf.unsplit(payload);
//         } else {
//             // 将self写入buf
//             self.encode(buf)?;
//         }
//         Ok(())
//     }
// // error[E0277]: the trait bound `[_]: BufMut` is not satisfied
// //   --> src/net/frame.rs:97:25
// //    |
// // 97 |             self.encode(&mut buf1[..len])?;
// //    |                  ------ ^^^^^^^^^^^^^^^^ the trait `BufMut` is not implemented for `[_]`
// //    |                  |
// //    |                  required by a bound introduced by this call
// //    |
// //    = help: the following other types implement trait `BufMut`:
// //              &mut [MaybeUninit<u8>]
// //              &mut [u8]
// // note: required by a bound in `prost::Message::encode`
//     fn decode_frame(buf: &mut BytesMut) -> anyhow::Result<Self> {
//         // 如果长度大于最大帧，则拒绝处理
//         if buf.len() > MAX_FRAME{
//             return Err(anyhow::anyhow!("frame too large"));
//         }
//
//         // 读取header
//         let header = buf.get_u32();
//         let (len, compressed) = decode_header(header as usize);
//
//         if compressed {
//             let mut gz = GzDecoder::new(&mut buf[LEN_LEN..]);
//             let mut buf1 = Vec::with_capacity(len * 2);
//             gz.read_to_end(&mut buf1)?;
//
//             let student = Self::decode(&mut buf1[..len])?;
//             buf.advance(buf.len());
//             Ok(student)
//         } else {
//             let student = Self::decode(&mut buf[LEN_LEN..])?;
//             buf.advance(LEN_LEN + len);
//             Ok(student)
//         }
//     }
// }
