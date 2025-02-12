use std::any::{Any, TypeId};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::bbox::BBox;
use crate::policy::{AnyPolicy, Policy, TahiniPolicy};
use crate::tarpc::enums::{TahiniEnum, TahiniSafeWrapper};
use crate::pure::PrivacyPureRegion;
use crate::tarpc::hacky::ExamplePolicy;

pub trait TahiniType :  Send {
    fn to_enum(&self) -> TahiniEnum;
}

impl<T: Serialize + DeserializeOwned + Clone + Send + 'static, P: Policy + Clone + Serialize + DeserializeOwned + 'static> TahiniType for BBox<T, P> {
    fn to_enum(&self) -> TahiniEnum {
        let t = self.data().clone();
        let p = self.policy();
        let anybox = Box::new(t) as Box<dyn erased_serde::Serialize>;
        TahiniEnum::BBox(BBox::new(anybox, TahiniPolicy::new(p.clone())))
    }
}
