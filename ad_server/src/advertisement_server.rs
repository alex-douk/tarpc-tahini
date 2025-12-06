use fizz_rs::{server_tls, CertificatePublic};
use hoodini_server::fetch_credentials;
use rake::{Rake, StopWords};
use rand::seq::{IndexedRandom, IteratorRandom};
//Clone model just clones the reference

use advertisement_tahini_utils::policies::MarketingReason;
use advertisement_tahini_utils::policies::{MarketingPolicy, THIRD_PARTY_PROCESSORS};
use std::any::Any;
use std::{collections::HashMap, str::FromStr, sync::Arc};
//Required for model locking across async tasks
use tokio::sync::Mutex;

//Channel transport Code
use futures::{
    future::{self, Ready},
    Future, StreamExt,
};
use tahini_tarpc::server::{TahiniBaseChannel, TahiniChannel};
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics
use sesame::context::UnprotectedContext;
use sesame::critical::{CriticalRegion, Signature};
use sesame::fold::fold;
use sesame::pcon::PCon;
use sesame::policy::{AnyPolicyDyn, Policy, Reason};
use sesame::verified::VerifiedRegion as VR;

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
struct AdServer {
    rake: Arc<Rake>,
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
            allowed_vendor
                .choose(&mut rng)
                .ok_or(())
                .map(|x: &&String| (**x).clone())
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
    rake: Arc<Rake>,
) -> PCon<String, MarketingPolicy> {
    match vendor {
        "Google_Ads" => google_ads::get_ad(data, rake),
        "Meta_Ads" => meta_ads::get_ad(data, rake),
        _ => unreachable!(),
    }
}

fn ad_strategy(pol: &MarketingPolicy) -> AdStrategy {
    match pol.targeted_ads_consent {
        false => match find_vendor(&pol.third_party_ad_vendors_allowed) {
            Ok(vendor) => AdStrategy::ThirdPartyAnonymous(vendor),
            Err(_) => AdStrategy::LocalProcessAnonymous,
        },
        true => match find_vendor(&pol.third_party_ad_vendors_allowed) {
            Ok(vendor) => AdStrategy::ThirdPartyTracked(vendor),
            Err(_) => AdStrategy::LocalProcessTracked,
        },
    }
}

use stop_words::{get, LANGUAGE};

pub fn parse_conversation_into_topics(conv: String, rake: Arc<rake::Rake>) -> String {
    let keywords = rake.run(conv.as_str());
    let top_keyword = keywords.first();
    match top_keyword {
        None => "this topic".to_string(),
        Some(kw) => kw.keyword.clone(),
    }
}

fn local_process(data: ThirdPartyProcessorData, rake: Arc<Rake>) -> PCon<String, MarketingPolicy> {
    match data.username {
        None => data.prompt.
            into_verified(VR::new(|conv| {
            format!(
                "Find more about {} on [https://SomeRandomWebSite.com](https://brown.edu)",
                parse_conversation_into_topics(conv, rake)
            )
        })),
        Some(username) => fold::<dyn AnyPolicyDyn, _>((username, data.prompt))
            .unwrap()
            .into_verified(VR::new(|(uname_unboxed, conv_unboxed)| {
                format!(
                    "Hi {}! You can find more about {} on [https://SomeRandomWebSite.com](https://brown.edu)",
                    uname_unboxed,
                    parse_conversation_into_topics(conv_unboxed, rake)
                )
            }))
            .specialize_policy()
            .expect("Couldn't coerce ad policies together during local processing"),
    }
}

impl Advertisement for AdServer {
    async fn auction_bidding(
        self,
        _context: tarpc::context::Context,
        prompt: PCon<MarketingData, MarketingPolicy>,
    ) -> Ad {
        let strategy = ad_strategy(prompt.policy());
        let tpd = ThirdPartyProcessorData {
            username: prompt
                .clone()
                .into_verified(VR::new(|x: MarketingData| x.username))
                .fold_in(),
            prompt: prompt.into_verified(VR::new(|x: MarketingData| x.prompt)),
        };
        let ad = match strategy {
            AdStrategy::ThirdPartyTracked(vendor) | AdStrategy::ThirdPartyAnonymous(vendor) => {
                fetch_ad_from_third_party(vendor.as_str(), tpd, self.rake)
            }
            AdStrategy::LocalProcessAnonymous | AdStrategy::LocalProcessTracked => {
                local_process(tpd, self.rake)
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
    let stop_words =
        StopWords::from_file("stopwords.txt").expect("Couldn't load the stopwords list");
    let rake = Rake::new(stop_words);
    let server = AdServer {
        rake: Arc::new(rake),
    };

    let public_cert = CertificatePublic::load_from_file("sidecar_cert.pem")
        .expect("Couldn't find public sidecar certificate");
    let ctxt = hoodini_server::fetch_credentials().map(|cred| {
        server_tls::ServerTlsContext::new(public_cert, cred)
            .expect("Couldn't create TLS-DC context")
    });
    // let server_tls_context = hoodini_server::fetch_credentials().map(|cred| {
    //
    // })
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 8002)).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        match ctxt {
            None => {
                let framed = codec_builder.new_framed(stream);
                let transport = new_transport(framed, Json::default());
                let fut = TahiniBaseChannel::with_defaults(transport)
                    .execute(server.clone().serve())
                    .for_each(wait_upon);
                tokio::spawn(fut);
            }
            Some(ref tls_ctx) => {
                let tls_stream = tls_ctx
                    .accept_from_stream(stream)
                    .await
                    .expect("Couldn't establish TLS channel with client");
                let framed = codec_builder.new_framed(tls_stream);
                let transport = new_transport(framed, Json::default());
                let fut = TahiniBaseChannel::with_defaults(transport)
                    .execute(server.clone().serve())
                    .for_each(wait_upon);
                tokio::spawn(fut);
            }
        }
    }
}
