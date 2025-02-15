use erased_serde::serialize_trait_object;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::any::{Any, TypeId};

use crate::bbox::BBox;
use crate::policy::{AnyPolicy, Policy, TahiniPolicy};
use crate::pure::PrivacyPureRegion;
use crate::tarpc::enums::{TahiniEnum, TahiniSafeWrapper};
use crate::tarpc::hacky::ExamplePolicy;

pub trait TahiniType: Send {
    fn to_tahini_enum(&self) -> TahiniEnum;
}

pub trait TahiniError: erased_serde::Serialize + std::error::Error {}

serialize_trait_object!(TahiniError);

impl<
        T: Serialize + DeserializeOwned + Clone + Send + 'static,
        P: Policy + Clone + Serialize + DeserializeOwned + 'static,
    > TahiniType for BBox<T, P>
{
    fn to_tahini_enum(&self) -> TahiniEnum {
        let t = self.data().clone();
        let p = self.policy();
        let anybox = Box::new(t) as Box<dyn erased_serde::Serialize>;
        TahiniEnum::BBox(BBox::new(anybox, TahiniPolicy::new(p.clone())))
    }
}

impl<T: TahiniType + Clone + 'static> TahiniType for Option<T> {
    fn to_tahini_enum(&self) -> TahiniEnum {
        TahiniEnum::Option(self.as_ref().map(|x| Box::new(x.to_tahini_enum())))
        // match &self {
        //
        //     None => TahiniEnum::Option(None::<T>),
        //     //Works for primitive types but not for BBox's
        //     Some(x) => TahiniEnum::Value(Box::new(Some(x.clone())))
        // }
    }
}

impl<T: TahiniType + Clone + 'static, E: TahiniError + Send + 'static + Clone> TahiniType
    for Result<T, E>
{
    fn to_tahini_enum(&self) -> TahiniEnum {
        TahiniEnum::Result(
            self.as_ref()
                .map(|x| Box::new(x.to_tahini_enum()))
                .map_err(|x| Box::new(x.clone()) as Box<dyn TahiniError>),
        )
    }
}

macro_rules! impl_tahini_trait_prim {
    ($ty: ty)=> {
        impl TahiniType for $ty {
            fn to_tahini_enum(&self) -> TahiniEnum {
                TahiniEnum::Value(Box::new(self.clone()))
            }
        }
    };
}

impl_tahini_trait_prim!(u8);
impl_tahini_trait_prim!(u16);
impl_tahini_trait_prim!(u32);
impl_tahini_trait_prim!(i8);
impl_tahini_trait_prim!(i16);
impl_tahini_trait_prim!(i32);
impl_tahini_trait_prim!(usize);
impl_tahini_trait_prim!(String);
