use alohomora::TahiniType;
use alohomora::bbox::BBox;
use alohomora::rocket::RequestBBoxJson;
use alohomora::tarpc::TahiniType;
use alohomora::tarpc::{TahiniDeserialize, TahiniSerialize};

use core_tahini_utils::policies::PromptPolicy;
use core_tahini_utils::policies::UsernamePolicy;
use core_tahini_utils::types::Message;

// use super::inference_types::BBoxConversation;

#[derive(TahiniDeserialize, Clone, TahiniType)]
pub struct DatabaseStoreForm {
    pub uuid: BBox<String, UsernamePolicy>,
    pub conv_id: BBox<Option<String>, UsernamePolicy>,
    pub message: BBox<Message, PromptPolicy>,
}

#[derive(TahiniDeserialize, Clone, TahiniType, RequestBBoxJson)]
pub struct DatabaseRetrieveForm {
    pub uuid: BBox<String, UsernamePolicy>,
    pub conv_id: CHATUID,
}

pub type CHATUID = BBox<String, UsernamePolicy>;

// pub type DatabaseRecord = DatabaseStoreForm;

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
impl alohomora::tarpc::traits::TahiniError for DatabaseError {}

#[derive(TahiniSerialize, TahiniDeserialize, Debug, Clone)]
pub struct PolicyError;

impl std::fmt::Display for PolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed policy check")
    }
}

impl std::error::Error for PolicyError {}
impl alohomora::tarpc::traits::TahiniError for PolicyError {}
