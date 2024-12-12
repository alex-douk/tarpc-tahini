use crate::policy::Policy;
use crate::bbox::BBox;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::Deserializer;

impl<'de, T: DeserializeOwned, P: Policy + Default> Deserialize<'de> for BBox<T, P>{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
                let res = T::deserialize(deserializer)?;
                Ok(BBox::new(res, P::default()))
    }
}

impl<T: Serialize, P: Policy> Serialize for BBox<T, P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str("[REDACTED]")
    }

}


pub mod json {
    /// JSON codec using [serde_json](https://docs.rs/serde_json) crate.
    ///
use crate::policy::Policy;
use crate::bbox::BBox;
use crate::AlohomoraType;
use crate::AlohomoraTypeEnum;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use tokio_serde::{Deserializer, Serializer};
use std::marker::PhantomData;
use std::pin::Pin;
use educe::Educe;
use tokio_util::bytes::{BytesMut, Bytes, Buf};
use std::ptr::read as ptr_read;
use std::ptr::copy_nonoverlapping as ptr_copy;

    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    #[derive(Educe)]
    #[educe(Debug, Default)]
    //Could add an endpoint field here to know if we are codec'ing into a checked or unchecked
    //gateway.
    pub struct Json<Item, SinkItem> {
        #[educe(Debug(ignore))]
        ghost: PhantomData<(Item, SinkItem)>,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub type SymmetricalJson<T> = Json<T, T>;

    impl<Item, SinkItem> Deserializer<Item> for Json<Item, SinkItem>
    where
        for<'a> Item: Deserialize<'a> + AlohomoraType,
    {
        type Error = serde_json::Error;

        fn deserialize(self: Pin<&mut Self>, src: &BytesMut) -> Result<Item, Self::Error> {
            println!("Deserializing stuff");
            let byt = src.clone();
            // println!("Bytes are : {}", String::from_utf8(src.to_vec()).unwrap());
            let res: Result<Item, _> = serde_json::from_reader(std::io::Cursor::new(src).reader());
            if res.is_err() {
                println!("Failed at deserializing");
                println!("Error is : {:?}", res.err());
            }
            // res
            serde_json::from_reader(std::io::Cursor::new(byt).reader())
        }
    }

    ///Serialization is hijacked from regular Serde's ser for custom BBox logic
    impl<Item, SinkItem> Serializer<SinkItem> for Json<Item, SinkItem>
    where
        SinkItem: AlohomoraType + Clone + std::fmt::Debug,
        SinkItem::Out: Serialize
    {
        type Error = serde_json::Error;

        fn serialize(self: Pin<&mut Self>, item: &SinkItem) -> Result<Bytes, Self::Error> {
            println!("Serializing stuff");
                // let copy = ptr_copy(item, copy, 1);
                println!("{:?}", item.clone());
                let unwrapped = item.clone().to_enum();
                let unwrapped = unwrapped.remove_bboxes();
                let res : SinkItem::Out = <SinkItem as AlohomoraType>::from_enum(unwrapped).expect("Serialization failed when going from enum to Out type");
                let res = serde_json::to_vec(&res).map(Into::into);
                println!("Serialized {:?}", res);
                res
        }
    }

}
