use tarpc::serde::{Deserialize, Serialize};
use alohomora::context::UnprotectedContext;
use alohomora::policy::{AnyPolicy, Policy, Reason};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExamplePolicy {
    pub field: u32,
}

impl Policy for ExamplePolicy {
    fn name(&self) -> String {
        String::from("ExamplePolicy")
    }
    fn check(&self, _context: &UnprotectedContext, _reason: Reason<'_>) -> bool {
        true
    }
    fn join(&self, _other: AnyPolicy) -> Result<AnyPolicy, ()> {
        todo!()
    }
    fn join_logic(&self, other: Self) -> Result<Self, ()> {
        Ok(other)
    }
}
