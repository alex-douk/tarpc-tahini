use alohomora::policy::{AnyPolicy, Policy, PolicyAnd, SimplePolicy};

#[derive(Clone)]
pub struct HistoryPolicy;

impl SimplePolicy for HistoryPolicy {
    fn simple_name(&self) -> String {
        "HistoryPolicy".to_string()
    }

    fn simple_check(
        &self,
        context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            alohomora::policy::Reason::Response => {
                if context.route == "history" {
                    match context.data.downcast_ref::<bool>() {
                        Some(auth) => *auth,
                        None => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, other: &mut Self) {}
}

