use alohomora::policy::{AnyPolicy, FrontendPolicy, Policy, PolicyAnd, Reason};

///Policy that is only allowed when sending the user its UUID, either via cookie or body
///Could be extended for tahini_check to include sending to DB
#[derive(Clone)]
pub struct UserIdWebPolicy;

impl Policy for UserIdWebPolicy {
    fn name(&self) -> String {
        "UUIDLoginPolicy".to_string()
    }
    fn check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::Response => true,
            //This one here only exists because we dont have secure tokens
            Reason::Cookie("uuid") => true,
            _ => false,
        }
    }
    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(AnyPolicy::new(PolicyAnd::new(Self, other)))
    }
    fn join_logic(&self, _other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(Self)
    }
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
