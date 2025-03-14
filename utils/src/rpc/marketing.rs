use alohomora::tarpc::{
    client::TahiniStub,
    TahiniType,
};
use alohomora::tahini_service;
use crate::types::marketing_types::MarketingData;

#[tahini_service]
pub trait Advertisement {
    async fn email(prompt: MarketingData) -> bool;
}
