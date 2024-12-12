use futures::{Future, StreamExt};
use tarpc;
use tarpc::serde_transport::new as new_transport;
use tarpc::server::Channel;

use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::BaseChannel;
use tokio::net::TcpStream;
use tarpc::tokio_serde::formats::Bincode;
use tokio_util::codec::LengthDelimitedCodec;

use rpc::{Inference, InferenceClient, DummyBox};
// use rpc::serde::json::Json;

use alohomora::{bbox::BBox, policy::NoPolicy, tarpc_serde::json::Json};

use types::UserPrompt;

pub mod rpc;
pub mod types;

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let test_prompt = UserPrompt{
        user: "alex".to_string(),
        prompt: BBox::new("Say to me a funny joke.".to_string(), NoPolicy::default()),
        nb_token: 30
    };
    
        let codec_builder = LengthDelimitedCodec::builder();

        let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await?;

        let transport = new_transport(codec_builder.new_framed(stream), Json::default());

        let response = InferenceClient::new(Default::default(), transport)
            .spawn()
            .inference(tarpc::context::current(), test_prompt)
            .await?;
    println!("{}", response.infered_tokens.discard_box());
    // println!("{}", response);
    Ok(())

}
