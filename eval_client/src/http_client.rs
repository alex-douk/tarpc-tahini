use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use core_tahini_utils::types::{LLMResponse, Message};
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::HeaderValue;
use reqwest::{Client, ClientBuilder};
use serde::Serialize;
use std::fs::OpenOptions;
use tracing_appender::non_blocking::WorkerGuard;
use url::Url;
use uuid::Uuid;

mod ads;
mod db;
mod llm;

const NB_ITER: usize = 5_000;

#[derive(Serialize)]
pub(crate) struct InferenceRequest {
    pub user: Option<String>,
    pub conv_id: Option<String>,
    pub conversation: Vec<Message>,
    pub nb_token: u32,
}

fn policy_cookies() -> Jar {
    let jar = Jar::default();
    let policy_cookies = vec![
        "storage=true",
        "ads=true",
        "image_gen=true",
        "targeted_ads=true",
        "allowed_third_party_data_vendors=[]",
    ];
    let headers_v: Vec<_> = policy_cookies
        .iter()
        .map(|c| HeaderValue::from_static(c))
        .collect();
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
    let client = ClientBuilder::new()
        .cookie_provider(cookie_store)
        .build()
        .expect("Couldn't build client");

    let username = Uuid::new_v4().to_string();
    let signup_url = Url::parse("http://localhost:8000/account/signup").expect("Wrong URL");
    let mut payload = HashMap::new();
    payload.insert("username", username);
    let resp = client
        .post(signup_url.clone())
        .json(&payload)
        .send()
        .await
        .unwrap();
    let body: HashMap<String, String> = resp.json().await.expect("Couldn't read response");
    let uuid = body.get("uuid");
    // test_chat_endpoint(&client).await;
    // fill_read_endpoint(&client, uuid.expect("Couldn't get a user_id on signup")).await;
    test_read_endpoint(&client, uuid.expect("Couldn't signup")).await;

    drop(guard)
}

//The proper way of doing it is to send about 10 LONGASS different messages to a specific
//conversation, and then try and query it for idk how many different times.
//Perhaps another idea is to query each conversation 100 times, and have 50 different
//conversations.

async fn test_chat_endpoint(client: &Client) {
    let chat_url = Url::parse("http://localhost:8000/chat").expect("Wrong URL");
    let conv = crate::llm::gen_conversation(5);
    let payload = InferenceRequest {
        user: None,
        conv_id: None,
        conversation: conv,
        nb_token: 300,
    };

    for _ in 0..NB_ITER {
        let start = Instant::now();
        let _resp = client.post(chat_url.clone()).json(&payload).send().await;
        let elapsed = start.elapsed();
        tracing::info!(?elapsed, "Time for end-to-end call");
    }
}

async fn test_read_endpoint(client: &Client, user_id: &String) {
    let mut conv_ids = Vec::new();
    for _ in 0..100 {
        let conv_id = fill_read_endpoint(client, &user_id).await;
        conv_ids.push(conv_id);
    }

    for conv_id in conv_ids {
        let read_url =
            Url::parse(format!("http://localhost:8000/c/{}", conv_id).as_str()).expect("Wrong URL");
        for _ in 0..200 {
            let start = Instant::now();
            let _resp = client.get(read_url.clone()).send().await;
            let elapsed = start.elapsed();
            tracing::info!(?elapsed, "Time for end-to-end call");
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct InferenceResponse {
    infered_tokens: Message,
    ad: Option<String>,
    db_uuid: Option<String>
}

async fn fill_read_endpoint(client: &Client, user_id: &String) -> String {
    let mut conv = Vec::new();
    conv.push(generate_one_long_message());
    let chat_url = Url::parse("http://localhost:8000/chat").expect("Wrong URL");
    let payload = InferenceRequest {
        user: None,
        conv_id: None,
        conversation: conv.clone(),
        nb_token: 300,
    };
    let resp = client
        .post(chat_url.clone())
        .json(&payload)
        .send()
        .await
        .expect("Couldn't post to the server");
    let body : InferenceResponse = resp.json().await.expect("Couldn't get request's body as text");
    // let body: String = resp.json().await.expect("Couldn't read response");
    // body
    // let body: HashMap<String, String> = resp.json().await.expect("Couldn't read response");
    let db_uuid = body.db_uuid
        .expect("Couldn't get hands on the conversation_id");
    // // cookies.add_cookie_str(format!("db_uuid={}",db_uuid).as_str(), &global_url);
    for _ in 0..(NB_ITER / 100) {
        let payload = InferenceRequest {
            user: Some(user_id.clone()),
            conv_id: Some(db_uuid.clone()),
            conversation: conv.clone(),
            nb_token: 300,
        };
        let resp = client
            .post(chat_url.clone())
            .json(&payload)
            .send()
            .await
            .expect("Couldn't post to the server");
    }
    db_uuid.clone()
}

use tracing_appender::non_blocking;

use crate::llm::generate_one_long_message;

fn init_tracing() -> WorkerGuard {
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
