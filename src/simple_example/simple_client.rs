mod service;

use std::net::{Ipv4Addr, IpAddr};
use tokio::net::TcpListener;
use tarpc::serde_transport::new as new_transport;
use tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;
use tarpc::tokio_serde::formats::Bincode;
use alohomora::{bbox::BBox, policy::NoPolicy, tarpc_serde::json::Json};
use alohomora::pure::PrivacyPureRegion;
use service::*;

use alohomora::tarpc::TahiniSimpleClient;

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

static SENSITIVE_VALUE: i32 = 987654321;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect((SERVER_ADDRESS, 5003)).await?;

    //Build transport channel: Code is similar to the server side
    let codec_builder = LengthDelimitedCodec::builder();
    let transport = new_transport(codec_builder.new_framed(stream), Bincode::default());
    let response = TahiniSimpleClient::new(Default::default(), transport)
        .spawn()
        // .increment(tarpc::context::current(), SENSITIVE_VALUE)
        .increment(tarpc::context::current(), BBox::new(SENSITIVE_VALUE, NoPolicy::default()))
        .await?;
    println!("Increment is a PCon, but NoPolicy allows us to see inside : {}", response.discard_box());
    // println!("Increment is {}", response);
    Ok(())
}
