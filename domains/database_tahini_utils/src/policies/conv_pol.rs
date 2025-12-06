use mysql::Value;
use rocket::{http::Cookie, Request};
use sesame::policy::{AnyPolicy, Policy, PolicyAnd, Reason, SimplePolicy};
use sesame_mysql::{schema_policy, SchemaPolicy};
use sesame_rocket::policy::FrontendPolicy;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

///A policy for conversational metadata (such as conversation id)
///Only allows for authenticated disclosure, and even so, only on specific routes.
///While extensible, this policy aims to be used in a user-only context, i.e., no disclosure
///to any third-parties.
#[derive(TahiniSerialize, TahiniDeserialize, Clone, Debug, Default)]
#[schema_policy(table = "conversations", column = 1)]
pub struct ConversationMetadataPolicy;

impl SimplePolicy for ConversationMetadataPolicy {
    fn simple_name(&self) -> String {
        "ConversationMetadataPolicy".to_string()
    }

    fn simple_check(
        &self,
        context: &sesame::context::UnprotectedContext,
        reason: Reason<'_>,
    ) -> bool {
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

    fn simple_join_direct(&mut self, other: &mut Self) {}
}

impl SchemaPolicy for ConversationMetadataPolicy {
    fn from_row(_table_name: &str, _row: &Vec<Value>) -> Self
    where
        Self: Sized,
    {
        Self
    }
}

impl FrontendPolicy for ConversationMetadataPolicy {
    fn from_cookie<'a, 'r>(
        _name: &str,
        _cookie: &'a Cookie<'static>,
        _request: &'a Request<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        Self
    }
    fn from_request<'a, 'r>(_request: &'a Request<'r>) -> Self
    where
        Self: Sized,
    {
        Self
    }
}
