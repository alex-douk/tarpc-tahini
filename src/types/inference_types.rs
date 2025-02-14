use std::collections::HashMap;

use alohomora::tarpc::{TahiniEnum, TahiniType};
use alohomora::bbox::BBox;
use tarpc::serde::{Deserialize, Serialize};

use crate::policies::PromptPolicy;
use super::database_types::{DatabaseForm, DBUUID};

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
    fn to_enum(&self) -> TahiniEnum {
        let mut map = HashMap::new();
        map.insert("user", TahiniEnum::Value(Box::new(self.user.clone())));
        map.insert("prompt", <BBox<_, _> as TahiniType>::to_enum(&self.prompt));
        map.insert("nb_token", TahiniEnum::Value(Box::new(self.nb_token)));
        TahiniEnum::Struct("UserPrompt", map)
    }
}

impl TahiniType for LLMResponse {
    fn to_enum(&self) -> TahiniEnum {
        let mut map = HashMap::new();
        map.insert("infered_tokens", self.infered_tokens.to_enum());
        map.insert("db_uuid", self.db_uuid.to_enum()); 
        TahiniEnum::Struct("LLMResponse", map) 
    }
}
