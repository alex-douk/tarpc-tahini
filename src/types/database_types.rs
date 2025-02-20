use std::collections::HashMap;

use alohomora::tarpc::{TahiniEnum, TahiniType};
use alohomora::bbox::BBox;
use tarpc::serde::{Deserialize, Serialize};

use crate::policies::PromptPolicy;

use alohomora::TahiniType;
#[derive(Deserialize, Clone, TahiniType)]
pub struct DatabaseSubmit {
    pub user: String,
    pub full_prompt: BBox<String, PromptPolicy>
}

pub type DBUUID = BBox<u32, PromptPolicy>;

pub type DatabaseRecord = DatabaseSubmit;
