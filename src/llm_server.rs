#![feature(auto_traits, negative_impls, min_specialization)]
//Stream abstractions
use std::{pin::Pin, sync::Arc};
//Internal LLM functionings
use crate::model_backend::{create_pipeline, TextGeneration};
use crate::types::{LLMResponse, UserPrompt};
use anyhow::Error as E;

use serde::Serialize;
//Required for model locking across async tasks
use tokio::sync::Mutex; // Tokio's async Mutex
                        //
use futures::{Future, StreamExt, future::{self, Ready}};
use tarpc::serde_transport::new as new_transport;
use tarpc::server::Channel;
use tarpc::tokio_serde::Serializer;

use crate::rpc::Inference;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::BaseChannel;
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
// use crate::rpc::serde::json::Json as BBoxJson;
use alohomora::{bbox::BBox, policy::NoPolicy, tarpc_serde::json::Json};
use alohomora::pure::PrivacyPureRegion;

mod model_backend;
mod rpc;
mod token_output_stream;
mod types;

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

impl Inference for InferenceServer {
    // type InferenceFut = Pin<dyn Future<Output = String>>;
    // type InferenceFut = Ready<LLMResponse>;
    // type InferenceFut = Ready<String>;

    async fn inference(self, _context: tarpc::context::Context, prompt: UserPrompt) -> LLMResponse {
        println!("Got a request");
        let mut locked_model = self.model.lock_owned().await;
        let inf = PrivacyPureRegion::new( move |unboxed_prompt: String| { 
            locked_model.run(unboxed_prompt.as_str(), prompt.nb_token as usize)
        });

        // //TODO: Douk:  Hide the serializer behind the transport instantation!!
        // let serializer: Pin<&mut Json< UserPrompt, UserPrompt>>  = std::pin::pin!(Json::default());
        // let bytes = serializer.serialize(&prompt).expect("Tried to serialize when impossible");
        //
        // println!("In application, we serialized {:?}", String::from_utf8(bytes.to_vec()));


        let mut writer = Vec::with_capacity(128);
        let mut ser = serde_json::ser::Serializer::new(&mut writer);
        let _ =prompt.serialize(&mut ser);
        println!("Using naive serializer, we get : {:?}", String::from_utf8(writer));
        let boxed_response = prompt.prompt.into_ppr(inf);

        let rsp = LLMResponse {
            infered_tokens : boxed_response.into_ppr(PrivacyPureRegion::new(|unboxed_rsp: Result<String, E>| {
                match unboxed_rsp {
                    Ok(tokens) => tokens,
                    Err(e) => e.to_string()
                }
            }))
        };
        rsp
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output =  ()> + Send + 'static) {
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
                let fut = BaseChannel::with_defaults(transport)
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
