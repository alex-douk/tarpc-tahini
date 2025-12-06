use sesame::policy::{Policy, Reason, SimplePolicy};
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct AdPolicy;

impl SimplePolicy for AdPolicy {
    fn simple_name(&self) -> String {
        "AdPolicy".to_string()
    }

    fn simple_check(
        &self,
        _context: &sesame::context::UnprotectedContext,
        reason: sesame::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::Response => true,
            _ => false,
        }
    }
    fn simple_join_direct(&mut self, other: &mut Self) {}
}
