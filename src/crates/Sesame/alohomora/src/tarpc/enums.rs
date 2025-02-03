use std::any::Any;
use std::collections::HashMap;

use crate::bbox::BBox;
use crate::policy::AnyPolicy;

// TODO(babman): this enum, the one in AlohomoraType, and the one in bbox_render are all very similar
//               variants of each other with slightly different bounds and move/refs
//               we should unify them.
//               If we unify them, we could have a singular derive macro for AlohomoraType and the
//               further below TahiniType.
pub enum TahiniEnum {
    Value(Box<dyn Any>),
    BBox(BBox<Box<dyn Any>, AnyPolicy>),
    Vec(Vec<TahiniEnum>),
    Struct(HashMap<String, TahiniEnum>),
    Enum(u32, Box<TahiniEnum>),
}
pub enum DeboxedTahiniEnum {
    Value(Box<dyn Any>),
    BBox((Box<dyn Any>, AnyPolicy)),
    Vec(Vec<DeboxedTahiniEnum>),
    Struct(HashMap<String, DeboxedTahiniEnum>),
    Enum(u32, Box<DeboxedTahiniEnum>),
}

impl TahiniEnum {
    // BBox content not leaked: we consume the bbox here in pub(crate) function.
    pub(crate) fn debox(self) -> DeboxedTahiniEnum {
        match self {
            TahiniEnum::Value(value) => DeboxedTahiniEnum::Value(value),
            TahiniEnum::BBox(bbox) => DeboxedTahiniEnum::BBox(bbox.consume()),
            TahiniEnum::Vec(vec) => DeboxedTahiniEnum::Vec(
                vec.into_iter().map(TahiniEnum::debox).collect()
            ),
            TahiniEnum::Struct(map) => DeboxedTahiniEnum::Struct(
                map.into_iter()
                    .map(|(key, val)| (key, val.debox()))
                    .collect()
            ),
            TahiniEnum::Enum(id, v) => DeboxedTahiniEnum::Enum(id, Box::new(v.debox())),
        }
    }
    pub(crate) fn rebox(intermediate: DeboxedTahiniEnum) -> Self {
        match intermediate {
            DeboxedTahiniEnum::Value(value) => TahiniEnum::Value(value),
            DeboxedTahiniEnum::BBox((t, p)) => TahiniEnum::BBox(BBox::new(t, p)),
            DeboxedTahiniEnum::Vec(vec) => TahiniEnum::Vec(
                vec.into_iter().map(TahiniEnum::rebox).collect()
            ),
            DeboxedTahiniEnum::Struct(map) => TahiniEnum::Struct(
                map.into_iter()
                    .map(|(k, v)| (k, TahiniEnum::rebox(v)))
                    .collect()
            ),
            DeboxedTahiniEnum::Enum(id, v) => TahiniEnum::Enum(
                id,
                Box::new(TahiniEnum::rebox(*v)),
            ),
        }
    }
}