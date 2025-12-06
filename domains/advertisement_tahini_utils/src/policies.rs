
use sesame::policy::{Policy, Reason, SimplePolicy};
use tahini_tarpc::{TahiniSerialize, TahiniDeserialize};
pub static THIRD_PARTY_PROCESSORS: [&str; 2] = ["Meta_Ads", "Google_Ads"];

///This policy is given by an external organization so that remote clients can
///be compatible with it. This policies contains:
///- A storage consent
///- A targeted ads consent (.e.g, locally processed but still sent to the particular user)
///- Consent for various next-hops services.
///
///Note the lack of information regarding unprotected services (yet)
#[derive(TahiniSerialize, TahiniDeserialize, Clone, Debug)]
pub struct MarketingPolicy {
    pub no_storage: bool,
    pub targeted_ads_consent: bool,
    pub third_party_ad_vendors_allowed: Vec<String>,
}

impl SimplePolicy for MarketingPolicy {
    fn simple_name(&self) -> String {
        "MarketingPolicy".to_string()
    }

    fn simple_check(
        &self,
        _context: &sesame::context::UnprotectedContext,
        reason: sesame::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::DB(_, _) => !self.no_storage,
            Reason::Response => true,
            //If we have a custom  reason, it needs to be an inference reason
            Reason::Custom(reason) => match reason.downcast_ref::<MarketingReason>() {
                None => {
                    println!("We are failing the downcast to MarketingReason");
                    false
                }
                Some(reason) => match reason {
                    //If it is, we check the inference reason
                    MarketingReason::Email => self.targeted_ads_consent,
                    MarketingReason::ThirdPartyProcessing(ref vendor) => {
                        self.third_party_ad_vendors_allowed.contains(vendor)
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

    fn simple_join_direct(&mut self, other: &mut Self) {
        self.no_storage = self.no_storage && other.no_storage;
        self.targeted_ads_consent = self.targeted_ads_consent && other.targeted_ads_consent;
        self.third_party_ad_vendors_allowed.extend(other.third_party_ad_vendors_allowed.iter().map(|v| v.clone()));
    }

    /*
    fn join(
        &self,
        other: sesame::policy::AnyPolicy,
    ) -> Result<sesame::policy::AnyPolicy, ()> {
        Ok(other)
    }

    fn join_logic(&self, _other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(self.clone())
    }
     */
}

#[derive(TahiniSerialize, TahiniDeserialize, Clone, Debug)]
pub struct AdPolicy {}

#[derive(Clone)]
pub enum MarketingReason {
    Email,
    ThirdPartyProcessing(String),
}
