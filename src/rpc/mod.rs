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

use crate::types::*;
// use tokio::sync::Mutex; // Tokio's async Mutex

#[macro_use]
pub mod tcp;

// pub mod serde;


#[service]
pub trait Inference {
    // #[tahini_unchecked]
    async fn inference(prompt: UserPromptClean) -> LLMResponseClean;
}


// #[derive(Deserialize, Serialize)]
// pub enum InferenceRequestOut {
//     Inference {
//         prompt: <UserPrompt as AlohomoraType>::Out
//     }
// }

// #[derive(Serialize)]
// pub enum InferenceResponseOut {
//     Inference(<LLMResponse as AlohomoraType>::Out)
// }


// impl AlohomoraType for InferenceRequest {
//     type Out = InferenceRequestOut;
//
//     fn to_enum(self) -> alohomora::AlohomoraTypeEnum {
//         match self {
//             InferenceRequest::Inference { prompt } => prompt.to_enum()
//         }
//     }
//
//     fn from_enum(e: alohomora::AlohomoraTypeEnum) -> Result<Self::Out, ()> {
//         Ok(InferenceRequestOut::Inference { prompt: UserPrompt::from_enum(e)?})
//     }
// }
