use core_tahini_utils::types::{LLMResponse, UserPrompt};

#[tarpc::service]
pub trait Inference {
    async fn inference(prompt: UserPrompt) -> LLMResponse;
}
