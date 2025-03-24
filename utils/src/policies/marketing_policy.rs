use std::collections::HashMap;

use alohomora::policy::{Policy, Reason};
use tarpc::serde::{Deserialize, Serialize};


pub static THIRD_PARTY_PROCESSORS : [&str; 2]= ["Meta_Ads", "Google_Ads"];

///This policy is given by an external organization so that remote clients can
///be compatible with it. This policies contains:
///- A storage consent
///- A targeted ads consent (.e.g, locally processed but still sent to the particular user)
///- Consent for various next-hops services.
///
///Note the lack of information regarding unprotected services (yet)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketingPolicy {
    pub no_storage: bool,
    pub targeted_ads_consent: bool,
    pub third_party_processing: HashMap<String, bool>,
}

impl Policy for MarketingPolicy {
    fn name(&self) -> String {
        "MarketingPolicy".to_string()
    }

    fn check(
        &self,
        context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::DB(_, _) => !self.no_storage,
            Reason::Response => true,
            //If we have a custom  reason, it needs to be an inference reason
            Reason::Custom(reason) => match reason.cast().downcast_ref::<MarketingReason>() {
                None => {
                    println!("We are failing the downcast to MarketingReason");
                    false
                }
                Some(reason) => match reason {
                    //If it is, we check the inference reason
                    MarketingReason::Email => self.targeted_ads_consent,
                    MarketingReason::ThirdPartyProcessing(vendor) => {
                        match self.third_party_processing.get(vendor) {
                            None => false,
                            Some(b) => *b,
                        }
                    }
                },
            },
            //If it is not as a direct query response, a DB request, or an inference specific
            //purpose, we deny
            _ => {
                println!("We are invoking for no good reason!");
                false
            }
        }
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(other)
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(self.clone())
    }
}

#[derive(Clone)]
pub enum MarketingReason {
    Email,
    ThirdPartyProcessing(String),
}
