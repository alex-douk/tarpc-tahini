use alohomora::{
    bbox::BBox, policy::{Policy, PolicyFrom}, tarpc::TahiniTransformFrom
};
use core_tahini_utils::policies::MessagePolicy;

pub struct LeakyPolicy;

impl Policy for LeakyPolicy {
    fn name(&self) -> String {
        "LeakyPolicy".to_string()
    }

    fn check(
        &self,
        context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        true
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(other)
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(other)
    }
}

impl PolicyFrom<MessagePolicy> for LeakyPolicy {
    fn from_policy(
        other_policy: MessagePolicy,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(LeakyPolicy)
    }
}
