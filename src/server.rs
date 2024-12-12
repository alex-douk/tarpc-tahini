use futures::{Future, StreamExt};
use tarpc;
use tarpc::serde_transport::new as new_transport;
use tarpc::server::Channel;

use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::BaseChannel;
use tokio::net::TcpListener;
use tokio_serde::formats::Bincode;
use tokio_util::codec::LengthDelimitedCodec;

use rpc::{InferenceServer, Inference};

pub mod rpc;
pub mod types;

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let server_task = get_server_transport!(InferenceServer, SERVER_ADDRESS, 5000);

    let _ = tokio::spawn(server_task).await;
    Ok(())
}
