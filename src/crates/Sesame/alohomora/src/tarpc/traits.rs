use std::any::{Any, TypeId};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::bbox::BBox;
use crate::policy::{AnyPolicy, Policy};
use crate::tarpc::enums::{DeboxedTahiniEnum, TahiniEnum};

// Serialization: first Sesame calls to_enum, then it removes bboxes in the enum
// using debox(), and then calls to_intermediate() to get Intermediate type which it can
// serialize using serde.
// Deserialization: first deserializes the bytes to Intermediate using serde,
// then it calls from_intermediate to turn it into an unboxed enum,
// then it calls rebox() followed by from_enum().
// TODO(babman): to_intermediate() and from_intermediate() as well as Self::Intermediate's impls of
//               serialize and deserialize are dangerous.
//               use Corinn's lints to ensure that custom implementations are disallowed except
//               using derive macro.
pub trait TahiniType {
    // Danger: Serialize and DeserializeOwned better not leak data!
    type Intermediate: Serialize + DeserializeOwned;
    // No danger in this function: it keeps bboxes boxed.
    fn to_enum(self) -> TahiniEnum;
    // Danger!
    fn to_intermediate(e: DeboxedTahiniEnum) -> Self::Intermediate;
    fn from_intermediate(e: Self::Intermediate) -> DeboxedTahiniEnum;
    // No danger.
    fn from_enum(e: TahiniEnum) -> Self;
    // Only for enums.
    fn name(e: &Self::Intermediate) -> &'static str {
        ""
    }
}

impl<T: Serialize + DeserializeOwned + 'static, P: Policy + Clone + Serialize + DeserializeOwned + 'static> TahiniType for BBox<T, P> {
    type Intermediate = (T, P);
    fn to_enum(self) -> TahiniEnum {
        TahiniEnum::BBox(self.into_any())
    }
    fn to_intermediate(e: DeboxedTahiniEnum) -> Self::Intermediate {
        match e {
            DeboxedTahiniEnum::BBox((t, p)) => (
                *t.downcast().unwrap(), 
                p.specialize().unwrap()
            ),
            _ => panic!("unreachable"),
        }
    }
    fn from_intermediate(e: Self::Intermediate) -> DeboxedTahiniEnum {
        DeboxedTahiniEnum::BBox((Box::new(e.0), AnyPolicy::new(e.1)))
    }
    fn from_enum(e: TahiniEnum) -> Self {
        match e {
            TahiniEnum::BBox(bbox) => bbox.specialize().unwrap(),
            _ => panic!("bad from_enum in TahiniType"),
        }
    }
}

// For Request enums to have names.
pub trait NamedTahiniType: TahiniType {
    fn enum_name(v: &<Self as TahiniType>::Intermediate) -> &'static str;
}

// Serialize/Deserialize.
pub(crate) fn serialize_tahini_type<A: TahiniType>(e: A) -> A::Intermediate {
    A::to_intermediate(e.to_enum().debox())
}
pub(crate) fn deserialize_tahini_type<A: TahiniType>(e: A::Intermediate) -> A {
    A::from_enum(TahiniEnum::rebox(A::from_intermediate(e)))
}