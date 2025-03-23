use crate::{policies::{shared_policies::UsernamePolicy, ConversationMetadataPolicy}, types::{database_types::{DatabaseRecord, DatabaseRetrieveForm, DatabaseStoreForm, CHATUID}, inference_types::BBoxConversation}};

use alohomora::{
    bbox::BBox, tahini_service, tarpc::{client::TahiniStub, TahiniType}
};

#[tahini_service]
pub trait Database {
    async fn store_prompt(prompt: DatabaseStoreForm) -> CHATUID;
    async fn retrieve_prompt(retrieve: DatabaseRetrieveForm) -> Option<BBoxConversation>;
    async fn fetch_or_insert_user(username: BBox<String, UsernamePolicy>) -> BBox<String, UsernamePolicy>;
    async fn fetch_history_headers(username: BBox<String, ConversationMetadataPolicy>) -> Vec<BBox<String, ConversationMetadataPolicy>>;
    async fn get_default_user() -> BBox<String, UsernamePolicy>;
}
