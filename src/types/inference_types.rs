use std::collections::HashMap;

use alohomora::tarpc::{TahiniEnum, TahiniType};
use alohomora::bbox::BBox;
use alohomora::AlohomoraType;
use tarpc::serde::{Deserialize, Serialize};

use crate::policies::PromptPolicy;
use super::database_types::{DatabaseSubmit, DBUUID};

//#[derive(TahiniType)]
#[derive(Deserialize, Clone, Debug)]
pub struct UserPrompt {
    pub user: String,
    pub prompt: BBox<String, PromptPolicy>,
    pub nb_token: u32,
}

//#[derive(TahiniType)]
#[derive(Deserialize, Clone, Debug)]
pub struct LLMResponse {
    pub infered_tokens: BBox<String, PromptPolicy>,
    // Why do we attach the same policy? 
    // Because same level of confidentality. Debatable
    pub db_uuid: BBox<u32, PromptPolicy>
}

//Auto-generated
impl TahiniType for UserPrompt {
    fn to_tahini_enum(&self) -> TahiniEnum {
        let mut map = HashMap::new();
        map.insert("user", self.user.to_tahini_enum());
        map.insert("prompt", self.prompt.to_tahini_enum());
        map.insert("nb_token", self.nb_token.to_tahini_enum());
        TahiniEnum::Struct("UserPrompt", map)
    }
}

impl TahiniType for LLMResponse {
    fn to_tahini_enum(&self) -> TahiniEnum {
        let mut map = HashMap::new();
        map.insert("infered_tokens", self.infered_tokens.to_tahini_enum());
        map.insert("db_uuid", self.db_uuid.to_tahini_enum());
        TahiniEnum::Struct("LLMResponse", map) 
    }
}
