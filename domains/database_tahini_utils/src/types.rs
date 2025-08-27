use serde::{Deserialize, Serialize};
use core_tahini_utils::types::Message;


#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseStoreForm {
    pub uuid: String,
    pub conv_id: Option<String>,
    pub message: Message
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseRetrieveForm {
    pub uuid: String,
    pub conv_id: CHATUID,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeleteForm {
    pub uuid: String,
    //TODO(douk): Change to conv metadata policy
    pub conv_id: String
}

pub type CHATUID = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DatabaseError {
    UserNotFound,
    AlreadyExists,
    InternalError,
    Ambiguous,
}

impl std::error::Error for DatabaseError {}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            DatabaseError::UserNotFound => "User not found in the database",
            DatabaseError::AlreadyExists => "User already exists in the database",
            DatabaseError::InternalError => "Internal Error",
            DatabaseError::Ambiguous => "Multiple entries found when one was expected",
        };
        write!(f, "{}", string)
    }
}
