use rand::seq::{IndexedRandom, IteratorRandom};
//Clone model just clones the reference

use std::collections::HashMap;
//Required for model locking across async tasks
use advertisement_tahini_utils::THIRD_PARTY_PROCESSORS;

//Channel transport Code
use futures::{
    Future, StreamExt,
};
use tarpc::{serde_transport::new as new_transport, server::BaseChannel, tokio_serde::formats::Json};
use tokio_util::codec::LengthDelimitedCodec;
use tarpc::server::Channel;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics

use advertisement_tahini_utils::service::Advertisement;
//Application-wide mods
use advertisement_tahini_utils::types::{Ad, MarketingData};
mod email;
mod google_ads;
mod meta_ads;

static GOOGLE_AD: &str = "Find more about {} on [https://google.com](Google)";
static META_AD: &str =
    "More interesting contents about {} await on [https://facebook.com](Facebook)!";

#[derive(Clone)]
struct AdServer;

enum AdStrategy {
    ThirdPartyTracked(String),
    ThirdPartyAnonymous(String),
    LocalProcessTracked,
    LocalProcessAnonymous,
}

fn find_vendor(consent_map: &Vec<String>) -> Result<String, ()> {
    let mut allowed_vendor = Vec::new();
    let tpp_vec = THIRD_PARTY_PROCESSORS.to_vec();
    for vendor in consent_map {
        if tpp_vec.contains(&vendor.as_str()) {
            allowed_vendor.push(vendor);
        }
    }
    match allowed_vendor.len() {
        0 => Err(()),
        _ => {
            let mut rng = rand::rng();
            allowed_vendor.choose(&mut rng).ok_or(()).map(|x: &&String| (**x).clone())
        }
    }
}

pub(crate) struct ThirdPartyProcessorData {
    pub username: Option<String>,
    pub prompt: String
}

fn fetch_ad_from_third_party(
    vendor: &str,
    data: ThirdPartyProcessorData,
) -> String {
    println!("Vendor is {}", vendor);
    match vendor {
        "Google_Ads" => google_ads::get_ad(data),
        "Meta_Ads" => meta_ads::get_ad(data),
        _ => unreachable!(),
    }
}

fn ad_strategy(targeted: bool, allowed_vendors: Vec<String>) -> AdStrategy {
    match targeted {
        false => match find_vendor(&allowed_vendors) {
            Ok(vendor) => AdStrategy::ThirdPartyAnonymous(vendor),
            Err(_) => AdStrategy::LocalProcessAnonymous,
        },
        true => {
            println!("We have targed consent");
            match find_vendor(&allowed_vendors) {
                Ok(vendor) => AdStrategy::ThirdPartyTracked(vendor),
                Err(_) => AdStrategy::LocalProcessTracked,
            }
        }
    }
}

use stop_words::{LANGUAGE, get};

pub fn parse_conversation_into_topics(conv: String) -> String {
    let mut stop_words = get(LANGUAGE::English);
    stop_words.push("user".to_string());
    stop_words.push("model".to_string());
    stop_words.push("It's".to_string());
    stop_words.push("it's".to_string());
    // let text_rank = TextRank::new(TextRankParams::WithDefaults(&conv, &stop_words));
    // let ranked_keywords = text_rank.get_ranked_words(10);
    // let mut kw_iters = ranked_keywords.iter().skip(2);
    // while let Some(t) = kw_iters.next() {
    //     if !stop_words.contains(t) {
    //         return t.to_string();
    //     }
    // }
    "this topic".to_string()
    // return ranked_keywords[0].clone();
}

fn local_process(data: ThirdPartyProcessorData) -> String {
    match data.username {
        None => 
            format!(
                "Find more about {} on [https://SomeRandomWebSite.com](https://brown.edu)",
                parse_conversation_into_topics(data.prompt)
            )
        ,
        Some(username) => format!("Hi {}! You can find more about {} on [https://SomeRandomWebSite.com](https://brown.edu)",
                    username,
                    parse_conversation_into_topics(data.prompt)
                )
    }
}

impl Advertisement for AdServer {
    async fn auction_bidding(
        self,
        _context: tarpc::context::Context,
        prompt: MarketingData
    ) -> Ad {
        let strategy = ad_strategy(prompt.username.is_some(), prompt.third_party_ad_vendors_allowed);
        let tpd = ThirdPartyProcessorData {
            username: prompt.username,
            prompt: prompt.prompt
        };
        let ad = match strategy {
            AdStrategy::ThirdPartyTracked(vendor) | AdStrategy::ThirdPartyAnonymous(vendor) => {
                fetch_ad_from_third_party(vendor.as_str(), tpd)
            }
            AdStrategy::LocalProcessAnonymous | AdStrategy::LocalProcessTracked => {
                local_process(tpd)
            }
        };

        Ad { ad }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the <ORG2> advertisement server!");
    //A hashmap that for a given username, yields a hashmap of all UUIDS : chats for that specific
    //
    //
    let stop_words = get(LANGUAGE::English);
    let server = AdServer;
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 8002)).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        println!("Accepted a connection");
        let framed = codec_builder.new_framed(stream);
        
        let transport = new_transport(framed, Json::default());

        // let transport = new_transport(framed, Bincode::default());
        let fut = BaseChannel::with_defaults(transport)
            // .execute(server.serve());
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
