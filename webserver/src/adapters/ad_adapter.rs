use advertisement_tahini_utils::{policies::MarketingPolicy, types::Ad as RemoteAd};
use alohomora::{
    bbox::BBox,
    policy::{PolicyAnd, PolicyFrom, PolicyInto},
    tarpc::{TahiniTransformFrom, TahiniTransformInto, context::TahiniContext as ProtectedTahiniContext},
};
use core_tahini_utils::policies::{MessagePolicy, UsernamePolicy};

use crate::policies::ad_policy::AdPolicy;
use super::PolicyAdapter;

pub struct AdAdapter(pub BBox<String, AdPolicy>);

impl TahiniTransformFrom<RemoteAd> for AdAdapter {
    fn transform_from(
        other: RemoteAd,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(Self(other.ad.transform_into(context)?))
    }
}

impl PolicyFrom<MarketingPolicy> for AdPolicy {
    fn from_policy(
        _other_policy: MarketingPolicy,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        match context.service.as_str() {
            "Advertisement" => match context.rpc.as_str() {
                "auction_bidding" => Ok(Self),
                _ => Err("Data from unauthorized Advertisement RPC call".to_string()),
            },
            _ => Err("Data from unauthorized service".to_string()),
        }
    }
}

impl PolicyInto<MarketingPolicy> for 
    PolicyAdapter<PolicyAnd<UsernamePolicy, MessagePolicy>> 
{
    fn into_policy(
        self, context: &ProtectedTahiniContext
        ) -> Result<MarketingPolicy, String> {
        let (p1, p2) = self.0.extract_policies();
        match context.service.as_str() {
            "Advertisement" => match context.rpc.as_str() {
                "auction_bidding" => Ok(MarketingPolicy {
                    no_storage: p2.storage,
                    targeted_ads_consent: p1.targeted_ads_consent,
                    third_party_processing: p2.third_party_consent.clone(),
                }),
                _ => panic!("Transformation not allowed for this RPC"),
            },
            _ => panic!("Transformation not allowed for this service"),
        }
    }
}


