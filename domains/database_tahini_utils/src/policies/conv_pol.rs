use mysql::Value;
use rocket::{http::Cookie, Request};
use sesame::{pcon::PCon, policy::{AnyPolicy, NoPolicy, Policy, PolicyAnd, Reason, SimplePolicy}, SesameType};
use sesame_mysql::{schema_policy, SchemaPolicy};
use sesame_rocket::{policy::FrontendPolicy, rocket::{FromPConRequest, PConRequest, PConRequestOutcome}};
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

///A policy for conversational metadata (such as conversation id)
///Only allows for authenticated disclosure, and even so, only on specific routes.
///While extensible, this policy aims to be used in a user-only context, i.e., no disclosure
///to any third-parties.
#[derive(TahiniSerialize, TahiniDeserialize, Clone, Debug, Default)]
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

#[derive(TahiniSerialize, TahiniDeserialize, Clone, Debug, Default)]
#[schema_policy(table = "conv_view", column = 0)]
#[schema_policy(table = "conversations", column = 3)]
#[schema_policy(table = "conversations", column = 4)]
pub struct ConversationAccessPolicy {
    pub user_id: Option<String>,
}

#[derive(SesameType, Clone, Debug)]
// #[sesame_out_type(verbatim = [config])]
pub struct UserIDContextData {
    pub user_id: PCon<String, NoPolicy>,
}

#[rocket::async_trait]
impl<'a, 'r> FromPConRequest<'a, 'r> for UserIDContextData {
    type PConError = ();

    async fn from_pcon_request(
        request: PConRequest<'a, 'r>,
    ) -> PConRequestOutcome<Self, Self::PConError> {
        let uid_opt = request.cookies().get::<NoPolicy>("user_id");
        match uid_opt {
            None => {
                let empty_uuid = PCon::new("".to_string(), NoPolicy {});
                PConRequestOutcome::Success(UserIDContextData {
                    user_id: empty_uuid,
                })
            }
            Some(uid) => {
                let pcon = uid.into();
                PConRequestOutcome::Success(UserIDContextData {
                user_id: pcon
            })
            }
        }
    }
}

impl SimplePolicy for ConversationAccessPolicy {
    fn simple_name(&self) -> String {
        "ConversationAccessPolicy".to_string()
    }

    fn simple_check(
        &self,
        context: &sesame::context::UnprotectedContext,
        reason: Reason<'_>,
    ) -> bool {
        match self.user_id {
            None => {false},
            Some(ref auth_uid) => {
                let is_correct_auth = match context.downcast_ref::<UserIDContextDataOut>() {
                    None => { false},
                    Some(uid) => {
                        uid.user_id.clone().eq(auth_uid)
                    }
                };
                match reason {
                    Reason::Response => is_correct_auth,
                    Reason::Redirect(_) => is_correct_auth,
                    _ => false,
                }
            }
        }
    }
    fn simple_join_direct(&mut self, other: &mut Self) {
        //If both are valid + they are equal, then nothing happens.
        //Otherwise, we consider an error
        if let Some(ref my_uid) = self.user_id {
            if let Some(ref other_uid) = other.user_id {
                if my_uid.eq(other_uid) {
                    return;
                }
            }
        }
        self.user_id = None
    }
}

impl SchemaPolicy for ConversationAccessPolicy {
    fn from_row(table_name: &str, row: &Vec<mysql::Value>) -> Self
    where
        Self: Sized,
    {

        match table_name {
            "conversations" => {
                let uid = mysql::from_value::<String>(row[2].clone());
                Self {
                    user_id: Some(uid),
                }
            }
            "conv_view" => {
                let uid = mysql::from_value::<String>(row[1].clone());
                Self {
                    user_id: Some(uid),
                }
            }
            _ => panic!("ConversationAccessPolicy constructor is invoked on a wrong table")
        }
    }
}
