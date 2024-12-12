use serde::Serialize;

pub(crate) mod json {
    use serde::{Deserialize, Serialize};
    use std::{marker::PhantomData, pin::Pin};
    use tarpc::{ClientMessage, Request, Response};
    use tokio_serde::{Deserializer, Serializer};
    use crate::rpc::{InferenceRequest, InferenceResponse};
    // use super::*;
    use tokio_util::bytes::{Buf, Bytes, BytesMut};

    use crate::rpc::DummyBox;

    static SECRET: Bytes = Bytes::from_static(&[1, 2, 3, 4, 5, 6, 7]);

    use educe::Educe;

    trait PrivateSerialize: Serialize {
        fn private_serialize(&self) -> Result<Bytes, serde_json::Error> {
            serde_json::to_vec(&self).map(Into::into)
        }
    }

    // trait NegTrait{}
    //
    // impl<T, P> NegTrait for crate::rpc::DummyBox<T, P>{}
    //
    // impl !NegTrait for T{}


    /// JSON codec using [serde_json](https://docs.rs/serde_json) crate.
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
        for<'a> Item: Deserialize<'a>,
    {
        type Error = serde_json::Error;

        fn deserialize(self: Pin<&mut Self>, src: &BytesMut) -> Result<Item, Self::Error> {
            println!("Deserializing stuff");
            let byt = src.clone();
            println!("Bytes are : {}", String::from_utf8(src.to_vec()).unwrap());
            let res: Result<Item, _> = serde_json::from_reader(std::io::Cursor::new(src).reader());
            if res.is_err() {
                println!("Failed at deserializing");
                println!("Error is : {:?}", res.err());
            }
            serde_json::from_reader(std::io::Cursor::new(byt).reader())
        }
    }

    ///Serialization is hijacked from regular Serde's ser for custom BBox logic
    impl<Item, SinkItem> Serializer<SinkItem> for Json<Item, SinkItem>
    where
        SinkItem: PrivateSerialize,
    {
        type Error = serde_json::Error;

        fn serialize(self: Pin<&mut Self>, item: &SinkItem) -> Result<Bytes, Self::Error> {
            println!("Serializing stuff");
            // let res = serde_json::to_vec(item).map(Into::into);
            let res = item.private_serialize();
            if res.is_err() {
                println!("Failed at serializing stuff");
            } else {
                println!("Res is {:?}", res);
            }
            res
        }
    }

    impl<T: Serialize, P> Serialize for crate::rpc::DummyBox<T, P> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_none()
            // self.get_val().serialize(serializer)
        }
    }

    ///Used in case of first-level boxes for tarpc serialization
    impl<T, P> PrivateSerialize for crate::rpc::DummyBox<T, P>
    where
        T: Serialize,
        P: Serialize,
    {
        fn private_serialize(&self) -> Result<Bytes, serde_json::Error> {
            println!("Serializing BBox");
            let mut writer = Vec::with_capacity(128);
            let mut ser = serde_json::ser::Serializer::new(&mut writer);
            let res = self.inner.serialize(&mut ser);
            match res {
                Ok(_) => Ok(writer.into()),
                Err(e) => Err(e),
            }
        }
    }

    ///Trait implementation for most common types
    macro_rules! safe_def_private_ser {
        ($T: ty) => {
            impl PrivateSerialize for $T {
                fn private_serialize(&self) -> Result<Bytes, serde_json::Error> {
                    serde_json::to_vec(&self).map(Into::into)
                }
            }
        };
    }

    safe_def_private_ser!(u8);
    safe_def_private_ser!(u16);
    safe_def_private_ser!(u32);
    safe_def_private_ser!(i8);
    safe_def_private_ser!(i16);
    safe_def_private_ser!(i32);
    safe_def_private_ser!(str);
    safe_def_private_ser!(String);
    safe_def_private_ser!(());

    ///Trait implementation for wrapper types
    impl<T> PrivateSerialize for Option<T>
    where
        T: PrivateSerialize,
    {
        fn private_serialize(&self) -> Result<Bytes, serde_json::Error> {
            match self {
                Some(t) => t.private_serialize(),
                None => serde_json::to_vec(&self).map(Into::into)
            }
            // serde_json::to_vec(&self).map(Into::into)
        }
    }

    impl<T, E> PrivateSerialize for Result<T, E>
    where
        T: PrivateSerialize,
        E: Serialize,
    {
        fn private_serialize(&self) -> Result<Bytes, serde_json::Error> {
            serde_json::to_vec(&self).map(Into::into)
        }
    }

    ///Trait implementation for tarpc response wrapper
    impl<T: Serialize> PrivateSerialize for Response<T> {
        fn private_serialize(&self) -> Result<Bytes, serde_json::Error> {
            serde_json::to_vec(&self).map(Into::into)
        }
    }

    //Wrapper type around our message. Need to ensure custom logic is used
    ///Trait implementation for tarpc request type
    impl<T: Serialize + PrivateSerialize> PrivateSerialize for Request<T> {
        fn private_serialize(&self) -> Result<Bytes, serde_json::Error> {
            self.message.private_serialize()
            // serde_json::to_vec(&self.message).map(Into::into)
        }
    }

    ///Trait implementation for top-level wrapper type.
    ///Everything stems from here on the client side
    impl<T: Serialize + PrivateSerialize> PrivateSerialize for ClientMessage<T> {
        fn private_serialize(&self) -> Result<Bytes, serde_json::Error> {
            match &self {
                //Only apply custom logic if we modified the contents
                ClientMessage::Request(e) => {
                    println!("Private serialize for ClientMessage");
                    let tmp = e.private_serialize();
                    //1. Reconstruct Request<T> in private_serialize
                    //1.1 Serialize the inner layers
                    //1.2 Deserialize the layers...
                    //2. Reconstruct ClientMessage in this function
                    //3. the use serde_json::to_vec to serialize one last time.
                    //
                    //What does this ensure? That we go through all layers of nesting with our
                    //custom logic once. Points 1 and 2 also ensure that we reconstruct all layers
                    //of wrapping types.
                    //Point 3 ensures the data is properly formatted for the network. 
                    //Deserialize impl for DummyBox ensures we wrap it adequately
                    

                    // let val = ClientMessage::Request(())
                    // let mut writer = Vec::with_capacity(128);
                    // let mut ser = serde_json::ser::Serializer::new(&mut writer);
                    // //How to serialize ClientMessage::Request(Request<UserPrompt>)??
                    // //We can not force the trait bound on T sadly
                    // //I would really like to call PrivateSerialize::private_serialize here
                    // serde_json::to_vec(&self).map(Into::into)
                    // //
                    // // let res = Ok(());
                    // //
                    // // // let res = self.inner.serialize(&mut ser);
                    // // match res {
                    // //     Ok(_) => Ok(writer.into()),
                    // //     Err(e) => Err(e),
                    // // }

                    e.private_serialize()
                }
                _ => serde_json::to_vec(&self).map(Into::into),
            }
        }
    }
}
