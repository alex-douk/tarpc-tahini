use crate::types::database_types::{DatabaseRecord, DatabaseSubmit, DBUUID};

use alohomora::{
    tahini_service,
    tarpc::client::TahiniStub,
    tarpc::TahiniType
};

#[tahini_service]
pub trait Database {
    async fn store_prompt(prompt: DatabaseSubmit) -> DBUUID;
    async fn retrieve_prompt(user: String, uuid: DBUUID) -> Option<DatabaseRecord>;
}
