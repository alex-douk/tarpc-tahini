use alohomora::AlohomoraType;
use ::serde::{Deserialize, Serialize};
use tarpc::service;

#[derive(Debug)]
pub struct DummyBox<T, P> {
    inner : T,
    policy: P
}

impl<T, P> DummyBox<T, P> {
    pub fn get_val(&self) -> &T {
        &self.inner
    }

    pub fn new(prompt : T, pol: P) -> Self {
        DummyBox {
            inner: prompt,
            policy: pol
        }
    }
}

use std::{pin::Pin, 
    sync::Arc,
    iter::FromFn};

use crate::types::{LLMResponse, LLMResponseOut, UserPrompt};
// use tokio::sync::Mutex; // Tokio's async Mutex

#[macro_use]
pub mod tcp;

// pub mod serde;


#[service]
pub trait Inference {
    async fn inference(prompt: UserPrompt) -> LLMResponse;
}


#[derive(Deserialize, Serialize)]
pub enum InferenceRequestOut {
    Inference {
        prompt: <UserPrompt as AlohomoraType>::Out
    }
}

// #[derive(Serialize)]
// pub enum InferenceResponseOut {
//     Inference(<LLMResponse as AlohomoraType>::Out)
// }


impl AlohomoraType for InferenceRequest {
    type Out = InferenceRequestOut;

    fn to_enum(self) -> alohomora::AlohomoraTypeEnum {
        match self {
            InferenceRequest::Inference { prompt } => prompt.to_enum()
        }
    }

    fn from_enum(e: alohomora::AlohomoraTypeEnum) -> Result<Self::Out, ()> {
        Ok(InferenceRequestOut::Inference { prompt: UserPrompt::from_enum(e)?})
    }
}

// impl AlohomoraType for InferenceResponse {
//     type Out = InferenceResponseOut;
//
//     fn to_enum(self) -> alohomora::AlohomoraTypeEnum {
//         match self {
//             InferenceResponse::Inference(rsp) => rsp.to_enum()
//         }
//     }
//
//     fn from_enum(e: alohomora::AlohomoraTypeEnum) -> Result<Self::Out, ()> {
//         Ok(InferenceResponseOut::Inference(LLMResponse::from_enum(e)?))
//     }
// }

// #[tarpc::service]
// pub trait Database {
//     async fn store_prompt(prompt: String) -> bool;
// }
//
// #[derive(Clone, Copy)]
// pub struct DatabaseServer {}
