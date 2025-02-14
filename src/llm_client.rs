use futures::{Future, StreamExt};
use tarpc;
use tarpc::context::Context;
use tarpc::serde_transport::new as new_transport;
use tarpc::server::Channel;

use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::BaseChannel;
use tokio::net::TcpStream;
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

use rpc::inference::{Inference, InferenceClient};
use rpc::database::{Database, DatabaseClient};
// use rpc::serde::json::Json;

use alohomora::{bbox::BBox, policy::NoPolicy};
use alohomora::pcr::{PrivacyCriticalRegion, Signature};


use crate::policies::PromptPolicy;
use crate::types::inference_types::{UserPrompt, LLMResponse};

pub mod rpc;
pub mod types;
pub mod policies;


static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let policy = PromptPolicy {
        consent: true
    };
    let test_prompt = UserPrompt{
        user: "alex".to_string(),
        prompt: BBox::new("Say to me a funny joke.".to_string(), policy),
        nb_token: 30
    };
    
        let codec_builder = LengthDelimitedCodec::builder();

        let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await?;

        let transport = new_transport(codec_builder.new_framed(stream), Json::default());

        let response = InferenceClient::new(Default::default(), transport)
            .spawn()
            .inference(tarpc::context::current(), test_prompt)
            .await?;
    //
    // Print output to screen.
    response.infered_tokens.into_pcr(
        PrivacyCriticalRegion::new(
            |v, p, _c| {
                println!("[ASSISTANT]: {}", v);
            },
            Signature { username: "", signature: "" },
            Signature { username: "", signature: "" },
            Signature { username: "", signature: "" },
        ),
        ()
    );

    let codec_builder = LengthDelimitedCodec::builder();

    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await?;

    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let retrieved_conv = DatabaseClient::new(Default::default(), transport)
        .spawn()
        .retrieve_prompt(Context::current(), "alex".to_string(), response.db_uuid)
        .await?;


    retrieved_conv.full_prompt.into_pcr(
        PrivacyCriticalRegion::new(
            |v, p, _c| {
                println!("Fetched conv : {}", v);
            },
            Signature { username: "", signature: "" },
            Signature { username: "", signature: "" },
            Signature { username: "", signature: "" },
        ),
        ()
    );
    // println!("{}", response);
    Ok(())
}
