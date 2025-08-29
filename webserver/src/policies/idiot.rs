use alohomora::{
    bbox::BBox, policy::Policy
};
use alohomora::policy::SimplePolicy;
use tahini_tarpc::{traits::PolicyFrom, TahiniTransformFrom};
use core_tahini_utils::policies::MessagePolicy;

pub struct LeakyPolicy;

impl SimplePolicy for LeakyPolicy {
    fn simple_name(&self) -> String {
        "LeakyPolicy".to_string()
    }

    fn simple_check(
        &self,
        context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        true
    }

    fn simple_join_direct(&mut self, other: &mut Self) {}
}

impl PolicyFrom<MessagePolicy> for LeakyPolicy {
    fn from_policy(
        other_policy: MessagePolicy,
        context: &tahini_tarpc::context::TahiniContext,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(LeakyPolicy)
    }
}
