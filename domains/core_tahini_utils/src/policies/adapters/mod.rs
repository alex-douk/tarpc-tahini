use alohomora::policy::Policy;
pub struct Adapter<P: Policy>(pub P);
#[cfg(feature="advertisement")]
pub mod advertisement;

impl<P: Policy> From<P> for Adapter<P> {
    fn from(value: P) -> Self {
        Self(value)
    }
}

impl<P: Policy> Policy for Adapter<P> {
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
