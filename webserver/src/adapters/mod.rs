pub mod ad_adapter;
pub mod database_adapters;
use alohomora::policy::{Join, MutRefReflection, NotAPolicyContainer, OwnedReflection, Policy, PolicyDyn, RefReflection, Reflective, SimplePolicy};

pub struct PolicyAdapter<P: Policy>(pub P);

impl<P: Policy> From<P> for PolicyAdapter<P> {
    fn from(value: P) -> Self {
        Self(value)
    }
}

impl<P: Policy> !NotAPolicyContainer for PolicyAdapter<P> {}

impl<P: Policy> Join for PolicyAdapter<P> {
    fn can_join_with(&mut self, _p: &MutRefReflection<'_>) -> bool {
        false
    }
    fn join_via_reflection(&mut self, _p: MutRefReflection<'_>) -> bool {
        false
    }
}

impl<P: Policy> Reflective for PolicyAdapter<P> {
    fn reflect_static(self: Box<Self>) -> OwnedReflection<'static>
    where
        Self: 'static,
    {
        todo!()
    }
    fn reflect_mut_ref(&mut self) -> MutRefReflection<'_> {
        todo!()
    }
    fn reflect_ref(&self) -> RefReflection<'_> {
        todo!()
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
}
