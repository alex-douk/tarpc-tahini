use alohomora::{
    bbox::BBox,
    context::Context,
    fold::fold,
    pcr::PrivacyCriticalRegion,
    rocket::{
        BBoxCookie, BBoxCookieJar, BBoxJson, BBoxRedirect, JsonResponse, RequestBBoxJson,
        ResponseBBoxJson, route,
    },
};
use services_utils::policies::marketing_policy::THIRD_PARTY_PROCESSORS;
use services_utils::policies::shared_policies::UsernamePolicy;
use services_utils::{
    funcs::parse_conversation, policies::marketing_policy::MarketingPolicy,
    types::inference_types::Message,
};
use services_utils::{
    policies::inference_policy::PromptPolicy,
    rpc::marketing::TahiniAdvertisementClient,
    types::{
        inference_types::BBoxConversation,
        marketing_types::{Ad, MarketingData},
    },
};

use crate::SERVER_ADDRESS;
use tarpc::tokio_serde::formats::Json;
use tarpc::{context, serde_transport::new as new_transport};
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

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

fn generate_marketing_policy(p1: UsernamePolicy, p2: PromptPolicy) -> MarketingPolicy {
    MarketingPolicy {
        no_storage: p2.storage,
        targeted_ads_consent: p1.targeted_ads_consent,
        third_party_processing: p2.third_party_consent,
    }
}

pub(crate) async fn send_to_marketing(
    uname: BBox<String, UsernamePolicy>,
    conv: BBoxConversation,
) -> BBox<String, PromptPolicy> {
    let pol = generate_marketing_policy(uname.policy().clone(), conv.policy().clone());
    let payload = fold((uname, conv.clone())).unwrap();
    let payload: BBox<MarketingData, MarketingPolicy> = payload.into_pcr(
        PrivacyCriticalRegion::new(
            |data: (String, Vec<Message>), _p, _c| {
                BBox::new(
                    MarketingData {
                        username: data.0,
                        prompt: parse_conversation(data.1).unwrap(),
                    },
                    pol,
                )
            },
            alohomora::pcr::Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
            alohomora::pcr::Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
            alohomora::pcr::Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
        ),
        (),
    );

    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 8002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let ad = TahiniAdvertisementClient::new(Default::default(), transport)
        .spawn()
        .auction_bidding(context::current(), payload)
        .await
        .unwrap()
        .ad;

    ad.into_pcr(
        PrivacyCriticalRegion::new(
            |ad_unboxed, _p, _c| {
                BBox::new(ad_unboxed, PromptPolicy {
                    storage: conv.policy().storage,
                    marketing_consent: true,
                    third_party_consent: conv.policy().third_party_consent.clone(),
                    unprotected_image_gen: conv.policy().unprotected_image_gen,
                })
            },
            alohomora::pcr::Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
            alohomora::pcr::Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
            alohomora::pcr::Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
        ),
        (),
    )
}
