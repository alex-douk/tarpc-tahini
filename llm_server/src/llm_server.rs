#![feature(auto_traits, negative_impls, min_specialization)]
//Clone model just clones the reference
use core_tahini_utils::{
    policies::MessagePolicy,
    types::{BBoxConversation, Message},
};
use std::sync::Arc;
//Required for model locking across async tasks
use tokio::sync::Mutex;

//Channel transport Code
use alohomora::tarpc::{server::{TahiniBaseChannel, TahiniChannel}, transport::new_tahini_transport};
use futures::{
    Future, StreamExt,
};
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics
use alohomora::bbox::BBox as PCon;
use alohomora::pure::PrivacyPureRegion as PPR;

//Inference import
//Internal LLM functionings
mod model_backend;
mod token_output_stream;
mod utils;
// mod quantized_gemma3;
use crate::model_backend::{TextGeneration, create_pipeline};

//Tarpc + types

use core_tahini_utils::funcs::parse_conversation;
use core_tahini_utils::types::{LLMError, LLMResponse, UserPrompt};
use llm_tahini_utils::service::Inference;

//Database import

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
static SYSTEM_PROMPT: &str = "You are a helpful assistant. You are tasked with responding to user queries, in an accurate, up-to-date, truthful manner. Keep your replies short and polite, with professional language. Never answer beyond the user's queries. Always stay on topic. Reply in a minimal manner." ;//Your knowledge goes up to March, 2024. Today's date is April,9, 2025. At each turn, if you decide to invoke any of the function(s), it should be wrapped with ```tool_code```. The methods described below are imported and available, you can only use defined methods. The generated code should be readable and efficient. The response to a method will be wrapped in ```tool_output``` use it to call more tools or generate a helpful, friendly response. When using a ```tool_call``` think step by step why and how it should be used.
// ```python
// def web_search(search_string: str) -> str:
//    \"\"\"Fetches up-to-date information on the research string given by the argument
//    Args:
//        search_string: The topic you want to search the internet for.
//     Returns:
//        The content of the two most relevant pages for this given search
//     \"\"\"
// ```
// <end_of_turn>";

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

        let parsed_conversation = prompt.conversation; 

        let inf = PPR::new(move |mut unboxed_prompt: Vec<Message>| {
            unboxed_prompt.insert(0, Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            });
            locked_model.run(unboxed_prompt, prompt.nb_token as usize)
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

                let transport = new_tahini_transport(framed, Json::default());
                let fut = TahiniBaseChannel::with_defaults(transport)
                    .execute(server.clone().serve())
                    .for_each(wait_upon);
                tokio::spawn(fut);
            }
        }
        Err(ref x) => println!("Failed at creating the pipeline with error {:?}", x),
    }

    Ok(())
}
