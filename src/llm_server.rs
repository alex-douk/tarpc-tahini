#![feature(auto_traits, negative_impls, min_specialization)]
//Clone model just clones the reference
use rpc::database::TahiniDatabaseClient;
use std::sync::Arc;
//Required for model locking across async tasks
use tokio::sync::Mutex;

//Channel transport Code
use alohomora::{
    pure::execute_pure,
    tarpc::server::{TahiniBaseChannel, TahiniChannel},
};
use futures::{
    future::{self, Ready},
    Future, StreamExt,
};
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

//Sesame basics
use alohomora::bbox::BBox as PCon;
use alohomora::fold::fold;
use alohomora::pure::PrivacyPureRegion as PPR;

//Application-wide mods
mod policies;
mod rpc;
mod types;
use crate::policies::PromptPolicy;

//Inference import
//Internal LLM functionings
mod model_backend;
mod token_output_stream;
use crate::model_backend::{create_pipeline, TextGeneration};
use anyhow::Error as E;

//Tarpc + types
use crate::rpc::inference::Inference;
use crate::types::inference_types::{LLMResponse, UserPrompt};

//Database import
use crate::types::database_types::{DatabaseSubmit, DBUUID};

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[derive(Clone)]
pub struct InferenceServer {
    model: Arc<Mutex<model_backend::TextGeneration>>,
}

impl InferenceServer {
    pub fn new(tg: TextGeneration) -> Self {
        InferenceServer {
            model: Arc::new(Mutex::new(tg)),
        }
    }
}

async fn store_to_database(user: String, prompt: PCon<String, PromptPolicy>) -> Option<DBUUID> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let payload = DatabaseSubmit {
        user,
        full_prompt: prompt,
    };

    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .store_prompt(tarpc::context::current(), payload)
        .await;

    match response {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}

impl Inference for InferenceServer {
    async fn inference(self, _context: tarpc::context::Context, prompt: UserPrompt) -> LLMResponse {
        println!("Got a request");

        let prompt_copy = prompt.prompt.clone();
        let pol = prompt_copy.policy().clone();

        let mut locked_model = self.model.lock_owned().await;

        let inf = PPR::new(move |unboxed_prompt: String| {
            locked_model.run(unboxed_prompt.as_str(), prompt.nb_token as usize)
        });

        // Keeping it here in case i ever need it later
        // let mut writer = Vec::with_capacity(128);
        // let mut ser = serde_json::ser::Serializer::new(&mut writer);
        // let _ =prompt.serialize(&mut ser);
        // println!("Using naive serializer, we get : {:?}", String::from_utf8(writer));
        let boxed_response = prompt.prompt.into_ppr(inf).transpose();

        match boxed_response {
            Err(e) => LLMResponse {
                //Can allow a fronting webserver to return a 500
                infered_tokens: PCon::new(
                    Err(types::inference_types::LLMError::InternalError),
                    pol.clone(),
                ),
                db_uuid: PCon::new(None::<u32>, pol.clone()),
            },
            Ok(boxed_infered) => {
                let pair =
                    fold((prompt_copy, boxed_infered.clone())).expect("Failed to combine PCons");
                let full_conv = pair
                    .into_ppr(PPR::new(|pair: (String, String)| {
                        format!("[USER]: {}\n[ASSISTANT]{}", pair.0, pair.1)
                    }))
                    .specialize_policy::<PromptPolicy>()
                    .expect("Failed to specialize policy");

                let uuid = store_to_database(prompt.user, full_conv).await;
                
                //Man just makes this easier already. 
                let some_uuid = match uuid {
                    Some(b) => b.into_ppr(PPR::new(|x| Some(x))),
                    None => PCon::new(None::<u32>, pol.clone())
                };
                LLMResponse {
                    infered_tokens: boxed_infered.into_ppr(PPR::new(|x| Ok(x))),
                    db_uuid: some_uuid,
                }
            }
        }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the LLM inference server!");
    let pipeline = create_pipeline();
    match pipeline {
        Ok(model) => {
            println!("Successfully created the pipeline!");

            let listener = TcpListener::bind(&(SERVER_ADDRESS, 5000)).await.unwrap();
            let codec_builder = LengthDelimitedCodec::builder();
            let server = InferenceServer {
                model: Arc::new(Mutex::new(model)),
            };
            loop {
                let (stream, _peer_addr) = listener.accept().await.unwrap();
                println!("Accepted a connection");
                let framed = codec_builder.new_framed(stream);

                let transport = new_transport(framed, Json::default());

                // let transport = new_transport(framed, Bincode::default());
                let fut = TahiniBaseChannel::with_defaults(transport)
                    // .execute(server.serve());
                    .execute(server.clone().serve())
                    .for_each(wait_upon);
                tokio::spawn(fut);
            }
        }
        Err(ref x) => println!("Failed at creating the pipeline with error {:?}", x),
    }

    Ok(())
}
