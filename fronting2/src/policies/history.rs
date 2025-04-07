use alohomora::policy::{AnyPolicy, Policy, PolicyAnd};

#[derive(Clone)]
pub struct HistoryPolicy;

impl Policy for HistoryPolicy {
    fn name(&self) -> String {
        "HistoryPolicy".to_string()
    }

    fn check(
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

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        if other.is::<HistoryPolicy>() {
            self.join_logic(other.specialize::<HistoryPolicy>().unwrap())
                .map(|x| AnyPolicy::new(x))
        } else {
            let and = PolicyAnd::new(Self, other);
            Ok(AnyPolicy::new(and))
        }
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(other)
    }
}

