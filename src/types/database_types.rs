use std::collections::HashMap;

use alohomora::tarpc::{TahiniEnum, TahiniType};
use alohomora::bbox::BBox;
use tarpc::serde::{Deserialize, Serialize};

use crate::policies::PromptPolicy;

//#[derive(TahiniType)]
#[derive(Deserialize, Clone)]
pub struct DatabaseSubmit {
    pub user: String,
    pub full_prompt: BBox<String, PromptPolicy>
}

//TODO(douk): Create Database record type that optionally contains the conversation

pub type DBUUID = BBox<u32, PromptPolicy>;

pub type DatabaseRecord = DatabaseSubmit;

impl TahiniType for DatabaseSubmit {
    fn to_tahini_enum(&self) -> TahiniEnum {
        let mut map = HashMap::new();
        map.insert("user", TahiniEnum::Value(Box::new(self.user.clone())));
        map.insert("full_prompt", <BBox<_, _> as TahiniType>::to_tahini_enum(&self.full_prompt));
        TahiniEnum::Struct("DatabaseForm", map)
    }
}


//No need for impl TahiniType for DBUUID
