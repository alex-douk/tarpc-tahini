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
    pub conversation: BBoxConversation,
    pub nb_token: u32,
}

#[derive(Deserialize, Clone, Debug, TahiniType)]
pub struct LLMResponse {
    pub infered_tokens: BBox<Result<String, LLMError>, PromptPolicy>,
}

pub type BBoxConversation = BBox<Vec<ConversationRound>, PromptPolicy>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConversationRound{
    pub user: String,
    pub assistant: String
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
