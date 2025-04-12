use crate::{
    policies::{ConversationMetadataPolicy, UserIdDBPolicy},
    types::{
        DatabaseError, DatabaseRetrieveForm, DatabaseStoreForm, PolicyError, CHATUID
    },
};
use core_tahini_utils::{policies::{MessagePolicy, UsernamePolicy}, types::Message};
use core_tahini_utils::types::BBoxConversation;

use alohomora::{
    bbox::BBox,
    tahini_service, allow_client_transform,
    tarpc::{TahiniType, client::TahiniStub},
};

#[tahini_service(domain = company)]
pub trait Database {
    #[allow_client_transform]
    async fn store_prompt(uuid: BBox<String, UserIdDBPolicy>, conv_id: BBox<Option<String>, UserIdDBPolicy>, message: BBox<Message, MessagePolicy>) -> Result<CHATUID, PolicyError>;
    async fn retrieve_prompt(uuid: BBox<String, UserIdDBPolicy>, conv_id: BBox<String, UserIdDBPolicy>) -> Option<BBoxConversation>;
    #[allow_client_transform]
    async fn fetch_user(
        username: BBox<String, UsernamePolicy>,
        //Should return a UUIDPolicy
    ) -> Result<BBox<String, UserIdDBPolicy>, DatabaseError>;
    #[allow_client_transform]
    async fn register_user(
        username: BBox<String, UsernamePolicy>,
        //UUIDPolicy
    ) -> Result<BBox<String, UserIdDBPolicy>, DatabaseError>;
    #[allow_client_transform]
    async fn fetch_history_headers(
        //This is UUID
        username: BBox<String, UsernamePolicy>,
    ) -> Vec<BBox<String, ConversationMetadataPolicy>>;
    async fn delete_conversation(
        data: (BBox<String, UserIdDBPolicy>, BBox<String, UserIdDBPolicy>),
    ) -> bool;
}
