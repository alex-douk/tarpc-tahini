use crate::policies::message_policy::InferenceReason;
use mysql::Value;
use serde_json::from_str;
use sesame::policy::{Join, Reason, SimplePolicy};
use sesame_mysql::{schema_policy, PConFromValue, SchemaPolicy};
use sesame_rocket::policy::FrontendPolicy;
use std::collections::HashMap;
use std::str::FromStr;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

pub static THIRD_PARTY_PROCESSORS: [&str; 2] = ["Meta_Ads", "Google_Ads"];

///This policy is user-and-session-bound and
///is invoked in operations that could lead to current-or-future disclosure of the username
#[schema_policy(table = "users", column = 1)]
// #[schema_policy(table = "conversations", column = 2)]
#[derive(TahiniSerialize, TahiniDeserialize, Clone, Debug, Default, PartialEq)]
pub struct UsernamePolicy {
    pub targeted_ads_consent: bool,
    pub third_party_vendors_consent: HashMap<String, bool>,
}

impl SimplePolicy for UsernamePolicy {
    fn simple_name(&self) -> String {
        "UsernamePolicy".to_string()
    }

    fn simple_check(
        &self,
        context: &sesame::context::UnprotectedContext,
        reason: Reason<'_>,
    ) -> bool {
        match reason {
            Reason::Response => true,
            Reason::DB(_, _) => true,
            Reason::Custom(reason) => match reason.downcast_ref::<InferenceReason>() {
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

    fn simple_join_direct(&mut self, other: &mut Self) {
        self.targeted_ads_consent = self.targeted_ads_consent && other.targeted_ads_consent;
        self.third_party_vendors_consent = self
            .third_party_vendors_consent
            .iter_mut()
            .filter_map(|(k, v)| {
                if other.third_party_vendors_consent.contains_key(k) {
                    Some((k.clone(), v.clone()))
                } else {
                    None
                }
            })
            .collect();
    }
}

impl SchemaPolicy for UsernamePolicy {
    fn from_row(table_name: &str, row: &Vec<Value>) -> Self
    where
        Self: Sized,
    {
        let value = match table_name {
            "users" => <String as PConFromValue>::from_value(row[3].clone()),
            "conversations" => <String as PConFromValue>::from_value(row[9].clone()),
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
                "users" => PConFromValue::from_value(row[2].clone()),
                "conversations" => PConFromValue::from_value(row[8].clone()),
                _ => false,
            },
        }
    }
}

impl FrontendPolicy for UsernamePolicy {
    fn from_request<'a, 'r>(request: &'a rocket::Request<'r>) -> Self
    where
        Self: Sized,
    {
        let mut hashmap = HashMap::with_capacity(THIRD_PARTY_PROCESSORS.len());
        for vendor in THIRD_PARTY_PROCESSORS {
            let cookie = request.cookies().get(vendor);
            hashmap.insert(
                vendor.to_string(),
                match cookie {
                    None => false,
                    Some(c) => bool::from_str(c.value()).unwrap_or(false),
                },
            );
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

    fn from_cookie<'a, 'r>(
        name: &str,
        cookie: &'a rocket::http::Cookie<'static>,
        request: &'a rocket::Request<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        let mut hashmap = HashMap::with_capacity(THIRD_PARTY_PROCESSORS.len());
        for vendor in THIRD_PARTY_PROCESSORS {
            let cookie = request.cookies().get(vendor);
            hashmap.insert(
                vendor.to_string(),
                match cookie {
                    None => false,
                    Some(c) => bool::from_str(c.value()).unwrap_or(false),
                },
            );
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
#[derive(TahiniDeserialize, TahiniSerialize, Clone, Debug)]
pub struct AbsolutePolicy {}

impl SimplePolicy for AbsolutePolicy {
    fn simple_name(&self) -> String {
        "AbsolutePolicy".to_string()
    }
    fn simple_check(
        &self,
        _context: &sesame::context::UnprotectedContext,
        _reason: Reason<'_>,
    ) -> bool {
        false
    }

    fn simple_join_direct(&mut self, other: &mut Self) {}
}
