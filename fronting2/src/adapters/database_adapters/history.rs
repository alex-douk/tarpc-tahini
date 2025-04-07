use alohomora::policy::Policy;
use alohomora::policy::PolicyInto;
use core_tahini_utils::policies::AbsolutePolicy;

use crate::policies::history::HistoryPolicy;

impl PolicyInto<HistoryPolicy> for AbsolutePolicy {
    fn into_policy(
        self,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<HistoryPolicy, String> {
        match context.service {
            "Database" => match context.service {
                "fetch_history_headers" => Ok(HistoryPolicy),
                _ => Err(format!(
                    "Could not parse {} into HistoryPolicy",
                    self.name()
                )),
            },
            _ => Err(format!(
                "Could not parse {} into HistoryPolicy",
                self.name()
            )),
        }
    }
}
