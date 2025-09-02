use std::{net::{IpAddr, Ipv4Addr}, time::Instant};

use alohomora::bbox::BBox;
use core_tahini_utils::{
    policies::MessagePolicy,
    types::{Message, UserPrompt},
};
use futures::executor::block_on;
use llm_tahini_utils::service::TahiniInferenceClient;
use tahini_tarpc::transport::new_tahini_client_transport as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
fn gen_conversation(rounds: usize) -> Vec<Message> {
    let user_message = Message {
        role: "user".to_string(),
        content: "Hello! This is a sample message from a user".to_string(),
    };
    let model_message = Message {
        role: "model".to_string(),
        content: "Hi! My name is Gemma, an AI assistant here to help you. I'm usually available but I can't right now.".to_string(),
    };

    if rounds <= 1 {
        return vec![user_message];
    }

    let mut conv = Vec::with_capacity(rounds);
    for _ in 0..rounds-1 {
        conv.push(user_message.clone());
        conv.push(model_message.clone());
    }
    conv.push(user_message);
    conv
}

async fn initialize_llm_client() -> TahiniInferenceClient {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    //Custom deadline for inference calls. Will also potentially allow for streaming
    //responses (but GitHub issues suggest tarpc is unable to do so)
    let client = TahiniInferenceClient::new(Default::default(), transport)
        .spawn()
        .await;
    client
}

async fn contact_llm_server(
    client: &TahiniInferenceClient,
    prompt: UserPrompt,
    iter: usize
) {
    let context = tarpc::context::current();
    let start = Instant::now();
    let _res = client.inference(context, prompt).await;
    let elapsed = start.elapsed();
    tracing::info!(?elapsed, "Time for LLM RPC call{}", iter);
}

pub async fn benchmark_llm(nb_iter: usize, rounds: usize) {
    let policy = MessagePolicy {
        storage: true,
        marketing_consent: true,
        third_party_ad_vendors_allowed: Vec::new(),
        unprotected_image_gen: true,
        reinforcement_learning_consent: true,
    };
    let prompt = UserPrompt {
        conversation: BBox::new(gen_conversation(rounds), policy),
        nb_token: 250,
    };
    let client = block_on(async { initialize_llm_client().await });
    for i in 0..nb_iter {
        let _ = contact_llm_server(&client, prompt.clone(), i).await;
    }
    println!("LLM Benchmark done");
}
