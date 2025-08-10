use crate::policies::MarketingPolicy;
use crate::types::{Ad, MarketingData};
use alohomora::bbox::BBox;
use tahini_tarpc::traits::Fromable;
use tahini_tarpc::{client::TahiniStub, TahiniType};
use tahini_tarpc::{allow_client_transform, tahini_service};

#[tahini_service(domain=foreign)]
pub trait Advertisement {
    #[allow_client_transform]
    async fn auction_bidding(
        prompt: BBox<MarketingData, MarketingPolicy>,
    ) -> Ad;
}
