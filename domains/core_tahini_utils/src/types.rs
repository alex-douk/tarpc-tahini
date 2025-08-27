use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPrompt {
    pub conversation: Conversation,
    pub nb_token: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LLMResponse {
    pub infered_tokens: Result<Message, LLMError>
}

pub type Conversation = Vec<Message>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
