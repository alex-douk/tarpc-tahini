use alohomora::bbox::BBox;
use alohomora::rocket::{RequestBBoxJson, ResponseBBoxJson};
use alohomora::tarpc::{TahiniEnum, TahiniType};
use alohomora::{AlohomoraType, TahiniType};
use std::collections::HashMap;
use alohomora::tarpc::{TahiniDeserialize, TahiniSerialize};

use crate::policies::MessagePolicy;

//#[derive(TahiniType)]
#[derive(TahiniDeserialize, Clone, Debug, TahiniType)]
pub struct UserPrompt {
    pub conversation: BBoxConversation,
    pub nb_token: u32,
}

#[derive(TahiniDeserialize, Clone, Debug, TahiniType)]
pub struct LLMResponse {
    pub infered_tokens: BBox<Result<Message, LLMError>, MessagePolicy>,
}

pub type BBoxConversation = BBox<Vec<Message>, MessagePolicy>;

#[derive(TahiniSerialize, RequestBBoxJson, TahiniDeserialize, Clone, Debug, ResponseBBoxJson)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, TahiniDeserialize, TahiniSerialize, Clone)]
pub enum LLMError {
    InternalError,
    ValidationError,
}

impl std::fmt::Display for LLMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal LLM Error")
    }
}

impl std::error::Error for LLMError {}
impl alohomora::tarpc::traits::TahiniError for LLMError {}
