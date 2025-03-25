pub mod database_types;
pub mod inference_types;
pub mod marketing_types;

use alohomora::tarpc::traits::TahiniError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyError;

impl std::fmt::Display for PolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed policy check")
    }
}

impl std::error::Error for PolicyError {}
impl TahiniError for PolicyError {}
