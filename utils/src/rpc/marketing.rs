use crate::policies::MarketingPolicy;
use crate::types::marketing_types::{Ad, MarketingData};
use alohomora::bbox::BBox;
use alohomora::tahini_service;
use alohomora::tarpc::{TahiniType, client::TahiniStub};

#[tahini_service]
pub trait Advertisement {
    async fn auction_bidding(prompt: BBox<MarketingData, MarketingPolicy>) -> Ad;
}
