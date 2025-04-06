use core_tahini_utils::policies::UsernamePolicy;
use core_tahini_utils::types::BBoxConversation;
use crate::{
    policies::ConversationMetadataPolicy,
    types::{
        PolicyError, CHATUID, DatabaseError, DatabaseRetrieveForm, DatabaseStoreForm,
    },
};

use alohomora::{
    bbox::BBox, tahini_service, tarpc::{client::TahiniStub, TahiniType}
};

#[tahini_service(domain = internal)]
pub trait Database {
    // TODO(douk): #[tahini_checked]
    //
    // pub struct DatabaseStoreForm {
    //     pub uuid: BBox<String, UUIDPolicy>,
    //     pub conv_id: BBox<Option<String>, ConversationMetadataPolicy>,
    //     pub message: BBox<Message, PromptPolicy>,
    // }
    async fn store_prompt(prompt: DatabaseStoreForm) -> Result<CHATUID, PolicyError>;
    async fn retrieve_prompt(retrieve: DatabaseRetrieveForm) -> Option<BBoxConversation>;
    async fn fetch_user(
        username: BBox<String, UsernamePolicy>,
        //Should return a UUIDPolicy
    ) -> Result<BBox<String, UsernamePolicy>, DatabaseError>;
    async fn register_user(
        username: BBox<String, UsernamePolicy>,
        //UUIDPolicy
    ) -> Result<BBox<String, UsernamePolicy>, DatabaseError>;
    async fn fetch_history_headers(
        //This is UUID
        username: BBox<String, UsernamePolicy>,
    ) -> Vec<BBox<String, ConversationMetadataPolicy>>;
    //UUIDPolicy
    async fn get_default_user() -> BBox<String, UsernamePolicy>;
    async fn delete_conversation(
        //UUID
        user_id: BBox<String, UsernamePolicy>,
        //ConvMetadataPolicy
        conv_id: BBox<String, ConversationMetadataPolicy>,
    ) -> bool;
}
