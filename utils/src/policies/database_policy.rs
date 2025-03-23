use alohomora::db::{BBoxFromValue, Value};
use alohomora::policy::{AnyPolicy, schema_policy};
use alohomora::{
    policy::{FrontendPolicy, Policy, Reason, SchemaPolicy},
    rocket::{RocketCookie, RocketRequest},
};
use std::str::FromStr;
use tarpc::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[schema_policy(table = "conversations", column = 1)]
pub struct ConversationMetadataPolicy {}

impl Policy for ConversationMetadataPolicy {
    fn name(&self) -> String {
        "ConversationMetadataPolicy".to_string()
    }

    fn check(&self, context: &alohomora::context::UnprotectedContext, reason: Reason<'_>) -> bool {
        match reason {
            // Reason::DB(query, _) => query.starts_with("INSERT") || query.starts_with("SELECT"),
            Reason::DB(_, _) => true,
            Reason::Response => match context.route.as_str() {
                "history" => match context.data.downcast_ref::<bool>() {
                    None => false,
                    Some(auth) => *auth,
                },
                _ => true,
            },
            _ => false,
        }
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        Ok(other)
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(other)
    }

    fn into_any(self) -> AnyPolicy
    where
        Self: Sized,
    {
        AnyPolicy::new(self)
    }
}

impl SchemaPolicy for ConversationMetadataPolicy {
    fn from_row(table_name: &str, row: &Vec<Value>) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

impl FrontendPolicy for ConversationMetadataPolicy {
    fn from_cookie<'a, 'r>(
        name: &str,
        cookie: &'a RocketCookie<'static>,
        request: &'a RocketRequest<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn from_request<'a, 'r>(request: &'a RocketRequest<'r>) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}
