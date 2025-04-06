use crate::policies::MarketingPolicy;
use alohomora::bbox::BBox;
use alohomora::tarpc::TahiniType;
use alohomora::TahiniType;
use alohomora::tarpc::{TahiniSerialize, TahiniDeserialize};

#[derive(TahiniSerialize, TahiniDeserialize, Clone, Debug)]
pub struct MarketingData {
    pub username: Option<String>,
    pub prompt: String,
}

#[derive(TahiniDeserialize, Clone, Debug, TahiniType)]
pub struct Ad {
    pub ad: BBox<String, MarketingPolicy>,
}
