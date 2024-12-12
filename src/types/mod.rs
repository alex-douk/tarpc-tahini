use tarpc::serde::{Deserialize, Serialize};
use alohomora::{policy::NoPolicy, AlohomoraType, bbox::BBox};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[derive(AlohomoraType)]
#[alohomora_out_type(to_derive=[Serialize, Debug, Deserialize])]
pub struct UserPrompt {
    pub user: String,
    pub prompt: BBox<String, NoPolicy>,
    pub nb_token: u32
}

#[derive(Deserialize, Serialize, AlohomoraType, Debug, Clone)]
#[alohomora_out_type(to_derive=[Serialize, Debug, Deserialize])]
pub struct LLMResponse {
    pub infered_tokens: BBox<String, NoPolicy>
}
