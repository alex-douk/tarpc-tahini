use std::path::Path;
use std::{sync::OnceLock, time::Instant};

use fizz_rs::CertificatePublic;
use hoodini_client::DynamicAttestationVerifier;
use sesame::{
    context::Context, fold::fold, pcon::PCon, policy::PolicyAnd, verified::VerifiedRegion,
};

use sesame::policy::AnyPolicyDyn;
use sesame_rocket::rocket::{route, JsonResponse};
use tahini_tarpc::traits::Fromable;
use tarpc::serde_transport::new as new_transport;

use advertisement_tahini_utils::{
    service::TahiniAdvertisementClient,
    types::{Ad, MarketingData},
    THIRD_PARTY_PROCESSORS,
};
use core_tahini_utils::{
    funcs::marketing_parse_conv,
    policies::{MessagePolicy, UsernamePolicy},
    types::{Message, PConConversation},
};

use crate::{
    adapters::{ad_adapter::AdAdapter, PolicyAdapter},
    policies::ad_policy::AdPolicy,
    SERVER_ADDRESS,
};
use tarpc::context;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub static ADCLIENT: OnceLock<TahiniAdvertisementClient> = OnceLock::new();

pub(crate) async fn initialize_ad_client() {
    println!("Creating new AdCorp client");
    let codec_builder = LengthDelimitedCodec::builder();
    let tahini_verifier = DynamicAttestationVerifier::from_config(&Path::new("client_attestation_config.toml"))
        .expect("Couldn't load Tahini client config");
    let pub_cred_opt = tahini_verifier
        .verify_binary(hoodini_client::ServiceName("Advertisement".to_string()))
        .await.ok();
    match pub_cred_opt {
        None => {

            let stream = TcpStream::connect((SERVER_ADDRESS, 8002)).await.unwrap();
            stream.set_nodelay(true).expect("Couldn't set NODELAY");
            println!("Sidecar is not running or attestation failed, we try and connect w/ TCP only");
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniAdvertisementClient::new(Default::default(), transport).spawn();
            if let Err(_) = ADCLIENT.set(client) {
                panic!("Client connection already exists");
            }
        },
        Some(pub_creds) => {
            let stream = TcpStream::connect((SERVER_ADDRESS, 8002)).await.unwrap();
            stream.set_nodelay(true).expect("Couldn't set NODELAY");
            let client_tls_ctx = fizz_rs::client_tls::ClientTlsContext::new(pub_creds, "sidecar_cert.pem").expect("Couldn't create client TLS context");
            let tls_stream = client_tls_ctx.connect(stream, "localhost").await.expect("Couldn't create TLS channel");
            let transport = new_transport(codec_builder.new_framed(tls_stream), Json::default());
            let client = TahiniAdvertisementClient::new(Default::default(), transport).spawn();
            if let Err(_) = ADCLIENT.set(client) {
                panic!("Client connection already exists");
            }

        }
    }
}

#[route(GET, "/get_vendors")]
pub(crate) async fn get_ads_vendors() -> JsonResponse<Vec<String>, ()> {
    JsonResponse(
        Vec::from(THIRD_PARTY_PROCESSORS)
            .iter()
            .map(|x| x.to_string())
            .collect(),
        Context::empty(),
    )
}

// #[tracing::instrument(name="Ads RPC")]
pub(crate) async fn send_to_marketing(
    uname: PCon<String, UsernamePolicy>,
    conv: PConConversation,
    context: tarpc::context::Context,
) -> PCon<String, AdPolicy> {
    // let start = Instant::now();
    let payload = fold::<dyn AnyPolicyDyn, _>((uname.clone(), conv.clone()))
        .unwrap()
        .specialize_policy::<PolicyAnd<UsernamePolicy, MessagePolicy>>()
        .expect("For ad transfer, wrong policy coercion");
    let payload: PCon<MarketingData, PolicyAdapter<_>> = payload
        .into_verified(VerifiedRegion::new(
            |(username, conv): (String, Vec<Message>)| MarketingData {
                username: match uname.policy().targeted_ads_consent {
                    false => None,
                    true => Some(username),
                },
                prompt: marketing_parse_conv(conv),
            },
        ))
        .into_pcon();

    let ad: Fromable<Ad> = match ADCLIENT.get() {
        None => {
            panic!("Ad client connection should already exist");
        }
        Some(client) => client.auction_bidding(context, payload).await.unwrap(),
    };

    let res = ad
        .transform_into::<AdAdapter>()
        .expect("Couldn't transform the data because of context")
        .0;
    // let elapsed = start.elapsed();
    // tracing::warn!(?elapsed, "Time for Ads RPC call");
    res
}
