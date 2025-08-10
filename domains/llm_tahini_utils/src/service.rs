use core_tahini_utils::types::{LLMResponse, UserPrompt};
use tahini_tarpc::enums;
use tahini_tarpc::{
    client::TahiniStub,
    TahiniType,
};
use tahini_tarpc::tahini_service;

#[tahini_service(domain=internal)]
pub trait Inference {
    async fn inference(prompt: UserPrompt) -> LLMResponse;
}
