use crate::policies::MarketingPolicy;
use alohomora::bbox::BBox;
use alohomora::tarpc::{TahiniEnum, TahiniType};
use alohomora::{AlohomoraType, TahiniType};
use tarpc::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketingData {
    pub username: String,
    pub prompt: String,
}

#[derive(Deserialize, Clone, Debug, TahiniType)]
pub struct Ad {
    pub ad: BBox<String, MarketingPolicy>,
}
