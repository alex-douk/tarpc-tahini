use std::collections::HashMap;

use alohomora::bbox::BBox;
use alohomora::tarpc::{TahiniEnum, TahiniType};
use alohomora::{AlohomoraType, TahiniType};
use tarpc::serde::{Deserialize, Serialize};

use super::database_types::{DatabaseSubmit, DBUUID};
use crate::policies::PromptPolicy;

//#[derive(TahiniType)]
#[derive(Deserialize, Clone, Debug, TahiniType)]
pub struct UserPrompt {
    pub user: String,
    pub prompt: BBox<String, PromptPolicy>,
    pub nb_token: u32,
}

#[derive(Deserialize, Clone, Debug, TahiniType)]
pub struct LLMResponse {
    pub infered_tokens: BBox<Result<String, LLMError>, PromptPolicy>,
    // Why do we attach the same policy?
    // Because same level of confidentality. Debatable
    // Return a None for the uuid in case of an LLM error
    pub db_uuid: BBox<Option<u32>, PromptPolicy>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LLMError {
    InternalError,
}

impl std::fmt::Display for LLMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal LLM Error")
    }
}

impl std::error::Error for LLMError {}
impl alohomora::tarpc::traits::TahiniError for LLMError {}
