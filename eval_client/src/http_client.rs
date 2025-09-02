use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;

use core_tahini_utils::types::Message;
use reqwest::header::HeaderValue;
use reqwest::ClientBuilder;
use reqwest::cookie::{CookieStore, Jar};
use serde::{Deserialize, Serialize};
use tracing_appender::non_blocking::WorkerGuard;
use std::fs::OpenOptions;
use url::Url;
use uuid::Uuid;

mod llm;
mod db;
mod ads;

const NB_ITER: usize = 200_000;

#[derive(Serialize)]
pub(crate) struct InferenceRequest {
    pub user: Option<String>,
    pub conv_id: Option<String>,
    pub conversation: Vec<Message>,
    pub nb_token: u32,
}

fn policy_cookies() -> Jar {
    let jar = Jar::default();
    let policy_cookies =  vec!["storage_consent=true", "ad_consent=true", "image_gen=true", "targeted_ads_consent=true", "allowed_third_party_data_vendors=[]"];
    let headers_v : Vec<_> = policy_cookies.iter().map(|c| HeaderValue::from_static(c)).collect();
    let mut headers = headers_v.iter();

    let global_url = Url::parse("http://localhost:8000").expect("Wrong url for webserver");
    jar.set_cookies(&mut headers, &global_url);
    jar

}

#[tokio::main]
async fn main() {
    let guard = init_tracing();
    let cookies = policy_cookies();
    let cookie_store = Arc::new(cookies);
    let client = ClientBuilder::new().
        cookie_provider(cookie_store).build().expect("Couldn't build client");
    let chat_url = Url::parse("http://localhost:8000/chat").expect("Wrong URL");
    let conv = crate::llm::gen_conversation(5);
    let payload = InferenceRequest {
        user: None,
        conv_id: None,
        conversation: conv,
        nb_token: 300
    };

    let username = Uuid::new_v4().to_string();
    let chat_url = Url::parse("http://localhost:8000/account/signup").expect("Wrong URL");
    let mut payload = HashMap::new();
    payload.insert("username", username);
    let resp = client.post(chat_url.clone()).json(&payload).send().await.unwrap();
    let body: HashMap<String, String> = resp.json().await.expect("Couldn't read response");
    let uuid = body.get("uuid");

    for _ in 0..NB_ITER {
        let start = Instant::now();
        let resp = client.post(chat_url.clone()).json(&payload).send().await;
        let elapsed = start.elapsed();
        tracing::info!(?elapsed, "Time for DB_Store RPC call");

    }
    drop(guard)
}

use tracing_appender::non_blocking;

fn init_tracing() -> WorkerGuard{
    let log_path = "benchmark.log";
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(log_path)
      .unwrap();
    let (non_blocking, guard) = non_blocking(file);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_target(false)
        .with_ansi(false)
        .init();
    guard
}
