use alohomora::policy::Policy;
use tahini_tarpc::traits::{PolicyFrom, PolicyInto};
use database_tahini_utils::policies::UserIdDBPolicy;

use crate::policies::login_uuid::UserIdWebPolicy;

impl PolicyFrom<UserIdDBPolicy> for UserIdWebPolicy {
    fn from_policy(
        other_policy: UserIdDBPolicy,
        context: &tahini_tarpc::context::TahiniContext,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        let error = Err(format!(
            "Policy transformation not allowed from {}",
            other_policy.name()
        ));
        match context.service.as_str() {
            "Database" => Ok(UserIdWebPolicy),
            _ => error,
        }
    }
}

impl PolicyInto<UserIdDBPolicy> for UserIdWebPolicy {
    fn into_policy(
        self,
        context: &tahini_tarpc::context::TahiniContext,
    ) -> Result<UserIdDBPolicy, String> {
        match context.service.as_str() {
            "Database" => Ok(UserIdDBPolicy),
            _ => Err(format!(
                "Policy transformation not allowed from {}",
                self.name()
            )),
        }
    }
}
