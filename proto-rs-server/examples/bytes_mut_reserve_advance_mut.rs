use bytes::{BytesMut, BufMut};

fn main() {
    // 创建一个初始容量为64字节的BytesMut
    let mut buf = BytesMut::with_capacity(64);

    // 假设我们知道接下来会读取10个字节的数据
    let additional_bytes = 10;

    // 预留额外的内存空间
    buf.reserve(additional_bytes);

    // 模拟从网络流中读取数据
    let data: &[u8] = b"abcdefghij"; // 假设这是从网络读取的数据
    // buf.put_slice(data); // 将数据放入buf中
    buf.chunk_mut()[..additional_bytes].copy_from_slice(data); // 将数据放入buf中
    unsafe {
        buf.advance_mut(additional_bytes); // 将写指针向前移动5个字节
    }

    // 确认buf的长度
    assert_eq!(buf.len(), 10);

    // 使用advance_mut移动写指针
    let len_to_advance = 5;
    unsafe {
        buf.advance_mut(len_to_advance); // 将写指针向前移动5个字节
    }

    // // 现在buf的内容是"fghij"，长度为5
    // assert_eq!(&buf[len_to_advance..10], b"fghij");

    // 再次读取更多数据
    let more_data: &[u8] = b"klmnop"; // 假设这是从网络读取的更多数据
    buf.put_slice(more_data); // 将更多数据放入buf中

    // 现在buf的内容是"fghijklmnop"
    assert_eq!(&buf[..], b"fghijklmnop");
}