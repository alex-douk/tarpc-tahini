use crate::policies::{PromptPolicy, UsernamePolicy};
use advertisement_tahini_utils::policies::MarketingPolicy;
use alohomora::policy::{PolicyAnd, PolicyInto};
use alohomora::tarpc::context::TahiniContext;

impl PolicyInto<MarketingPolicy> for super::Adapter<PolicyAnd<UsernamePolicy, PromptPolicy>> {
    fn into_policy(self, context: &TahiniContext) -> Result<MarketingPolicy, String> {
        let (p1, p2) = self.0.extract_policies();
        match context.service {
            "Advertisement" => match context.rpc {
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
