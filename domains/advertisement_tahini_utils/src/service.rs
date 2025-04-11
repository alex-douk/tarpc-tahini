use crate::policies::MarketingPolicy;
use alohomora::tarpc::traits::Fromable;
use crate::types::{Ad, MarketingData};
use alohomora::bbox::BBox;
use alohomora::tahini_service;
use alohomora::tarpc::{TahiniType, client::TahiniStub};

#[tahini_service(domain=foreign)]
pub trait Advertisement {
    async fn auction_bidding(prompt: BBox<MarketingData, MarketingPolicy>) -> Ad;
}
