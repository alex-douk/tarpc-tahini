use std::{
    net::{IpAddr, Ipv4Addr},
    sync::{Arc, OnceLock, RwLock},
};

use futures::future::Future;
use futures::StreamExt;
use tahini_tarpc::{
    client::new,
    server::{TahiniBaseChannel, TahiniChannel, TahiniServe},
    transport::{new_tahini_server_transport},
};
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;

mod policy;
mod service;

use crate::service::{SimpleService, SimpleServiceServer};

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

pub struct SimpleServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup for connection
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 5003)).await.unwrap();

    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        println!("Accepted a connection");

        // Building the transport channel
        let codec_builder = LengthDelimitedCodec::builder();
        let framed = codec_builder.new_framed(stream);

        // Bincode represents the codec for ser/de over the wire.
        // let transport = new_transport(framed, Bincode::default());
        // FIXME: Compiles but does not work because it expects a sidecar for the key session
        // management!!!!
        let transport =
            new_tahini_server_transport(framed, Json::default(), Arc::new(RwLock::default()));
        let server = TahiniBaseChannel::with_defaults(transport);
        tokio::spawn(
            server
                .execute(SimpleServiceServer.serve())
                .for_each(wait_upon),
        );
    }
}
