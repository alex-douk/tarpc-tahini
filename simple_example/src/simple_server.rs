use std::net::{Ipv4Addr, IpAddr};

use tahini_tarpc::{server::{TahiniBaseChannel, TahiniChannel, TahiniServe}, transport::new_tahini_transport};
use futures::future::Future;
use futures::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;

mod service;
mod policy;

use crate::service::{SimpleService, SimpleServiceServer};

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

async fn wait_upon(fut: impl Future<Output =  ()> + Send + 'static) {
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
        let transport = new_tahini_transport(framed, Json::default());
        let server = TahiniBaseChannel::with_defaults(transport);
        tokio::spawn(server.execute(SimpleServiceServer.serve()).for_each(wait_upon));

    }
}
