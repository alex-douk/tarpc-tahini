use alohomora::policy::Policy;
use alohomora::policy::PolicyFrom;
use alohomora::tarpc::context::TahiniContext;
use database_tahini_utils::policies::ConversationMetadataPolicy;

use crate::policies::history::HistoryPolicy;

impl PolicyFrom<ConversationMetadataPolicy> for HistoryPolicy {
    fn from_policy(
        other_policy: ConversationMetadataPolicy,
        context: &TahiniContext,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        let err = Err(format!(
            "Could not parse {} into HistoryPolicy with context: {}.{}",
            other_policy.name(),
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
