#![feature(auto_traits, negative_impls, min_specialization)]
//Clone model just clones the reference
use core_tahini_utils::types::{Conversation, Message};
use tokio_rustls::TlsAcceptor;
use std::sync::Arc;
use std::thread;
//Required for model locking across async tasks
use tokio::sync::Mutex;

//Channel transport Code
use futures::{Future, StreamExt};
use tarpc::tokio_serde::formats::Json;
use tarpc::{
    serde_transport::new as new_transport,
    server::{BaseChannel, Channel},
};
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics

//Inference import
//Internal LLM functionings
mod model_backend;
mod token_output_stream;
mod utils;
mod certificate;
use crate::certificate::{load_certs, load_private_key, END_CERT, END_PRIVATEKEY};
// mod quantized_gemma3;
use crate::model_backend::{create_pipeline, TextGeneration};

//Tarpc + types

use core_tahini_utils::types::{LLMError, LLMResponse, UserPrompt};
use llm_tahini_utils::service::Inference;

//Database import

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
static SYSTEM_PROMPT: &str = "You are a helpful assistant. You are tasked with responding to user queries, in an accurate, up-to-date, truthful manner. Keep your replies short and polite, with professional language. Never answer beyond the user's queries. Always stay on topic. Reply in a minimal manner." ; //Your knowledge goes up to March, 2024. Today's date is April,9, 2025. At each turn, if you decide to invoke any of the function(s), it should be wrapped with ```tool_code```. The methods described below are imported and available, you can only use defined methods. The generated code should be readable and efficient. The response to a method will be wrapped in ```tool_output``` use it to call more tools or generate a helpful, friendly response. When using a ```tool_call``` think step by step why and how it should be used.
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
    // model: Arc<Mutex<model_backend::TextGeneration>>,
}

impl InferenceServer {
    // pub fn new(tg: TextGeneration) -> Self {
    //     InferenceServer {
    //         model: Arc::new(Mutex::new(tg)),
    //     }
    // }
}

impl Inference for InferenceServer {
    async fn inference(self, _context: tarpc::context::Context, prompt: UserPrompt) -> LLMResponse {
        LLMResponse {
            infered_tokens: Ok(Message { role: "model".to_string(), content: "This is a sample string".to_string() })
        }
        // let mut conv = prompt.conversation;
        // conv.insert(
        //     0,
        //     Message {
        //         role: "system".to_string(),
        //         content: SYSTEM_PROMPT.to_string(),
        //     },
        // );
        // let mut locked_model = self.model.lock_owned().await;
        // let infered = locked_model.run(conv, prompt.nb_token as usize);
        //
        // match infered {
        //     Err(e) => {
        //         eprintln!("Got error {}", e);
        //         LLMResponse {
        //             infered_tokens: Err(LLMError::InternalError),
        //         }
        //     }
        //     Ok(tokens) => LLMResponse {
        //         infered_tokens: Ok(Message {
        //             role: "model".to_string(),
        //             content: tokens,
        //         }),
        //     },
        // }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
    println!("Welcome to the LLM inference server!");
    // let pipeline = create_pipeline();
    // let pipeline: Result<String, String> = Ok("".to_string());
    // match pipeline {
    //     Ok(model) => {
            println!("Successfully created the pipeline!");
            let cert = load_certs(END_CERT);
            let key = load_private_key(END_PRIVATEKEY);
            let listener = TcpListener::bind(&(SERVER_ADDRESS, 5000)).await.unwrap();
            let config = rustls::ServerConfig::builder().with_no_client_auth().with_single_cert(cert, key).unwrap();
            let acceptor = TlsAcceptor::from(Arc::new(config));
            println!("Created server TLS context");
            let codec_builder = LengthDelimitedCodec::builder();
            let server = InferenceServer {
                // model: Arc::new(Mutex::new(model)),
            };
            loop {
                let (stream, _peer_addr) = listener.accept().await.unwrap();
                let tls_stream = acceptor.accept(stream).await.unwrap();
                println!("Accepted a connection");
                let framed = codec_builder.new_framed(tls_stream);

                let transport = new_transport(framed, Json::default());
                let fut = BaseChannel::with_defaults(transport)
                    .execute(server.clone().serve())
                    .for_each(wait_upon);
                tokio::spawn(fut);
            }
        // }
        // Err(ref x) => println!("Failed at creating the pipeline with error {:?}", x),
    // }

    Ok(())
}
