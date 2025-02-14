use std::any::Any;
use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTupleVariant};
use tokio_util::bytes::{Buf, BytesMut};

use crate::bbox::BBox;
// use crate::policy::AnyPolicy;
// use crate::policy::{Policy, RefTahiniPolicy};
use crate::policy::TahiniPolicy;
use crate::pure::PrivacyPureRegion;
use crate::tarpc::hacky::ExamplePolicy;
use crate::tarpc::traits::TahiniType;

pub enum TahiniEnum {
    Value(Box<dyn erased_serde::Serialize>),
    BBox(BBox<Box<dyn erased_serde::Serialize>, TahiniPolicy>),
    Vec(Vec<TahiniEnum>),
    Struct(&'static str, HashMap<&'static str, TahiniEnum>),
    // Add potential enum variant if tarpc decides to generate a struct based on parameters?
    // EnumStruct(String, u32, String, Box<dyn TahiniType2>),
    EnumNewType(&'static str, u32, &'static str, Box<TahiniEnum>),
    EnumTuple(&'static str, u32, &'static str, Vec<TahiniEnum>),
}

//Only messy part here is to have two different wrappers operating the same function
//The real reason is that it gives explicit typing to our structs.
//It's open to debate whether we want to have two layers :shrug:

//See the type, but no touchy to what's inside!
pub struct TahiniSafeWrapper<T: TahiniType>(pub(crate) T);

//Private struct for specific TahiniEnum leaves.
struct PrivEnumWrapper<'a>(&'a TahiniEnum);

impl<'a> serde::Serialize for PrivEnumWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            TahiniEnum::Value(val) => erased_serde::serialize(&val, serializer),
            TahiniEnum::BBox(bbox) => {
                let t = bbox.data();
                let p = bbox.policy();
                // let mut bbox_ser = erased_serde::serialize(value, serializer)
                let mut bbox_ser = serializer.serialize_struct("BBox", 2)?;
                bbox_ser.serialize_field("fb", &(*t))?;
                bbox_ser.serialize_field("p", &p)?;
                bbox_ser.end()
            }
            TahiniEnum::Vec(vec) => {
                let mut vec_ser = serializer.serialize_seq(Some(vec.len()))?;
                for e in vec.iter() {
                    vec_ser.serialize_element(&PrivEnumWrapper(e))?;
                }
                vec_ser.end()
            }
            TahiniEnum::Struct(struct_name, map) => {
                let mut struct_ser = serializer.serialize_struct(struct_name, map.len())?;
                for (k, v) in map.iter() {
                    struct_ser.serialize_field(k, &PrivEnumWrapper(v))?;
                }
                struct_ser.end()
            }
            _ => panic!(""),
        }
    }
}

impl<T: TahiniType + Sized> serde::Serialize for TahiniSafeWrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0.to_enum() {
            //Required because we can't infer type from the val here
            TahiniEnum::Value(val) => erased_serde::serialize(&val, serializer),
            TahiniEnum::BBox(bbox) => {
                let t = bbox.data();
                let p = bbox.policy();
                // let mut bbox_ser = erased_serde::serialize(value, serializer)
                let mut bbox_ser = serializer.serialize_struct("BBox", 2)?;
                bbox_ser.serialize_field("fb", &(*t))?;
                bbox_ser.serialize_field("p", &p)?;
                bbox_ser.end()
            }
            TahiniEnum::Vec(vec) => {
                let mut vec_ser = serializer.serialize_seq(Some(vec.len()))?;
                for e in vec.iter() {
                    vec_ser.serialize_element(&PrivEnumWrapper(e))?;
                }
                vec_ser.end()
            }
            TahiniEnum::Struct(struct_name, map) => {
                let mut struct_ser = serializer.serialize_struct(struct_name, map.len())?;
                for (k, v) in map.iter() {
                    struct_ser.serialize_field(k, &PrivEnumWrapper(v))?;
                }
                struct_ser.end()
            }
            TahiniEnum::EnumNewType(enum_name, variant_nb, variant_name, val) => serializer
                .serialize_newtype_variant(
                    enum_name,
                    variant_nb,
                    variant_name,
                    &PrivEnumWrapper(&(*val)),
                ),
            TahiniEnum::EnumTuple(enum_name, variant_nb, variant_name, val) => {
                let mut tuple_ser = serializer.serialize_tuple_variant(
                    enum_name,
                    variant_nb,
                    variant_name,
                    val.len(),
                )?;
                for e in val.iter() {
                    tuple_ser.serialize_field(&PrivEnumWrapper(e))?;
                }
                tuple_ser.end()
            }
        }
    }
}
