pub mod ad_adapter;
pub mod database_adapters;
use alohomora::policy::Policy;

pub struct PolicyAdapter<P: Policy>(pub P);

impl<P: Policy> From<P> for PolicyAdapter<P> {
    fn from(value: P) -> Self {
        Self(value)
    }
}

impl<P: Policy> Policy for PolicyAdapter<P> {
    fn name(&self) -> String {
        self.0.name()
    }
    fn check(
        &self,
        context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        self.0.check(context, reason)
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        self.0.join(other)
    }
    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        self.0.join_logic(other.0).map(|x| Self(x))
    }
}
