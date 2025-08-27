use crate::{
    types::{CHATUID, DatabaseError, DatabaseRetrieveForm, DatabaseStoreForm},
};
use core_tahini_utils::types::Conversation;
use core_tahini_utils::{
    types::Message,
};


#[tarpc::service]
pub trait Database {
    ///Stores a given message from the LLM conversation for a given (user_id, conversation_id)
    ///pair. Will use a local database policy on the server side for uuid and conv_id.
    async fn store_prompt(
        uuid: String,
        conv_id: Option<String>,
        message: Message,
    ) -> CHATUID;

    ///Retrieves a conversation (if it exists) for a given (uuid, conv_id) pair.
    ///The policy attached to the data is company-wide MessagePolicy. No downgrade allowed.
    async fn retrieve_prompt(
        uuid: String,
        conv_id: String,
    ) -> Option<Conversation>;

    ///Check if a user exists in a database, and return its User ID if that's the case.
    async fn fetch_user(
        username: String,
    ) -> Result<String, DatabaseError>;

    async fn register_user(
        username: String,
        //UUIDPolicy
    ) -> Result<String, DatabaseError>;

    ///Fetches the list of conversation IDs for a given username.
    async fn fetch_history_headers(
        //This is UUID
        username: String,
    ) -> Vec<String>;

    ///Deletes a conversation from the database for a given (user_id, conv_id) pair.
    async fn delete_conversation(
        data: (String, String),
    ) -> bool;
}
