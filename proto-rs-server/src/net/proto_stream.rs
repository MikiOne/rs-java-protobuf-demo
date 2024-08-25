use bytes::BytesMut;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::net::frame::{FrameCodec, read_frame};

struct ProtoStream<S, C> {
    inner: S,
    codec: C,
}

impl<S, C> ProtoStream<S, C>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    C: FrameCodec, //+ Message + Default + Send + 'static,
{
    pub fn new(inner: S, codec: C) -> Self {
        Self { inner, codec }
    }

    pub async fn recv(&mut self) -> anyhow::Result<C> {
        let mut buf = BytesMut::new();
        read_frame(&mut self.inner, &mut buf).await?;
        Ok(C::decode_frame(&mut buf)?)
    }

    pub async fn send(&mut self) -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        self.codec.encode_frame(&mut buf).await?;
        self.inner.write_all(&buf).await?;
        Ok(())
    }
}
