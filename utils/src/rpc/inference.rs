use crate::types::inference_types::{LLMResponse, UserPrompt};
use alohomora::tarpc::{
    client::TahiniStub,
    TahiniType,
};
use alohomora::tahini_service;

#[tahini_service(domain=internal)]
pub trait Inference {
    async fn inference(prompt: UserPrompt) -> LLMResponse;
}
