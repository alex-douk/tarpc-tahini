use std::collections::HashMap;

use crate::types::inference_types::{LLMResponse, UserPrompt};
use alohomora::tarpc::{
    client::{TahiniChannel, TahiniNewClient, TahiniRequestDispatch, TahiniStub},
    enums::TahiniSafeWrapper,
    server::TahiniServe,
    TahiniEnum, TahiniType, TahiniVariantsEnum,
};
use alohomora::{tahini_service, TahiniType};
use tarpc::{
    client::{Config, RpcError},
    context::Context,
};
use tarpc::{serde::Deserialize, ClientMessage, Response, Transport};

#[tahini_service]
pub trait Inference {
    async fn inference(prompt: UserPrompt) -> LLMResponse;
}
