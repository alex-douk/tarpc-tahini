#![feature(auto_traits, negative_impls, min_specialization)]
//Clone model just clones the reference
use core_tahini_utils::{
    policies::{InferenceReason, MessagePolicy},
    types::{BBoxConversation, Message},
};
use std::{collections::HashMap, sync::Arc};
//Required for model locking across async tasks
use tokio::sync::Mutex;

//Channel transport Code
use alohomora::tarpc::server::{TahiniBaseChannel, TahiniChannel};
use futures::{
    Future, StreamExt,
    future::{self, Ready},
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
use alohomora::context::UnprotectedContext;
use alohomora::fold::fold;
use alohomora::pcr::{PrivacyCriticalRegion as PCR, Signature};
use alohomora::policy::{Policy, Reason};
use alohomora::pure::PrivacyPureRegion as PPR;


//Inference import
//Internal LLM functionings
mod model_backend;
mod token_output_stream;
use crate::model_backend::{TextGeneration, create_pipeline};
use anyhow::Error as E;

//Tarpc + types


use core_tahini_utils::funcs::parse_conversation;
use llm_tahini_utils::service::Inference;
use core_tahini_utils::types::{LLMError, LLMResponse, UserPrompt};

//Database import

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
static SYSTEM_PROMPT: &str = "<start_of_turn>user
You are a helpful assistant. You should reply to the user demands in a thoughtful yet concise manner. Do not go beyond the user's initial requests. Remain on topic, truthful, and accurate. Keep your response simple and short. Only answer the initial requests. Do not ask questions.<end_of_turn>";

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


fn apply_chat_template(
    conversation: BBoxConversation,
) -> Result<PCon<String, MessagePolicy>, LLMError> {
    conversation
        .into_ppr(PPR::new(|rounds: Vec<Message>| {
            let parsed = parse_conversation(rounds);
            match parsed {
                Ok(conv) => Ok(format!("{}\n{}", SYSTEM_PROMPT, conv)),
                Err(e) => Err(e),
            }
        }))
        .transpose()
}

impl Inference for InferenceServer {
    async fn inference(self, _context: tarpc::context::Context, prompt: UserPrompt) -> LLMResponse {
        let pol = prompt.conversation.policy().clone();

        let mut locked_model = self.model.lock_owned().await;

        let conversation = apply_chat_template(prompt.conversation);
        match conversation {
            Err(e) => {
                println!("Got an error parsing the conversation, we failed?");

                return LLMResponse {
                    infered_tokens: PCon::new(Err(e), pol.clone()),
                };
            }
            _ => (),
        };
        let parsed_conversation = conversation.unwrap();

        let inf = PPR::new(move |unboxed_prompt: String| {
            locked_model.run(unboxed_prompt.as_str(), prompt.nb_token as usize)
        });

        // Keeping it here in case i ever need it later
        // let mut writer = Vec::with_capacity(128);
        // let mut ser = serde_json::ser::Serializer::new(&mut writer);
        // let _ =prompt.serialize(&mut ser);
        // println!("Using naive serializer, we get : {:?}", String::from_utf8(writer));
        let boxed_response = parsed_conversation.into_ppr(inf).transpose();

        match boxed_response {
            Err(e) => {
                eprintln!("Got error {}", e);
                LLMResponse {
                    infered_tokens: PCon::new(Err(LLMError::InternalError), pol.clone()),
                }
            }
            Ok(boxed_infered) => {
                // send_to_marketing(prompt.user.clone(), full_conv.clone()).await;
                LLMResponse {
                    infered_tokens: boxed_infered.into_ppr(PPR::new(|x| {
                        Ok(Message {
                            // role: "assistant".to_string(),
                            role: "model".to_string(),
                            content: x,
                        })
                    })),
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
