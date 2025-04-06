use rand::seq::{IndexedRandom, IteratorRandom};
//Clone model just clones the reference
use services_utils::policies::marketing_policy::MarketingReason;
use services_utils::policies::{MarketingPolicy, marketing_policy::THIRD_PARTY_PROCESSORS};
use std::{collections::HashMap, str::FromStr, sync::Arc};
//Required for model locking across async tasks
use tokio::sync::Mutex;

//Channel transport Code
use alohomora::tarpc::server::{TahiniBaseChannel, TahiniChannel};
use futures::{
    Future, StreamExt,
    future::{self, Ready},
};
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics
use alohomora::bbox::BBox as PCon;
use alohomora::context::UnprotectedContext;
use alohomora::fold::fold;
use alohomora::pcr::{PrivacyCriticalRegion, Signature};
use alohomora::policy::Policy;
use alohomora::policy::Reason;
use alohomora::pure::PrivacyPureRegion as PPR;

//Application-wide mods
use services_utils::rpc::marketing::Advertisement;
use services_utils::types::marketing_types::{Ad, MarketingData};
mod email;
mod google_ads;
mod meta_ads;

static GOOGLE_AD: &str = "Find more about {} on [https://google.com](Google)";
static META_AD: &str =
    "More interesting contents about {} await on [https://facebook.com](Facebook)!";

#[derive(Clone)]
struct AdServer;

enum AdStrategy {
    ThirdPartyTracked(&'static str),
    ThirdPartyAnonymous(&'static str),
    LocalProcessTracked,
    LocalProcessAnonymous,
}

fn find_vendor(consent_map: &HashMap<String, bool>) -> Result<&'static str, ()> {
    let mut allowed_vendor = Vec::new();
    for vendor in THIRD_PARTY_PROCESSORS {
        if *consent_map.get(&vendor.to_string()).unwrap_or(&false) {
            allowed_vendor.push(vendor);
        }
    }
    match allowed_vendor.len() {
        0 => Err(()),
        _ => {
            let mut rng = rand::rng();
            allowed_vendor.iter().choose(&mut rng).ok_or(()).copied()
        }
    }
}

pub(crate) struct ThirdPartyProcessorData {
    pub username: Option<PCon<String, MarketingPolicy>>,
    pub prompt: PCon<String, MarketingPolicy>,
}

fn fetch_ad_from_third_party(
    vendor: &str,
    data: ThirdPartyProcessorData,
) -> PCon<String, MarketingPolicy> {
    println!("Vendor is {}", vendor);
    match vendor {
        "Google_Ads" => google_ads::get_ad(data),
        "Meta_Ads" => meta_ads::get_ad(data),
        _ => unreachable!(),
    }
}

fn ad_strategy(pol: &MarketingPolicy) -> AdStrategy {
    match pol.targeted_ads_consent {
        false => match find_vendor(&pol.third_party_processing) {
            Ok(vendor) => AdStrategy::ThirdPartyAnonymous(vendor),
            Err(_) => AdStrategy::LocalProcessAnonymous,
        },
        true => {
            println!("We have targed consent");
            match find_vendor(&pol.third_party_processing) {
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

fn local_process(data: ThirdPartyProcessorData) -> PCon<String, MarketingPolicy> {
    match data.username {
        None => data.prompt.into_ppr(PPR::new(|conv| {
            format!(
                "Find more about {} on [https://SomeRandomWebSite.com](https://brown.edu)",
                parse_conversation_into_topics(conv)
            )
        })),
        Some(username) => fold((username, data.prompt))
            .unwrap()
            .into_ppr(PPR::new(|(uname_unboxed, conv_unboxed)| {
                format!(
                    "Hi {}! You can find more about {} on [https://SomeRandomWebSite.com](https://brown.edu)",
                    uname_unboxed,
                    parse_conversation_into_topics(conv_unboxed)
                )
            }))
            .specialize_policy()
            .expect("Couldn't coerce ad policies together during local processing"),
    }
}

impl Advertisement for AdServer {
    async fn auction_bidding(
        self,
        context: tarpc::context::Context,
        prompt: PCon<MarketingData, MarketingPolicy>,
    ) -> services_utils::types::marketing_types::Ad {
        let strategy = ad_strategy(prompt.policy());
        let tpd = ThirdPartyProcessorData {
            username: prompt
                .clone()
                .into_ppr(PPR::new(|x: MarketingData| x.username))
                .transpose(),
            prompt: prompt.into_ppr(PPR::new(|x: MarketingData| x.prompt)),
        };
        let ad = match strategy {
            AdStrategy::ThirdPartyTracked(vendor) | AdStrategy::ThirdPartyAnonymous(vendor) => {
                fetch_ad_from_third_party(vendor, tpd)
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
        let fut = TahiniBaseChannel::with_defaults(transport)
            // .execute(server.serve());
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
