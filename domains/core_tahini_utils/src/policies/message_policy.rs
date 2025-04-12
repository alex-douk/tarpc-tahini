use alohomora::db::{BBoxFromValue, Value};
use alohomora::policy::{schema_policy, AnyPolicy, PolicyAnd, PolicyFrom};
use alohomora::tarpc::{TahiniDeserialize, TahiniSerialize};
use alohomora::{
    policy::{FrontendPolicy, Policy, Reason, SchemaPolicy},
    rocket::{RocketCookie, RocketRequest},
};
use serde_json::from_str;
use std::collections::HashMap;
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
    pub third_party_consent: HashMap<String, bool>,
    pub unprotected_image_gen: bool,
    pub reinforcement_learning_consent: bool,
}

impl Policy for MessagePolicy {
    fn name(&self) -> String {
        "PromptPolicy".to_string()
    }

    fn check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::DB(_, _) => self.storage,
            Reason::Response => true,
            //If we have a custom  reason, it needs to be an inference reason
            Reason::Custom(reason) => match reason.cast().downcast_ref::<InferenceReason>() {
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

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        if other.is::<MessagePolicy>() {
            self.join_logic(other.specialize().map_err(|_| ())?)
                .map(|pol| pol.into_any())
        } else if other.is::<UsernamePolicy>() {
            let spec = other.specialize::<UsernamePolicy>();
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
        //Merge the two policies
        let mut hashmap = self.third_party_consent.clone();
        for (key, value) in other.third_party_consent.iter() {
            hashmap
                .entry(key.clone())
                .and_modify(|e| *e = *e && *value)
                .or_insert(*value);
        }

        //Take the AND of each
        Ok(MessagePolicy {
            third_party_consent: hashmap,
            storage: self.storage && other.storage,
            marketing_consent: self.marketing_consent && other.marketing_consent,
            unprotected_image_gen: self.unprotected_image_gen && other.unprotected_image_gen,
            reinforcement_learning_consent: self.reinforcement_learning_consent
                && other.reinforcement_learning_consent,
        })
    }

    fn into_any(self) -> alohomora::policy::AnyPolicy
    where
        Self: Sized,
    {
        AnyPolicy::new(self)
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

        let mut hashmap = HashMap::with_capacity(THIRD_PARTY_PROCESSORS.len());
        for vendor in THIRD_PARTY_PROCESSORS {
            let cookie = request.cookies().get(vendor);
            hashmap.insert(vendor.to_string(), match cookie {
                None => false,
                Some(c) => bool::from_str(c.value()).unwrap_or(false),
            });
        }
        // let reinforcement_learning_consent =
            // bool::from_str(request.cookies().get("rl_consent").unwrap().value()).unwrap();
        MessagePolicy {
            third_party_consent: hashmap,
            storage: no_storage,
            marketing_consent,
            unprotected_image_gen,
            reinforcement_learning_consent: false
            // reinforcement_learning_consent
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
                HashMap::<String, bool>::new()
            }
        };
        MessagePolicy {
            third_party_consent: hashmap,
            storage: BBoxFromValue::from_value(row[5].clone()),
            marketing_consent: BBoxFromValue::from_value(row[6].clone()),
            unprotected_image_gen: BBoxFromValue::from_value(row[7].clone()),
            //TODO(douk): Change schema to take into account
            reinforcement_learning_consent: false
        }
    }
}
