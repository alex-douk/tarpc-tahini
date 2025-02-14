#![feature(auto_traits, negative_impls, min_specialization)]
//Clone model just clones the reference
use std::sync::Arc;
use rpc::database::DatabaseClient;
//Required for model locking across async tasks
use tokio::sync::Mutex;



//Channel transport Code
use tokio_util::codec::LengthDelimitedCodec;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use alohomora::{pure::execute_pure, tarpc::server::{TahiniBaseChannel, TahiniChannel}};
use futures::{Future, StreamExt, future::{self, Ready}};

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

//Sesame basics
use alohomora::bbox::BBox as PCon;
use alohomora::pure::PrivacyPureRegion;
use alohomora::fold::fold;

//Application-wide mods
mod rpc;
mod types;
mod policies;
use crate::policies::PromptPolicy;

//Inference import
    //Internal LLM functionings
mod model_backend;
mod token_output_stream;
use crate::model_backend::{create_pipeline, TextGeneration};
use anyhow::Error as E;

    //Tarpc + types
use crate::types::inference_types::{LLMResponse, UserPrompt};
use crate::rpc::inference::Inference;

//Database import
use crate::types::database_types::{DatabaseForm, DBUUID};

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

async fn store_to_database(user: String, prompt: PCon<String, PromptPolicy>) -> DBUUID {


        let codec_builder = LengthDelimitedCodec::builder();
        let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
        let transport = new_transport(codec_builder.new_framed(stream), Json::default());

        let payload = DatabaseForm {
            user,
            full_prompt: prompt
        };

        let response = DatabaseClient::new(Default::default(), transport)
            .spawn()
            .store_prompt(tarpc::context::current(), payload)
            .await.unwrap();
        response
}

impl Inference for InferenceServer {

    async fn inference(self, _context: tarpc::context::Context, prompt: UserPrompt) -> LLMResponse {
        println!("Got a request");

        let prompt_copy = prompt.prompt.clone();

        let mut locked_model = self.model.lock_owned().await;

        //TODO(douk): Fix so that data-dependent control flow is decided by policy
        //i.e. have a way to say "this is a result: if data is error, don't send"
        //Could prove interesting to maintain this behavior actually, in case of content moderation
        //policies. You might wish to store violations
        let inf = PrivacyPureRegion::new( move |unboxed_prompt: String| { 
            let res = locked_model.run(unboxed_prompt.as_str(), prompt.nb_token as usize);
            match res {
                Ok(r) => r,
                Err(e) => e.to_string()
            }
        });

        // Keeping it here in case i ever need it later
        // let mut writer = Vec::with_capacity(128);
        // let mut ser = serde_json::ser::Serializer::new(&mut writer);
        // let _ =prompt.serialize(&mut ser);
        // println!("Using naive serializer, we get : {:?}", String::from_utf8(writer));
        let boxed_response = prompt.prompt.into_ppr(inf);

        let pair = fold((prompt_copy, boxed_response.clone())).unwrap();
        let pair = pair.specialize_policy::<PromptPolicy>().unwrap();



        // let send_and_ret = PrivacyPureRegion::new(move |pair: (String, Result<String, E>)| {
        //     let prompt = pair.0;
        //     let infered = pair.1;
        //     match infered {
        //         Err(e) => 
        //
        //
        //     }
        //
        //
        //
        // })
        //
        //TODO(douk): Find a way to elegantly merge two PCons into one.
        //Also references the data-dependent control flow
        let uuid = store_to_database(prompt.user, boxed_response.clone()).await;

        let rsp = LLMResponse {
            infered_tokens : boxed_response
            //     .into_ppr(PrivacyPureRegion::new(|unboxed_rsp: Result<String, E>| {
            //     match unboxed_rsp {
            //         Ok(tokens) => tokens,
            //         Err(e) => e.to_string()
            //     }
            // })),
            ,
        db_uuid : uuid
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
