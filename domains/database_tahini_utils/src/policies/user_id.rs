use alohomora::policy::{AnyPolicy, Policy, PolicyAnd, Reason, SchemaPolicy, schema_policy};
use alohomora::tarpc::{TahiniDeserialize, TahiniSerialize};
#[derive(TahiniDeserialize, TahiniSerialize, Clone)]
#[schema_policy(table = "users", column = 0)]
#[schema_policy(table = "conversations", column = 2)]
pub struct UserIdDBPolicy;

impl Policy for UserIdDBPolicy {
    fn name(&self) -> String {
        "UserIdDBPolicy".to_string()
    }
    fn check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::DB(ref _query, _) => true, //query.starts_with("INSERT") || query.starts_with("SELECT"),
            _ => false,
        }
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        if other.is::<Self>() {
            Ok(AnyPolicy::new(UserIdDBPolicy))
        } else {
            Ok(AnyPolicy::new(PolicyAnd::new(UserIdDBPolicy, other)))
        }
    }

    fn join_logic(&self, _other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}

impl SchemaPolicy for UserIdDBPolicy {
    fn from_row(_table_name: &str, _row: &Vec<alohomora::db::Value>) -> Self
    where
        Self: Sized,
    {
        Self
    }
}
