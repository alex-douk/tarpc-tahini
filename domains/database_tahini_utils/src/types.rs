use tahini_tarpc::TahiniType;
use alohomora::bbox::BBox;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

use core_tahini_utils::policies::MessagePolicy;
use core_tahini_utils::types::Message;

use crate::policies::UserIdDBPolicy;

// use super::inference_types::BBoxConversation;

#[derive(TahiniDeserialize, Clone, TahiniType)]
pub struct DatabaseStoreForm {
    pub uuid: BBox<String, UserIdDBPolicy>,
    pub conv_id: BBox<Option<String>, UserIdDBPolicy>,
    pub message: BBox<Message, MessagePolicy>,
}

#[derive(TahiniDeserialize, Clone, TahiniType)]
pub struct DatabaseRetrieveForm {
    pub uuid: BBox<String, UserIdDBPolicy>,
    pub conv_id: CHATUID,
}

#[derive(TahiniDeserialize, Clone, TahiniType)]
pub struct DeleteForm {
    pub uuid: BBox<String, UserIdDBPolicy>,
    //TODO(douk): Change to conv metadata policy
    pub conv_id: BBox<String, UserIdDBPolicy>
}

pub type CHATUID = BBox<String, UserIdDBPolicy>;

#[derive(TahiniSerialize, TahiniDeserialize, Clone, TahiniType, Debug)]
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
impl tahini_tarpc::traits::TahiniError for DatabaseError {}

#[derive(TahiniSerialize, TahiniDeserialize, Debug, Clone)]
pub struct PolicyError;

impl std::fmt::Display for PolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed policy check")
    }
}

impl std::error::Error for PolicyError {}
impl tahini_tarpc::traits::TahiniError for PolicyError {}
