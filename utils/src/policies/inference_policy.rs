use alohomora::policy::Policy;
use tarpc::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptPolicy {
    pub consent: bool
}

impl Policy for PromptPolicy {
    fn name(&self) -> String {
        "PromptPolicy".to_string()
    }

    fn check(&self, context: &alohomora::context::UnprotectedContext, reason: alohomora::policy::Reason<'_>) -> bool {
        true
    }

    fn join(&self, other: alohomora::policy::AnyPolicy) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(other)
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()> where Self: Sized {
        Ok(self.clone())
    }
}
