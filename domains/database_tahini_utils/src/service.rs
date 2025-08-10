use crate::{
    policies::{ConversationMetadataPolicy, UserIdDBPolicy},
    types::{CHATUID, DatabaseError, DatabaseRetrieveForm, DatabaseStoreForm, PolicyError},
};
use core_tahini_utils::types::BBoxConversation as PConConversation;
use core_tahini_utils::{
    policies::{MessagePolicy, UsernamePolicy},
    types::Message,
};

use tahini_tarpc::{allow_client_transform, tahini_service, TahiniType, client::TahiniStub};
use alohomora::{
    bbox::BBox as PCon,
};

#[tahini_service(domain = company)]
pub trait Database {
    ///Stores a given message from the LLM conversation for a given (user_id, conversation_id)
    ///pair. Will use a local database policy on the server side for uuid and conv_id.
    #[allow_client_transform]
    async fn store_prompt(
        uuid: PCon<String, UserIdDBPolicy>,
        conv_id: PCon<Option<String>, UserIdDBPolicy>,
        message: PCon<Message, MessagePolicy>,
    ) -> Result<CHATUID, PolicyError>;

    ///Retrieves a conversation (if it exists) for a given (uuid, conv_id) pair.
    ///The policy attached to the data is company-wide MessagePolicy. No downgrade allowed.
    async fn retrieve_prompt(
        uuid: PCon<String, UserIdDBPolicy>,
        conv_id: PCon<String, UserIdDBPolicy>,
    ) -> Option<PConConversation>;

    ///Check if a user exists in a database, and return its User ID if that's the case.
    #[allow_client_transform]
    async fn fetch_user(
        username: PCon<String, UsernamePolicy>,
    ) -> Result<PCon<String, UserIdDBPolicy>, DatabaseError>;

    #[allow_client_transform]
    async fn register_user(
        username: PCon<String, UsernamePolicy>,
        //UUIDPolicy
    ) -> Result<PCon<String, UserIdDBPolicy>, DatabaseError>;

    ///Fetches the list of conversation IDs for a given username.
    #[allow_client_transform]
    async fn fetch_history_headers(
        //This is UUID
        username: PCon<String, UsernamePolicy>,
    ) -> Vec<PCon<String, ConversationMetadataPolicy>>;

    ///Deletes a conversation from the database for a given (user_id, conv_id) pair.
    async fn delete_conversation(
        data: (PCon<String, UserIdDBPolicy>, PCon<String, UserIdDBPolicy>),
    ) -> bool;
}
