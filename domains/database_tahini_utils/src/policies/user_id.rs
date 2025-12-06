use sesame::policy::{AnyPolicy, Policy, PolicyAnd, Reason, SimplePolicy};
use sesame_mysql::{schema_policy, SchemaPolicy};
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
#[derive(TahiniDeserialize, TahiniSerialize, Clone, Debug)]
#[schema_policy(table = "users", column = 0)]
#[schema_policy(table = "conversations", column = 2)]
pub struct UserIdDBPolicy;

impl SimplePolicy for UserIdDBPolicy {
    fn simple_name(&self) -> String {
        "UserIdDBPolicy".to_string()
    }
    fn simple_check(
        &self,
        _context: &sesame::context::UnprotectedContext,
        reason: sesame::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::DB(ref _query, _) => true, //query.starts_with("INSERT") || query.starts_with("SELECT"),
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, other: &mut Self) {}
}

impl SchemaPolicy for UserIdDBPolicy {
    fn from_row(_table_name: &str, _row: &Vec<mysql::Value>) -> Self
    where
        Self: Sized,
    {
        Self
    }
}
