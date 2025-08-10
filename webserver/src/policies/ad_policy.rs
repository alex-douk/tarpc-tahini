use alohomora::policy::{Policy, Reason};
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct AdPolicy;

impl Policy for AdPolicy {
    fn name(&self) -> String {
        "AdPolicy".to_string()
    }

    fn check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::Response => true,
            _ => false,
        }
    }
    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(other)
    }
    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(other)
    }
}
