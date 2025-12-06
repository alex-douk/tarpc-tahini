use rake::{Rake, StopWords};
use rand::seq::{IndexedRandom, IteratorRandom};
use tokio_rustls::TlsAcceptor;
//Clone model just clones the reference

use std::collections::HashMap;
use std::sync::Arc;
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

use crate::certificate::{load_certs, load_private_key, END_CERT, END_PRIVATEKEY};
mod email;
mod google_ads;
mod meta_ads;
mod certificate;

static GOOGLE_AD: &str = "Find more about {} on [https://google.com](Google)";
static META_AD: &str =
    "More interesting contents about {} await on [https://facebook.com](Facebook)!";

#[derive(Clone)]
struct AdServer{
    stop_words: Arc<rake::Rake>
}

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
    rake: Arc<rake::Rake>
) -> String {
    match vendor {
        "Google_Ads" => google_ads::get_ad(data, rake),
        "Meta_Ads" => meta_ads::get_ad(data, rake),
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
            match find_vendor(&allowed_vendors) {
                Ok(vendor) => AdStrategy::ThirdPartyTracked(vendor),
                Err(_) => AdStrategy::LocalProcessTracked,
            }
        }
    }
}

pub fn parse_conversation_into_topics(conv: String, rake: Arc<rake::Rake>) -> String {

    let keywords = rake.run(conv.as_str());
    let top_keyword = keywords.first();
    match top_keyword {
        None => "this topic".to_string(),
        Some(kw) => kw.keyword.clone()
    }
}

fn local_process(data: ThirdPartyProcessorData, rake: Arc<rake::Rake>) -> String {
    match data.username {
        None => 
            format!(
                "Find more about {} on [https://SomeRandomWebSite.com](https://brown.edu)",
                parse_conversation_into_topics(data.prompt, rake)
            )
        ,
        Some(username) => format!("Hi {}! You can find more about {} on [https://SomeRandomWebSite.com](https://brown.edu)",
                    username,
                    parse_conversation_into_topics(data.prompt, rake)
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
                fetch_ad_from_third_party(vendor.as_str(), tpd, self.stop_words)
            }
            AdStrategy::LocalProcessAnonymous | AdStrategy::LocalProcessTracked => {
                local_process(tpd, self.stop_words)
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
    // let stop_words = get(LANGUAGE::English);
    //
    let stop_words = StopWords::from_file("stopwords.txt").expect("Couldn't load the stopwords list");
    let rake = Rake::new(stop_words);
    let server = AdServer{
        stop_words: Arc::new(
                        rake
                    )
    };

    let cert = load_certs(END_CERT);
    let key = load_private_key(END_PRIVATEKEY);
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 8002)).await.unwrap();
    let config = rustls::ServerConfig::builder().with_no_client_auth().with_single_cert(cert, key).unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        let tls_stream = acceptor.accept(stream).await.unwrap();
        println!("Accepted a connection");
        let framed = codec_builder.new_framed(tls_stream);
        let transport = new_transport(framed, Json::default());

        // let transport = new_transport(framed, Bincode::default());
        let fut = BaseChannel::with_defaults(transport)
            // .execute(server.serve());
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
