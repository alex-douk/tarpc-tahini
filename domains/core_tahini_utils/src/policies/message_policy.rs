use alohomora::db::{BBoxFromValue, Value};
use alohomora::policy::{schema_policy, AnyPolicy, PolicyAnd, SimplePolicy};
use tahini_tarpc::traits::PolicyFrom;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

use alohomora::{
    policy::{FrontendPolicy, Policy, Reason, SchemaPolicy},
    rocket::{RocketCookie, RocketRequest},
};
use serde_json::from_str;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use super::UsernamePolicy;
pub static THIRD_PARTY_PROCESSORS: [&str; 2] = ["Meta_Ads", "Google_Ads"];

///This policy is invoked when the use of conversation/message information
///Three main fields are invoked here:
///The storage to database (i.e. ephemeral chats)
///Allowing to send anonymized data to Tahini-fied third-parties
///Allowing the use of unprotected third-party services (e.g. image gen)
#[derive(TahiniSerialize, TahiniDeserialize, Clone, Debug, Default, PartialEq)]
#[schema_policy(table = "conversations", column = 3)]
#[schema_policy(table = "conversations", column = 4)]
pub struct MessagePolicy {
    pub storage: bool,
    pub marketing_consent: bool,
    pub third_party_ad_vendors_allowed: Vec<String>,
    pub unprotected_image_gen: bool,
    pub reinforcement_learning_consent: bool,
}

impl SimplePolicy for MessagePolicy {
    fn simple_name(&self) -> String {
        "PromptPolicy".to_string()
    }

    fn simple_check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::DB(_, _) => self.storage,
            Reason::Response => true,
            //If we have a custom  reason, it needs to be an inference reason
            Reason::Custom(reason) => match reason.downcast_ref::<InferenceReason>() {
                None => false,
                Some(reason) => match reason {
                    //If it is, we check the inference reason
                    InferenceReason::SendToMarketing => self.marketing_consent,
                    InferenceReason::SendToImageGen => self.unprotected_image_gen,
                    InferenceReason::SendToDB => self.storage,
                },
            },
            //If it is not as a direct query response, a DB request, or an inference specific
            //purpose, we deny
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, other: &mut Self) {
        self.storage = self.storage && other.storage;
        self.marketing_consent = self.marketing_consent && other.marketing_consent;
        self.third_party_ad_vendors_allowed.retain(|k| other.third_party_ad_vendors_allowed.contains(k));
        self.unprotected_image_gen = self.unprotected_image_gen && other.unprotected_image_gen;
        self.reinforcement_learning_consent = self.reinforcement_learning_consent && other.reinforcement_learning_consent;
    }
}

#[derive(Clone)]
pub enum InferenceReason {
    SendToMarketing,
    SendToImageGen,
    SendToDB,
}

impl FrontendPolicy for MessagePolicy {
    fn from_request<'a, 'r>(request: &'a RocketRequest<'r>) -> Self
    where
        Self: Sized,
    {
        let no_storage = bool::from_str(request.cookies().get("storage").unwrap().value()).unwrap();
        let marketing_consent =
            bool::from_str(request.cookies().get("ads").unwrap().value()).unwrap();
        let unprotected_image_gen =
            bool::from_str(request.cookies().get("image_gen").unwrap().value()).unwrap();



        let vendor_allow_list = request.cookies().get("third_party_ad_vendors_allowed").map_or_else(
            Vec::new,
            |c| {
                serde_json::from_str::<Vec<String>>(c.value()).expect("Couldn't parse the provided vendors list")
            }
        );
        // let mut third_party_ad_vendors_allowed = Vec::with_capacity(THIRD_PARTY_PROCESSORS.len());
        // for vendor in THIRD_PARTY_PROCESSORS {
        //     match request.cookies().get(vendor) {
        //         None => (),
        //         Some(_) => 
        //
        //     }
        //
        //     hashmap.insert(
        //         vendor.to_string(),
        //         match cookie {
        //             None => false,
        //             Some(c) => bool::from_str(c.value()).unwrap_or(false),
        //         },
        //     );
        // }
        // let reinforcement_learning_consent =
        // bool::from_str(request.cookies().get("rl_consent").unwrap().value()).unwrap();
        MessagePolicy {
            third_party_ad_vendors_allowed: vendor_allow_list,
            storage: no_storage,
            marketing_consent,
            unprotected_image_gen,
            reinforcement_learning_consent: false, // reinforcement_learning_consent
        }
    }

    fn from_cookie<'a, 'r>(
        _name: &str,
        _cookie: &'a RocketCookie<'static>,
        request: &'a RocketRequest<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        Self::from_request(request)
    }
}

impl SchemaPolicy for MessagePolicy {
    fn from_row(_table_name: &str, row: &Vec<Value>) -> Self
    where
        Self: Sized,
    {
        let value = <String as BBoxFromValue>::from_value(row[9].clone());
        let hashmap = match from_str(value.as_str()) {
            Ok(map) => map,
            Err(_) => {
                eprintln!(
                    "Couldn't parse consent table into the proper type, got {}",
                    value
                );
                Vec::new()
            }
        };
        MessagePolicy {
            third_party_ad_vendors_allowed: hashmap,
            storage: BBoxFromValue::from_value(row[5].clone()),
            marketing_consent: BBoxFromValue::from_value(row[6].clone()),
            unprotected_image_gen: BBoxFromValue::from_value(row[7].clone()),
            reinforcement_learning_consent: false,
        }
    }
}
