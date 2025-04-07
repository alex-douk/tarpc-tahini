use crate::{
    policies::{ConversationMetadataPolicy, UserIdDBPolicy},
    types::{
        CHATUID, DatabaseError, DatabaseRetrieveForm, DatabaseStoreForm, DeleteForm, PolicyError,
    },
};
use core_tahini_utils::policies::{AbsolutePolicy, UsernamePolicy};
use core_tahini_utils::types::BBoxConversation;

use alohomora::{
    bbox::BBox,
    tahini_service,
    tarpc::{TahiniType, client::TahiniStub},
};

#[tahini_service(domain = company)]
pub trait Database {
    // TODO(douk): #[tahini_checked]
    async fn store_prompt(prompt: DatabaseStoreForm) -> Result<CHATUID, PolicyError>;
    async fn retrieve_prompt(retrieve: DatabaseRetrieveForm) -> Option<BBoxConversation>;
    async fn fetch_user(
        username: BBox<String, UsernamePolicy>,
        //Should return a UUIDPolicy
    ) -> Result<BBox<String, UserIdDBPolicy>, DatabaseError>;
    async fn register_user(
        username: BBox<String, UsernamePolicy>,
        //UUIDPolicy
    ) -> Result<BBox<String, UserIdDBPolicy>, DatabaseError>;
    async fn fetch_history_headers(
        //This is UUID
        username: BBox<String, UsernamePolicy>,
    ) -> Vec<BBox<String, AbsolutePolicy>>;
    async fn delete_conversation(
        data: (BBox<String, UserIdDBPolicy>, BBox<String, UserIdDBPolicy>),
    ) -> bool;
}
