use alohomora::db::{BBoxFromValue, Value};
use alohomora::policy::{AnyPolicy, FrontendPolicy, Policy, Reason, SchemaPolicy, schema_policy};
use alohomora::rocket::{BBoxCookie, RocketCookie, RocketRequest};
use std::str::FromStr;
use tarpc::serde::{Deserialize, Serialize};

#[schema_policy(table = "users", column = 0)]
#[schema_policy(table = "users", column = 1)]
#[schema_policy(table = "conversations", column = 2)]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UsernamePolicy {
    pub targeted_ads_consent: bool,
}

impl Policy for UsernamePolicy {
    fn name(&self) -> String {
        "UsernamePolicy".to_string()
    }

    fn check(&self, context: &alohomora::context::UnprotectedContext, reason: Reason<'_>) -> bool {
        match reason {
            Reason::Response => true,
            Reason::DB(_, _) => true,
            _ => false,
        }
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(self.clone().into_any())
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
        Self {
            targeted_ads_consent: match table_name {
                "users" => BBoxFromValue::from_value(row[2].clone()),
                "conversations" => BBoxFromValue::from_value(row[6].clone()),
                _ => false, //Default no consent to targeted ads (utopist)
            },
        }
    }
}

impl FrontendPolicy for UsernamePolicy {
    fn from_cookie<'a, 'r>(
        name: &str,
        cookie: &'a RocketCookie<'static>,
        request: &'a RocketRequest<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        UsernamePolicy {
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
        UsernamePolicy {
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

#[derive(Clone)]
pub struct AbsolutePolicy {}

impl Policy for AbsolutePolicy {
    fn name(&self) -> String {
        "AbsolutePolicy".to_string()
    }
    fn check(&self, context: &alohomora::context::UnprotectedContext, reason: Reason<'_>) -> bool {
        false
    }
    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
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
