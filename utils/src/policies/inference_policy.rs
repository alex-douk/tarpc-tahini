use alohomora::db::{BBoxFromValue, Value};
use alohomora::policy::{AnyPolicy, schema_policy};
use alohomora::{
    policy::{FrontendPolicy, Policy, Reason, SchemaPolicy},
    rocket::{RocketCookie, RocketRequest},
};
use std::collections::HashMap;
use std::str::FromStr;
use tarpc::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[schema_policy(table = "conversations", column = 3)]
#[schema_policy(table = "conversations", column = 4)]
pub struct PromptPolicy {
    pub storage: bool,
    pub marketing_consent: bool,
    //TODO(douk): Add third-party consent
    // pub third_party_consent: HashMap<String, bool>
    pub unprotected_image_gen: bool,
}

impl Policy for PromptPolicy {
    fn name(&self) -> String {
        "PromptPolicy".to_string()
    }

    fn check(
        &self,
        context: &alohomora::context::UnprotectedContext,
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
        if other.is::<PromptPolicy>() {
            self.join_logic(other.specialize().map_err(|_| ())?)
                .map(|pol| pol.into_any())
        } else {
            Ok(self.clone().into_any())
        }
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        //Take the OR of each
        Ok(PromptPolicy {
            //We explicitely annotate when we DONT want to store
            storage: self.storage && other.storage,
            marketing_consent: self.marketing_consent && other.marketing_consent,
            unprotected_image_gen: self.unprotected_image_gen && other.unprotected_image_gen,
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

impl FrontendPolicy for PromptPolicy {
    fn from_request<'a, 'r>(request: &'a RocketRequest<'r>) -> Self
    where
        Self: Sized,
    {
        let no_storage =
            bool::from_str(request.cookies().get("storage").unwrap().value()).unwrap();
        let marketing_consent =
            bool::from_str(request.cookies().get("ads").unwrap().value()).unwrap();
        let unprotected_image_gen =
            bool::from_str(request.cookies().get("image_gen").unwrap().value()).unwrap();
        PromptPolicy {
            storage: no_storage,
            marketing_consent,
            unprotected_image_gen,
        }
    }

    fn from_cookie<'a, 'r>(
        name: &str,
        cookie: &'a RocketCookie<'static>,
        request: &'a RocketRequest<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        let storage =
            bool::from_str(request.cookies().get("storage").unwrap().value()).unwrap();
        let marketing_consent =
            bool::from_str(request.cookies().get("ads").unwrap().value()).unwrap();
        let unprotected_image_gen =
            bool::from_str(request.cookies().get("image_gen").unwrap().value()).unwrap();
        PromptPolicy {
            storage,
            marketing_consent,
            unprotected_image_gen,
        }
    }
}

impl SchemaPolicy for PromptPolicy {
    fn from_row(table_name: &str, row: &Vec<Value>) -> Self
    where
        Self: Sized,
    {
        PromptPolicy {
            storage: BBoxFromValue::from_value(row[5].clone()),
            marketing_consent: BBoxFromValue::from_value(row[6].clone()),
            unprotected_image_gen: BBoxFromValue::from_value(row[7].clone()),
        }
    }
}
