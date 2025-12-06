use rocket::{route, serde::json::Json as JsonGuard};
use tokio_rustls::TlsConnector;
use std::{collections::HashMap, sync::{Arc, OnceLock}, time::Instant};

use advertisement_tahini_utils::{
    service::AdvertisementClient,
    types::{Ad, MarketingData},
    THIRD_PARTY_PROCESSORS,
};
use core_tahini_utils::{
    funcs::marketing_parse_conv,
    types::{Conversation, Message},
};

use crate::SERVER_ADDRESS;
use tarpc::tokio_serde::formats::Json;
use tarpc::{context, serde_transport::new as new_transport};
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub static ADCLIENT: OnceLock<AdvertisementClient> = OnceLock::new();

pub(crate) async fn initialize_ad_client() {
    let mut root_store = rustls::RootCertStore::empty();
        for root in super::load_certs(super::END_CHAIN) {
            root_store.add(root).unwrap();
        }
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let domain = rustls::pki_types::ServerName::try_from("localhost").unwrap();
    let connector = TlsConnector::from(Arc::new(config));
    println!("Creating new AdCorp client");
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 8002)).await.unwrap();
    let stream = connector.connect(domain, stream).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = AdvertisementClient::new(Default::default(), transport).spawn();
    if let Err(_) = ADCLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

#[route(GET, uri = "/get_vendors")]
pub(crate) async fn get_ads_vendors() -> JsonGuard<Vec<String>> {
    JsonGuard(
        Vec::from(THIRD_PARTY_PROCESSORS)
            .iter()
            .map(|x| x.to_string())
            .collect(),
    )
}
// #[tracing::instrument(name="Ads RPC")]
pub(crate) async fn send_to_marketing(
    uname: Option<String>,
    conv: Conversation,
    tpp_vendors: Vec<String>,
    context: tarpc::context::Context,
) -> String {
    // let start = Instant::now();
    //Let's set some bad defaults of targeted ads + all third party processors
    let payload = MarketingData {
        username: uname,
        prompt: marketing_parse_conv(conv),
        third_party_ad_vendors_allowed: tpp_vendors,
    };

    let ad: Ad = match ADCLIENT.get() {
        None => {
            panic!("Ad client connection should already exist");
        }
        Some(client) => client.auction_bidding(context, payload).await.unwrap(),
    };
    // let elapsed = start.elapsed();
    // tracing::warn!(?elapsed, "Time for Ads RPC call");
    ad.ad
}
