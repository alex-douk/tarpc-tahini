use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Instant;

use core_tahini_utils::types::Message;
use reqwest::header::HeaderValue;
use reqwest::ClientBuilder;
use reqwest::cookie::{CookieStore, Jar};
use serde::{Deserialize, Serialize};
use url::Url;

mod llm;
mod db;
mod ads;

#[derive(Serialize)]
pub(crate) struct InferenceRequest {
    pub user: Option<String>,
    pub conv_id: Option<String>,
    pub conversation: Vec<Message>,
    pub nb_token: u32,
}

fn policy_cookies() -> Jar {
    let jar = Jar::default();
    let policy_cookies =  vec!["storage=true", "ads=true", "image_gen=true", "targeted_ads=true", "allowed_third_party_data_vendors=[]"];
    let headers_v : Vec<_> = policy_cookies.iter().map(|c| HeaderValue::from_static(c)).collect();
    let mut headers = headers_v.iter();

    let global_url = Url::parse("http://localhost:8000").expect("Wrong url for webserver");
    jar.set_cookies(&mut headers, &global_url);
    jar

}

#[tokio::main]
async fn main() {
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
    let start = Instant::now();
    let resp = client.post(chat_url.clone()).json(&payload).send().await;
    let elapsed = start.elapsed();
    println!("{:?}", elapsed)
}
