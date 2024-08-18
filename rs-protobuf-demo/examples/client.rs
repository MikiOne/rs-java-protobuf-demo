// use anyhow::Result;
// use async_prost::AsyncProstStream;
// use futures::prelude::*;
// use tokio::net::TcpStream;
// use tracing::info;
// use tracing::instrument::WithSubscriber;
// use tracing_subscriber::{EnvFilter, fmt, Registry};
// use tracing_subscriber::fmt::Layer;
// use tracing_subscriber::layer::SubscriberExt;
// use tracing_subscriber::util::SubscriberInitExt;
//
// // use rskv::{CommandRequest, CommandResponse};
//
// #[tokio::main]
// async fn main() -> Result<()> {
//     let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
//     Registry::default().with(filter).with(Layer::new().with_line_number(true)
//         .with_thread_ids(true).with_thread_names(true)
//     ).init();
//
//     // tracing_subscriber::fmt::init();
//
//     let addr = "127.0.0.1:9527";
//     // 连接服务器
//     let stream = TcpStream::connect(addr).await?;
//
//     // 使用 AsyncProstStream 来处理 TCP Frame
//     let mut client =
//         AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();
//
//     // 生成一个 HSET 命令
//     let cmd = CommandRequest::new_hset("table1", "hello", "world".into());
//     client.send(cmd).await?;
//
//     let cmd = CommandRequest::new_hset("table1", "hello", "world1".into());
//     // 发送 HSET 命令
//     client.send(cmd).await?;
//
//     while let Some(Ok(data)) = client.next().await {
//         info!("Got response {:?}", data);
//     }
//
//     Ok(())
// }
fn main() {}