use alohomora::policy::{FrontendPolicy, Policy, Reason};
use tarpc::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UsernamePolicy{}

impl Policy for UsernamePolicy{

    fn name(&self) -> String {
        "UsernamePolicy".to_string()
    }

    fn check(&self, context: &alohomora::context::UnprotectedContext, reason: Reason<'_>) -> bool {
        true
    }

    fn join(&self, other: alohomora::policy::AnyPolicy) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(self.clone().into_any())
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()> where Self: Sized {
        Ok(self.clone())
    }
}
