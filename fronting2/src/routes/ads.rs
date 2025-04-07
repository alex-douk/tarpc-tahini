use alohomora::{
    bbox::BBox,
    context::Context,
    fold::fold,
    policy::PolicyAnd,
    pure::PrivacyPureRegion,
    rocket::{JsonResponse, route},
};

use advertisement_tahini_utils::{
    THIRD_PARTY_PROCESSORS, policies::MarketingPolicy, service::TahiniAdvertisementClient,
    types::MarketingData,
};
use core_tahini_utils::{
    funcs::marketing_parse_conv,
    policies::{MessagePolicy, UsernamePolicy},
    types::{BBoxConversation, Message},
};

use crate::{
    SERVER_ADDRESS,
    adapters::{PolicyAdapter, ad_adapter::AdAdapter},
    policies::ad_policy::AdPolicy,
};
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

pub(crate) async fn send_to_marketing(
    uname: BBox<String, UsernamePolicy>,
    conv: BBoxConversation,
) -> BBox<String, AdPolicy> {
    let payload = fold((uname.clone(), conv.clone()))
        .unwrap()
        .specialize_policy::<PolicyAnd<UsernamePolicy, MessagePolicy>>()
        .expect("For ad transfer, wrong policy coercion");
    let payload: BBox<MarketingData, PolicyAdapter<_>> = payload
        .into_ppr(PrivacyPureRegion::new(
            |(username, conv): (String, Vec<Message>)| MarketingData {
                username: match uname.policy().targeted_ads_consent {
                    false => None,
                    true => Some(username),
                },
                prompt: marketing_parse_conv(conv),
            },
        ))
        .into_bbox();

    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 8002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let ad: AdAdapter = TahiniAdvertisementClient::new(Default::default(), transport)
        .spawn()
        .auction_bidding(context::current(), payload)
        .await
        .unwrap();
    ad.0
}
