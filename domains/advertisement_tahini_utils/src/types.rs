use std::collections::HashMap;

use tarpc::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketingData {
    pub username: Option<String>,
    pub prompt: String,
    pub third_party_ad_vendors_allowed: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ad {
    pub ad: String
}
