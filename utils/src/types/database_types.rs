use alohomora::rocket::RequestBBoxJson;
use alohomora::TahiniType;
use alohomora::bbox::BBox;
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
    pub conv_id: CHATUID
}

pub type CHATUID = BBox<String, UsernamePolicy>;

pub type DatabaseRecord = DatabaseStoreForm;
