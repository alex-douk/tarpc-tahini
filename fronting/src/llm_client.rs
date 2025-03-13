use futures::{Future, StreamExt};
use tarpc::client::RpcError;
use tarpc::context::Context;
use tarpc::serde_transport::new as new_transport;
use tarpc::server::Channel;
use services_utils::types::database_types::{DatabaseRecord, DBUUID};

use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::BaseChannel;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

use services_utils::rpc::database::{Database, TahiniDatabaseClient};
use services_utils::rpc::inference::{Inference, TahiniInferenceClient};
// use rpc::serde::json::Json;

use alohomora::pcr::{PrivacyCriticalRegion, Signature};
use alohomora::{bbox::BBox, policy::NoPolicy};

use services_utils::policies::PromptPolicy;
use services_utils::types::inference_types::{LLMResponse, UserPrompt};


static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

async fn get_entry(user: String, uuid: DBUUID) -> Result<Option<DatabaseRecord>, RpcError> {
    let codec_builder = LengthDelimitedCodec::builder();

    let stream = TcpStream::connect((SERVER_ADDRESS, 5002))
        .await
        .expect("Couldn't connect to database");

    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let retrieved_conv = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .retrieve_prompt(Context::current(), user, uuid)
        .await?;

    Ok(retrieved_conv)
}

fn parse_conv(conv: Option<DatabaseRecord>) {
    match conv {
        None => println!("Fetched failed"),
        Some(res) => res.full_prompt.into_pcr(
            PrivacyCriticalRegion::new(
                |v, p, _c| {
                    println!("Fetched conv : {}", v);
                },
                Signature {
                    username: "",
                    signature: "",
                },
                Signature {
                    username: "",
                    signature: "",
                },
                Signature {
                    username: "",
                    signature: "",
                },
            ),
            (),
        ),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let policy = PromptPolicy { consent: true };
    let test_prompt = UserPrompt {
        user: "alex".to_string(),
        prompt: BBox::new("Say to me a funny joke.".to_string(), policy),
        nb_token: 30,
    };

    let codec_builder = LengthDelimitedCodec::builder();

    let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await?;

    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let response = TahiniInferenceClient::new(Default::default(), transport)
        .spawn()
        .inference(tarpc::context::current(), test_prompt)
        .await?;
    //
    // Print output to screen.

    match response.infered_tokens.transpose() {
        Ok(tokens) => {
            tokens.into_pcr(
                PrivacyCriticalRegion::new(
                    |v, p, _c| {
                        println!("[ASSISTANT]: {}", v);
                    },
                    Signature {
                        username: "",
                        signature: "",
                    },
                    Signature {
                        username: "",
                        signature: "",
                    },
                    Signature {
                        username: "",
                        signature: "",
                    },
                ),
                (),
            );
        }
        Err(e) => println!("Got error : {}", e.to_string()),
    }

    match response.db_uuid.clone().transpose() {
        None => println!("No uuid, can't retrieve"),
        Some(uuid) => {
            let true_prompt = get_entry("alex".to_string(), uuid)
                .await
                .expect("Database retrieval failed");
            parse_conv(true_prompt);
        }
    }

    println!("Sending wrong data");
    match response.db_uuid.clone().transpose() {
        None => println!("No uuid, can't retrieve"),
        Some(uuid) => {
            let no_conv = get_entry("malte".to_string(), uuid)
                .await
                .expect("Database retrieval failed");
            parse_conv(no_conv);
        }
    }
    Ok(())
}
