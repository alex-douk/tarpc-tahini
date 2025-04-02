use crate::{
    policies::{ConversationMetadataPolicy, shared_policies::UsernamePolicy},
    types::{
        PolicyError,
        database_types::{
            CHATUID, DatabaseError, DatabaseRecord, DatabaseRetrieveForm, DatabaseStoreForm,
        },
        inference_types::BBoxConversation,
    },
};

use alohomora::{
    bbox::BBox, tahini_service, tarpc::{client::TahiniStub, TahiniType}
};

#[tahini_service(domain = internal)]
pub trait Database {
    // TODO(douk): #[tahini_checked]
    async fn store_prompt(prompt: DatabaseStoreForm) -> Result<CHATUID, PolicyError>;
    async fn retrieve_prompt(retrieve: DatabaseRetrieveForm) -> Option<BBoxConversation>;
    async fn fetch_user(
        username: BBox<String, UsernamePolicy>,
    ) -> Result<BBox<String, UsernamePolicy>, DatabaseError>;
    async fn register_user(
        username: BBox<String, UsernamePolicy>,
    ) -> Result<BBox<String, UsernamePolicy>, DatabaseError>;
    async fn fetch_history_headers(
        username: BBox<String, UsernamePolicy>,
    ) -> Vec<BBox<String, ConversationMetadataPolicy>>;
    async fn get_default_user() -> BBox<String, UsernamePolicy>;
    async fn delete_conversation(
        user_id: BBox<String, UsernamePolicy>,
        conv_id: BBox<String, ConversationMetadataPolicy>,
    ) -> bool;
}
