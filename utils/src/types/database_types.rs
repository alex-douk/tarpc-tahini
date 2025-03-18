use alohomora::TahiniType;
use alohomora::bbox::BBox;
use alohomora::tarpc::{TahiniEnum, TahiniType};
use serde::{Deserialize, Serialize};

use crate::policies::PromptPolicy;
use crate::policies::shared_policies::UsernamePolicy;

use super::inference_types::BBoxConversation;

#[derive(Deserialize, Clone, TahiniType)]
pub struct DatabaseStoreForm {
    pub user: BBox<String, UsernamePolicy>,
    pub full_prompt: BBoxConversation,
}

#[derive(Deserialize, Clone, TahiniType)]
pub struct DatabaseRetrieveForm {
    pub user: BBox<String, UsernamePolicy>,
    pub conv_id: CHATUID
}

pub type CHATUID = BBox<String, UsernamePolicy>;

pub type DatabaseRecord = DatabaseStoreForm;
