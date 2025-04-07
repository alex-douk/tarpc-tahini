use alohomora::policy::{Policy, PolicyFrom, PolicyInto};
use database_tahini_utils::policies::UserIdDBPolicy;

use crate::policies::login_uuid::UserIdWebPolicy;

impl PolicyFrom<UserIdDBPolicy> for UserIdWebPolicy {
    fn from_policy(
        other_policy: UserIdDBPolicy,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        let error = Err(format!(
            "Policy transformation not allowed from {}",
            other_policy.name()
        ));
        match context.service {
            "Database" => Ok(UserIdWebPolicy),
            _ => error,
        }
    }
}

impl PolicyInto<UserIdDBPolicy> for UserIdWebPolicy {
    fn into_policy(
        self,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<UserIdDBPolicy, String> {
        match context.service {
            "Database" => Ok(UserIdDBPolicy),
            _ => Err(format!(
                "Policy transformation not allowed from {}",
                self.name()
            )),
        }
    }
}
