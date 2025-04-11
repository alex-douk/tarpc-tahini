use alohomora::policy::Policy;
use alohomora::policy::PolicyInto;
use database_tahini_utils::policies::ConversationMetadataPolicy;

use crate::policies::history::HistoryPolicy;

impl PolicyInto<HistoryPolicy> for ConversationMetadataPolicy {
    fn into_policy(
        self,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<HistoryPolicy, String> {
        let err = Err(format!(
            "Could not parse {} into HistoryPolicy with context: {}.{}",
            self.name(),
            context.service,
            context.rpc
        ));
        match context.service.as_str() {
            "Database" => match context.rpc.as_str() {
                "fetch_history_headers" => Ok(HistoryPolicy),
                _ => err,
            },
            _ => err,
        }
    }
}
