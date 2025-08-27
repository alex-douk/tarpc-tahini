use crate::types::{Ad, MarketingData};

#[tarpc::service]
pub trait Advertisement {
    async fn auction_bidding(
        prompt: MarketingData
    ) -> Ad;
}
