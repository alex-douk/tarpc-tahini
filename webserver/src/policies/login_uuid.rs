use sesame::policy::{AnyPolicy, Policy, PolicyAnd, Reason, SimplePolicy};
use sesame_rocket::policy::FrontendPolicy;

///Policy that is only allowed when sending the user its UUID, either via cookie or body
///Could be extended for tahini_check to include sending to DB
#[derive(Clone)]
pub struct UserIdWebPolicy;

impl SimplePolicy for UserIdWebPolicy {
    fn simple_name(&self) -> String {
        "UUIDLoginPolicy".to_string()
    }
    fn simple_check(
        &self,
        _context: &sesame::context::UnprotectedContext,
        reason: sesame::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::Response => true,
            //This one here only exists because we dont have secure tokens
            Reason::Cookie("uuid") => true,
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, other: &mut Self) {}
}

impl FrontendPolicy for UserIdWebPolicy {
    fn from_cookie<'a, 'r>(
        _name: &str,
        _cookie: &'a rocket::http::Cookie<'static>,
        _request: &'a rocket::Request<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        Self
    }
    fn from_request<'a, 'r>(_request: &'a rocket::Request<'r>) -> Self
    where
        Self: Sized,
    {
        Self
    }
}
