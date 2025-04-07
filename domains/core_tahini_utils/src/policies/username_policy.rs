use crate::policies::message_policy::InferenceReason;
use alohomora::db::{BBoxFromValue, Value};
use alohomora::policy::{
    AnyPolicy, FrontendPolicy, Policy, PolicyAnd, Reason, SchemaPolicy,
    schema_policy,
};
use alohomora::rocket::{RocketCookie, RocketRequest};
use serde_json::from_str;
use std::collections::HashMap;
use std::str::FromStr;
use tarpc::serde::{Deserialize, Serialize};

use super::MessagePolicy;
pub static THIRD_PARTY_PROCESSORS: [&str; 2] = ["Meta_Ads", "Google_Ads"];

///This policy is user-and-session-bound and
///is invoked in operations that could lead to current-or-future disclosure of the username
#[schema_policy(table = "users", column = 1)]
// #[schema_policy(table = "conversations", column = 2)]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UsernamePolicy {
    pub targeted_ads_consent: bool,
    pub third_party_vendors_consent: HashMap<String, bool>,
}

impl Policy for UsernamePolicy {
    fn name(&self) -> String {
        "UsernamePolicy".to_string()
    }

    fn check(&self, context: &alohomora::context::UnprotectedContext, reason: Reason<'_>) -> bool {
        match reason {
            Reason::Response => true,
            Reason::DB(_, _) => true,
            Reason::Custom(reason) => match reason.cast().downcast_ref::<InferenceReason>() {
                None => false,
                Some(reason) => match reason {
                    //TODO(douk): Check if verifying the third-party vendor list now makes sense.
                    //I believe we just propagate it.
                    InferenceReason::SendToMarketing => self.targeted_ads_consent,
                    //If it is, we check the inference reason
                    _ => false,
                },
            },
            _ => false,
        }
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        if other.is::<UsernamePolicy>() {
            self.join_logic(other.specialize().map_err(|_| ())?)
                .map(|p| AnyPolicy::new(p))
        } else if other.is::<MessagePolicy>() {
            let spec = other.specialize::<MessagePolicy>();
            if spec.is_err() {
                return Err(());
            }

            Ok(AnyPolicy::new(PolicyAnd::new(self.clone(), spec.unwrap())))
        } else {
            Ok(AnyPolicy::new(PolicyAnd::new(self.clone(), other)))
        }
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(self.clone())
    }

    fn into_any(self) -> AnyPolicy
    where
        Self: Sized,
    {
        AnyPolicy::new(self)
    }
}

impl SchemaPolicy for UsernamePolicy {
    fn from_row(table_name: &str, row: &Vec<Value>) -> Self
    where
        Self: Sized,
    {
        let value = match table_name {
            "users" => <String as BBoxFromValue>::from_value(row[3].clone()),
            "conversations" => <String as BBoxFromValue>::from_value(row[9].clone()),
            _ => "{}".to_string(),
        };
        let hashmap = match from_str(value.as_str()) {
            Ok(map) => map,
            Err(_) => {
                eprintln!(
                    "Couldn't parse consent table into the proper type, got {}",
                    value
                );
                HashMap::<String, bool>::new()
            }
        };
        Self {
            third_party_vendors_consent: hashmap,
            targeted_ads_consent: match table_name {
                "users" => BBoxFromValue::from_value(row[2].clone()),
                "conversations" => BBoxFromValue::from_value(row[8].clone()),
                _ => false,
            },
        }
    }
}

impl FrontendPolicy for UsernamePolicy {
    fn from_cookie<'a, 'r>(
        _name: &str,
        _cookie: &'a RocketCookie<'static>,
        request: &'a RocketRequest<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        let mut hashmap = HashMap::with_capacity(THIRD_PARTY_PROCESSORS.len());
        for vendor in THIRD_PARTY_PROCESSORS {
            let cookie = request.cookies().get(vendor);
            hashmap.insert(vendor.to_string(), match cookie {
                None => false,
                Some(c) => bool::from_str(c.value()).unwrap_or(false),
            });
        }
        UsernamePolicy {
            third_party_vendors_consent: hashmap,
            targeted_ads_consent: match request.cookies().get("targeted_ads") {
                None => false,
                Some(c) => match bool::from_str(c.value()) {
                    Ok(b) => b,
                    Err(_) => false,
                },
            },
        }
    }

    fn from_request<'a, 'r>(request: &'a RocketRequest<'r>) -> Self
    where
        Self: Sized,
    {
        let mut hashmap = HashMap::with_capacity(THIRD_PARTY_PROCESSORS.len());
        for vendor in THIRD_PARTY_PROCESSORS {
            let cookie = request.cookies().get(vendor);
            hashmap.insert(vendor.to_string(), match cookie {
                None => false,
                Some(c) => bool::from_str(c.value()).unwrap_or(false),
            });
        }
        UsernamePolicy {
            third_party_vendors_consent: hashmap,
            targeted_ads_consent: match request.cookies().get("targeted_ads") {
                None => false,
                Some(c) => match bool::from_str(c.value()) {
                    Ok(b) => b,
                    Err(_) => false,
                },
            },
        }
    }
}

///Used for internal processing. Can be passed around at unchecked RPCs, but can never leave the
///org nor be passed to checked RPCs.
///Such a policy can ensure that data paths terminating in an uncontrolled sink are taken into
///account.
#[derive(Deserialize, Serialize, Clone)]
pub struct AbsolutePolicy {}

impl Policy for AbsolutePolicy {
    fn name(&self) -> String {
        "AbsolutePolicy".to_string()
    }
    fn check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        _reason: Reason<'_>,
    ) -> bool {
        false
    }
    fn join(
        &self,
        _other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(self.clone().into_any())
    }
    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(other)
    }
    fn into_any(self) -> alohomora::policy::AnyPolicy
    where
        Self: Sized,
    {
        AnyPolicy::new(self)
    }
}

// impl PolicyInto<MarketingPolicy> for PolicyAnd<UsernamePolicy, PromptPolicy> {
//     fn into_policy(self, context: &TahiniContext) -> Result<MarketingPolicy, String> {
//         let (p1, p2) = self.extract_policies();
//         match context.service {
//             "Advertisement" => match context.rpc {
//                 "auction_bidding" => Ok(MarketingPolicy {
//                     no_storage: p2.storage,
//                     targeted_ads_consent: p1.targeted_ads_consent,
//                     third_party_processing: p2.third_party_consent.clone(),
//                 }),
//                 _ => panic!("Transformation not allowed for this RPC"),
//             },
//             _ => panic!("Transformation not allowed for this service"),
//         }
//     }
// }
