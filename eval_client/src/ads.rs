use std::{net::{IpAddr, Ipv4Addr}, time::Instant};
use advertisement_tahini_utils::{policies::MarketingPolicy, service::TahiniAdvertisementClient, types::MarketingData};
use alohomora::bbox::BBox;
use core_tahini_utils::{
    funcs::marketing_parse_conv, policies::MessagePolicy, types::{Message}
};
use futures::executor::block_on;

use tahini_tarpc::{transport::new_tahini_client_transport as new_transport};
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
        return vec![user_message, model_message];
    }

    let mut conv = Vec::with_capacity(rounds);
    for _ in 0..rounds {
        conv.push(user_message.clone());
        conv.push(model_message.clone());
    }
    conv
}

async fn initialize_ad_client() -> TahiniAdvertisementClient {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 8002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let client = TahiniAdvertisementClient::new(Default::default(), transport)
        .spawn()
        .await;
    client
}

async fn contact_ad_server(
    client: &TahiniAdvertisementClient,
    prompt: BBox<MarketingData, MarketingPolicy>,
    iter: usize
) -> Result<(), String> {
    let context = tarpc::context::current();
    let start = Instant::now();
    let res = client.auction_bidding(context, prompt).await;
    let elapsed = start.elapsed();
    tracing::info!(?elapsed, "Time for Ads RPC call {}", iter);
    res.map(|_| ()).map_err(|_| "Call failed".to_string())
}

pub async fn benchmark_ads(nb_iter: usize, rounds: usize) {
    let _policy = MessagePolicy {
        storage: true,
        marketing_consent: true,
        third_party_ad_vendors_allowed: Vec::new(),
        unprotected_image_gen: true,
        reinforcement_learning_consent: true,
    };
    let prompt = gen_conversation(rounds);
    let data = MarketingData 
    {
        username: None,
        prompt : marketing_parse_conv(prompt)
    };
    let pol = MarketingPolicy {
        no_storage: true,
        targeted_ads_consent: false,
        third_party_ad_vendors_allowed: Vec::new()
    };

    let payload = BBox::new(data, pol);

    let client = block_on(async { initialize_ad_client().await });
    for i in 0..nb_iter {
        let _ = contact_ad_server(&client, payload.clone(), i).await;
    }
    println!("Ads Benchmark done");
}
