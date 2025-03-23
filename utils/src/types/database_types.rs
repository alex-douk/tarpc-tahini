use alohomora::TahiniType;
use alohomora::bbox::BBox;
use alohomora::rocket::RequestBBoxJson;
use alohomora::tarpc::traits::TahiniError;
use alohomora::tarpc::{TahiniEnum, TahiniType};
use serde::{Deserialize, Serialize};

use crate::policies::PromptPolicy;
use crate::policies::shared_policies::UsernamePolicy;
use crate::types::inference_types::Message;

// use super::inference_types::BBoxConversation;

#[derive(Deserialize, Clone, TahiniType)]
pub struct DatabaseStoreForm {
    pub uuid: BBox<String, UsernamePolicy>,
    pub conv_id: BBox<Option<String>, UsernamePolicy>,
    pub message: BBox<Message, PromptPolicy>,
}

#[derive(Deserialize, Clone, TahiniType, RequestBBoxJson)]
pub struct DatabaseRetrieveForm {
    pub uuid: BBox<String, UsernamePolicy>,
    pub conv_id: CHATUID,
}

pub type CHATUID = BBox<String, UsernamePolicy>;

pub type DatabaseRecord = DatabaseStoreForm;

#[derive(Serialize, Deserialize, Clone, TahiniType, Debug)]
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
