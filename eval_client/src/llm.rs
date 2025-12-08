use std::{
    net::{IpAddr, Ipv4Addr},
    time::Instant,
};

use core_tahini_utils::{
    policies::MessagePolicy,
    types::{Message, UserPrompt},
};
use futures::executor::block_on;
use llm_tahini_utils::service::TahiniInferenceClient;
use sesame::pcon::PCon;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
pub fn gen_conversation(rounds: usize) -> Vec<Message> {
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
    for _ in 0..rounds - 1 {
        conv.push(user_message.clone());
        conv.push(model_message.clone());
    }
    conv.push(user_message);
    conv
}

pub fn generate_one_long_message() -> Message {
    Message { role: "user".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin ut lacus egestas, ultricies elit a, pretium sem. Aliquam aliquet et eros sit amet finibus. Duis id hendrerit ligula. Maecenas ullamcorper, nisl ac volutpat viverra, dui turpis laoreet purus, vitae consequat sem nisl nec nisi. Maecenas bibendum turpis at nisl tempor egestas. Sed eu enim id sem condimentum maximus sed ac diam. Nulla quis mi vitae ante pretium pulvinar. Aliquam dapibus elit ut interdum dignissim. Etiam orci turpis, venenatis ut odio eget, viverra luctus mauris.

Morbi ultrices tempus massa. Praesent in enim neque. Suspendisse suscipit est ac purus feugiat, sit amet egestas est eleifend. Proin eget neque eget neque sagittis ultrices lacinia vitae erat. Donec justo turpis, euismod aliquet eros vel, consectetur fermentum velit. Proin blandit elit ut diam bibendum, ac efficitur enim mollis. Phasellus posuere nulla a fringilla tempus. Duis ut dui odio. Praesent ut justo quis diam dignissim ullamcorper ut mollis dui. Maecenas pulvinar, ligula quis eleifend commodo, mi felis vehicula purus, non accumsan arcu mi in odio. Cras dictum ultricies sem a gravida. Pellentesque lobortis nulla non ipsum auctor suscipit. In consectetur tellus at arcu eleifend, vitae ultrices enim ultricies. Phasellus id hendrerit nulla. Maecenas ultrices, ante at feugiat porttitor, mauris justo congue mauris, sed sollicitudin arcu nisl id tellus. ".to_string() }
}

async fn initialize_llm_client() -> TahiniInferenceClient {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    //Custom deadline for inference calls. Will also potentially allow for streaming
    //responses (but GitHub issues suggest tarpc is unable to do so)
    let client = TahiniInferenceClient::new(Default::default(), transport).spawn();
    client
}

async fn contact_llm_server(client: &TahiniInferenceClient, prompt: UserPrompt, iter: usize) {
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
        conversation: PCon::new(gen_conversation(rounds), policy),
        nb_token: 250,
    };
    let client = block_on(async { initialize_llm_client().await });
    for i in 0..nb_iter {
        let _ = contact_llm_server(&client, prompt.clone(), i).await;
    }
    println!("LLM Benchmark done");
}
