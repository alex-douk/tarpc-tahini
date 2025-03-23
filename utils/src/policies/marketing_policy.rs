use std::collections::HashMap;

use alohomora::policy::{Policy, Reason};
use tarpc::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketingPolicy {
    pub no_storage: bool,
    pub email_consent: bool,
    // pub third_party_processing: bool
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
                    MarketingReason::Email => self.email_consent,
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
