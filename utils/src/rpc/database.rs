use crate::types::{database_types::{DatabaseRecord, DatabaseRetrieveForm, DatabaseStoreForm, CHATUID}, inference_types::BBoxConversation};

use alohomora::{
    tahini_service,
    tarpc::client::TahiniStub,
    tarpc::TahiniType
};

#[tahini_service]
pub trait Database {
    async fn store_prompt(prompt: DatabaseStoreForm) -> CHATUID;
    async fn retrieve_prompt(retrieve: DatabaseRetrieveForm) -> Option<BBoxConversation>;
}
