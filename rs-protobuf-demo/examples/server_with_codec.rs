use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use rs_protobuf_demo::pb::student::Student;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    Registry::default().with(filter).with(Layer::new().with_line_number(true)
        .with_thread_ids(true).with_thread_names(true)
    ).init();

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);

    // new memory table
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);

        tokio::spawn(async move {
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {
                let stu = Student::decode(&buf[..]).unwrap();
                info!("Got a Student info: {:?}", stu);
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}