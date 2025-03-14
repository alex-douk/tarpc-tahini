use alohomora::bbox::BBox;
use alohomora::tarpc::{TahiniEnum, TahiniType};
use alohomora::{AlohomoraType, TahiniType};
use tarpc::serde::{Deserialize, Serialize};
use crate::policies::MarketingPolicy;

#[derive(Deserialize, Clone, Debug, TahiniType)]
pub struct MarketingData {
    pub email: String,
    pub prompt: BBox<String, MarketingPolicy>
}
