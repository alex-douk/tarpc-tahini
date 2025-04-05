#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use crate::policy::ExamplePolicy;
use alohomora::bbox::BBox;
use alohomora::pcr::{PrivacyCriticalRegion, Signature};
use alohomora::tarpc::client::new as new2;
use alohomora::tarpc::enums::TahiniSafeWrapper;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::client::RpcError;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Bincode;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;
mod policy {
    use tarpc::serde::{Deserialize, Serialize};
    use alohomora::context::UnprotectedContext;
    use alohomora::policy::{AnyPolicy, Policy, Reason};
    pub struct ExamplePolicy {
        pub field: u32,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ExamplePolicy {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ExamplePolicy",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "field",
                    &self.field,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ExamplePolicy {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "field" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"field" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ExamplePolicy>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ExamplePolicy;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ExamplePolicy",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ExamplePolicy with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ExamplePolicy { field: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("field"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("field")?
                            }
                        };
                        _serde::__private::Ok(ExamplePolicy { field: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["field"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ExamplePolicy",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ExamplePolicy>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for ExamplePolicy {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ExamplePolicy",
                "field",
                &&self.field,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ExamplePolicy {
        #[inline]
        fn clone(&self) -> ExamplePolicy {
            ExamplePolicy {
                field: ::core::clone::Clone::clone(&self.field),
            }
        }
    }
    impl Policy for ExamplePolicy {
        fn name(&self) -> String {
            String::from("ExamplePolicy")
        }
        fn check(&self, _context: &UnprotectedContext, _reason: Reason<'_>) -> bool {
            true
        }
        fn join(&self, _other: AnyPolicy) -> Result<AnyPolicy, ()> {
            ::core::panicking::panic("not yet implemented")
        }
        fn join_logic(&self, other: Self) -> Result<Self, ()> {
            Ok(other)
        }
    }
}
mod service {
    use std::collections::HashMap;
    use alohomora::bbox::BBox as PCon;
    use alohomora::pure::PrivacyPureRegion as PPR;
    use alohomora::tarpc::client::{
        TahiniChannel, TahiniNewClient, TahiniRequestDispatch, TahiniStub,
    };
    use alohomora::tarpc::server::TahiniServe;
    use alohomora::tarpc::{
        enums::{TahiniEnum, TahiniSafeWrapper, TahiniVariantsEnum},
        traits::{TahiniError, TahiniType},
    };
    use alohomora::AlohomoraType;
    use tarpc::serde::{Deserialize, Serialize};
    use tarpc::client::{Config, RpcError};
    use tarpc::server::Serve;
    use tarpc::{ClientMessage, Response, Transport};
    use alohomora::{TahiniType, tahini_service};
    use crate::policy::ExamplePolicy;
    pub struct InnerStruct {
        pub a: u16,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InnerStruct {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "InnerStruct",
                "a",
                &&self.a,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InnerStruct {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "a" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"a" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<InnerStruct>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InnerStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct InnerStruct",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            u16,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct InnerStruct with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(InnerStruct { a: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u16>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("a")?
                            }
                        };
                        _serde::__private::Ok(InnerStruct { a: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["a"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InnerStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<InnerStruct>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for InnerStruct {
        #[inline]
        fn clone(&self) -> InnerStruct {
            InnerStruct {
                a: ::core::clone::Clone::clone(&self.a),
            }
        }
    }
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::alohomora::tarpc::TahiniType for InnerStruct {
        fn to_tahini_enum(&self) -> ::alohomora::tarpc::TahiniEnum {
            let mut map: ::std::collections::HashMap<
                &'static str,
                ::alohomora::tarpc::TahiniEnum,
            > = ::std::collections::HashMap::from([
                ("a", <u16 as TahiniType>::to_tahini_enum(&self.a)),
            ]);
            ::alohomora::tarpc::TahiniEnum::Struct("InnerStruct", map)
        }
        fn tahini_policy_check(
            &self,
            members_fmt: &String,
            context: &::alohomora::context::UnprotectedContext,
            reason: &::alohomora::policy::Reason,
        ) -> bool {
            let mut policy_vec = Vec::new();
            policy_vec.push(self.a.tahini_policy_check(members_fmt, context, reason));
            policy_vec.iter().all(|x: &bool| *x)
        }
    }
    pub struct MyError {}
    #[automatically_derived]
    impl ::core::fmt::Debug for MyError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "MyError")
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MyError {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MyError>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MyError;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MyError",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::__private::Ok(MyError {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        _serde::__private::Ok(MyError {})
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MyError",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MyError>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MyError {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "MyError",
                    false as usize,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for MyError {
        #[inline]
        fn clone(&self) -> MyError {
            MyError {}
        }
    }
    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("Generic error"))
        }
    }
    impl std::error::Error for MyError {}
    impl TahiniError for MyError {}
    pub struct MyType {
        pub a: i32,
        pub b: PCon<String, ExamplePolicy>,
        pub c: Result<i32, MyError>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MyType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "MyType",
                "a",
                &self.a,
                "b",
                &self.b,
                "c",
                &&self.c,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MyType {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "a" => _serde::__private::Ok(__Field::__field0),
                            "b" => _serde::__private::Ok(__Field::__field1),
                            "c" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"a" => _serde::__private::Ok(__Field::__field0),
                            b"b" => _serde::__private::Ok(__Field::__field1),
                            b"c" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MyType>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MyType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MyType",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MyType with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            PCon<String, ExamplePolicy>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct MyType with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Result<i32, MyError>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct MyType with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(MyType {
                            a: __field0,
                            b: __field1,
                            c: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            PCon<String, ExamplePolicy>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Result<i32, MyError>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            PCon<String, ExamplePolicy>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("c"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Result<i32, MyError>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("a")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("b")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("c")?
                            }
                        };
                        _serde::__private::Ok(MyType {
                            a: __field0,
                            b: __field1,
                            c: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["a", "b", "c"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MyType",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MyType>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for MyType {
        #[inline]
        fn clone(&self) -> MyType {
            MyType {
                a: ::core::clone::Clone::clone(&self.a),
                b: ::core::clone::Clone::clone(&self.b),
                c: ::core::clone::Clone::clone(&self.c),
            }
        }
    }
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::alohomora::tarpc::TahiniType for MyType {
        fn to_tahini_enum(&self) -> ::alohomora::tarpc::TahiniEnum {
            let mut map: ::std::collections::HashMap<
                &'static str,
                ::alohomora::tarpc::TahiniEnum,
            > = ::std::collections::HashMap::from([
                ("a", <i32 as TahiniType>::to_tahini_enum(&self.a)),
                (
                    "b",
                    <PCon<String, ExamplePolicy> as TahiniType>::to_tahini_enum(&self.b),
                ),
                ("c", <Result<i32, MyError> as TahiniType>::to_tahini_enum(&self.c)),
            ]);
            ::alohomora::tarpc::TahiniEnum::Struct("MyType", map)
        }
        fn tahini_policy_check(
            &self,
            members_fmt: &String,
            context: &::alohomora::context::UnprotectedContext,
            reason: &::alohomora::policy::Reason,
        ) -> bool {
            let mut policy_vec = Vec::new();
            policy_vec.push(self.a.tahini_policy_check(members_fmt, context, reason));
            policy_vec.push(self.b.tahini_policy_check(members_fmt, context, reason));
            policy_vec.push(self.c.tahini_policy_check(members_fmt, context, reason));
            policy_vec.iter().all(|x: &bool| *x)
        }
    }
    pub trait SimpleService: ::core::marker::Sized + Clone {
        async fn increment(
            self,
            context: ::tarpc::context::Context,
            x: PCon<i32, ExamplePolicy>,
        ) -> PCon<String, ExamplePolicy>;
        /// Returns a serving function to use with
        /// [InFlightRequest::execute](::tarpc::server::InFlightRequest::execute).
        fn serve(self) -> ServeSimpleService<Self> {
            ServeSimpleService {
                service: self,
            }
        }
    }
    /// A serving function to use with [::tarpc::server::InFlightRequest::execute].
    pub struct ServeSimpleService<S> {
        service: S,
    }
    #[automatically_derived]
    impl<S: ::core::clone::Clone> ::core::clone::Clone for ServeSimpleService<S> {
        #[inline]
        fn clone(&self) -> ServeSimpleService<S> {
            ServeSimpleService {
                service: ::core::clone::Clone::clone(&self.service),
            }
        }
    }
    impl<S> ::alohomora::tarpc::server::TahiniServe for ServeSimpleService<S>
    where
        S: SimpleService + Clone,
    {
        type Req = SimpleServiceTahiniRequest;
        type Resp = SimpleServiceTahiniResponse;
        async fn serve(
            self,
            ctx: ::tarpc::context::Context,
            req: SimpleServiceTahiniRequest,
        ) -> ::core::result::Result<SimpleServiceTahiniResponse, ::tarpc::ServerError> {
            match req {
                SimpleServiceTahiniRequest::Increment { x } => {
                    ::core::result::Result::Ok(
                        SimpleServiceTahiniResponse::Increment(
                            SimpleService::increment(self.service, ctx, x).await,
                        ),
                    )
                }
            }
        }
    }
    /// The request sent over the wire from the client to the server.
    #[allow(missing_docs)]
    pub enum SimpleServiceTahiniRequest {
        Increment { x: PCon<i32, ExamplePolicy> },
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SimpleServiceTahiniRequest {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Increment" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Increment" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SimpleServiceTahiniRequest>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SimpleServiceTahiniRequest;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum SimpleServiceTahiniRequest",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "x" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"x" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<
                                        SimpleServiceTahiniRequest,
                                    >,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = SimpleServiceTahiniRequest;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant SimpleServiceTahiniRequest::Increment",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            PCon<i32, ExamplePolicy>,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant SimpleServiceTahiniRequest::Increment with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(SimpleServiceTahiniRequest::Increment {
                                            x: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<
                                            PCon<i32, ExamplePolicy>,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("x"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<
                                                            PCon<i32, ExamplePolicy>,
                                                        >(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("x")?
                                            }
                                        };
                                        _serde::__private::Ok(SimpleServiceTahiniRequest::Increment {
                                            x: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["x"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<
                                            SimpleServiceTahiniRequest,
                                        >,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Increment"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "SimpleServiceTahiniRequest",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            SimpleServiceTahiniRequest,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::alohomora::tarpc::TahiniType for SimpleServiceTahiniRequest {
        fn to_tahini_enum(&self) -> ::alohomora::tarpc::TahiniEnum {
            match self {
                SimpleServiceTahiniRequest::Increment { x } => {
                    ::alohomora::tarpc::enums::TahiniEnum::Enum(
                        "SimpleServiceTahiniRequest",
                        0u32,
                        "Increment",
                        {
                            ::alohomora::tarpc::enums::TahiniVariantsEnum::Struct(
                                ::std::collections::HashMap::from([
                                    (
                                        "x",
                                        <PCon<i32, ExamplePolicy> as TahiniType>::to_tahini_enum(x),
                                    ),
                                ]),
                            )
                        },
                    )
                }
            }
        }
        fn tahini_policy_check(
            &self,
            members_fmt: &String,
            context: &::alohomora::context::UnprotectedContext,
            reason: &::alohomora::policy::Reason,
        ) -> bool {
            match self {
                SimpleServiceTahiniRequest::Increment { x } => {
                    let mut policy_vec = Vec::new();
                    policy_vec.push(x.tahini_policy_check(members_fmt, context, reason));
                    policy_vec.iter().all(|x: &bool| *x)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(missing_docs)]
    impl ::core::clone::Clone for SimpleServiceTahiniRequest {
        #[inline]
        fn clone(&self) -> SimpleServiceTahiniRequest {
            match self {
                SimpleServiceTahiniRequest::Increment { x: __self_0 } => {
                    SimpleServiceTahiniRequest::Increment {
                        x: ::core::clone::Clone::clone(__self_0),
                    }
                }
            }
        }
    }
    /// The response sent over the wire from the server to the client.
    #[allow(missing_docs)]
    pub enum SimpleServiceTahiniResponse {
        Increment(PCon<String, ExamplePolicy>),
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SimpleServiceTahiniResponse {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Increment" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Increment" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SimpleServiceTahiniResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SimpleServiceTahiniResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum SimpleServiceTahiniResponse",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        PCon<String, ExamplePolicy>,
                                    >(__variant),
                                    SimpleServiceTahiniResponse::Increment,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Increment"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "SimpleServiceTahiniResponse",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            SimpleServiceTahiniResponse,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::alohomora::tarpc::TahiniType for SimpleServiceTahiniResponse {
        fn to_tahini_enum(&self) -> ::alohomora::tarpc::TahiniEnum {
            match self {
                SimpleServiceTahiniResponse::Increment(x) => {
                    ::alohomora::tarpc::enums::TahiniEnum::Enum(
                        "SimpleServiceTahiniResponse",
                        0u32,
                        "Increment",
                        ::alohomora::tarpc::enums::TahiniVariantsEnum::NewType(
                            Box::new(x.to_tahini_enum()),
                        ),
                    )
                }
            }
        }
        fn tahini_policy_check(
            &self,
            members_fmt: &String,
            context: &::alohomora::context::UnprotectedContext,
            reason: &::alohomora::policy::Reason,
        ) -> bool {
            match self {
                SimpleServiceTahiniResponse::Increment(x) => {
                    x.tahini_policy_check(members_fmt, context, reason)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(missing_docs)]
    impl ::core::clone::Clone for SimpleServiceTahiniResponse {
        #[inline]
        fn clone(&self) -> SimpleServiceTahiniResponse {
            match self {
                SimpleServiceTahiniResponse::Increment(__self_0) => {
                    SimpleServiceTahiniResponse::Increment(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    #[allow(unused)]
    /// The client stub that makes RPC calls to the server. All request methods return
    /// [Futures](::core::future::Future).
    pub struct TahiniSimpleServiceClient(
        ::alohomora::tarpc::client::TahiniChannel<
            SimpleServiceTahiniRequest,
            SimpleServiceTahiniResponse,
        >,
    );
    #[automatically_derived]
    #[allow(unused)]
    impl ::core::clone::Clone for TahiniSimpleServiceClient {
        #[inline]
        fn clone(&self) -> TahiniSimpleServiceClient {
            TahiniSimpleServiceClient(::core::clone::Clone::clone(&self.0))
        }
    }
    impl TahiniSimpleServiceClient {
        /// Returns a new client stub that sends requests over the given transport.
        pub fn new<T>(
            config: ::tarpc::client::Config,
            transport: T,
        ) -> ::alohomora::tarpc::client::TahiniNewClient<
            Self,
            ::alohomora::tarpc::client::TahiniRequestDispatch<
                SimpleServiceTahiniRequest,
                SimpleServiceTahiniResponse,
                T,
            >,
        >
        where
            T: ::tarpc::Transport<
                ::tarpc::ClientMessage<
                    ::alohomora::tarpc::enums::TahiniSafeWrapper<
                        SimpleServiceTahiniRequest,
                    >,
                >,
                ::tarpc::Response<SimpleServiceTahiniResponse>,
            >,
        {
            let new_client = ::alohomora::tarpc::client::new(config, transport);
            ::alohomora::tarpc::client::TahiniNewClient {
                client: TahiniSimpleServiceClient(new_client.client),
                dispatch: new_client.dispatch,
            }
        }
        #[allow(unused)]
        pub fn increment<
            InputLocalPolicy: ::alohomora::policy::PolicyInto<ExamplePolicy> + 'static
                + Send,
            OutputLocalType: ::alohomora::tarpc::traits::TahiniTransformFrom<
                    PCon<String, ExamplePolicy>,
                > + Send,
        >(
            &self,
            ctx: ::tarpc::context::Context,
            x: ::alohomora::bbox::BBox<i32, InputLocalPolicy>,
        ) -> impl ::core::future::Future<
            Output = ::core::result::Result<OutputLocalType, ::tarpc::client::RpcError>,
        > + '_ {
            let closure = |x| {
                SimpleServiceTahiniRequest::Increment {
                    x: x,
                }
            };
            let resp = self
                .0
                .transform_and_call(
                    ctx,
                    "SimpleServiceTahiniRequest.Increment",
                    "SimpleService.increment",
                    closure,
                    x,
                );
            let context = ::alohomora::tarpc::context::TahiniContext::new(
                "SimpleService",
                increment,
            );
            async move {
                match resp.await? {
                    SimpleServiceTahiniResponse::Increment(msg) => {
                        ::core::result::Result::Ok(
                            OutputLocalType::transform_from(msg, &context)
                                .expect("Couldn't transform output policy"),
                        )
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
    }
    pub struct SimpleServiceServer;
    #[automatically_derived]
    impl ::core::clone::Clone for SimpleServiceServer {
        #[inline]
        fn clone(&self) -> SimpleServiceServer {
            SimpleServiceServer
        }
    }
    impl SimpleService for SimpleServiceServer {
        async fn increment(
            self,
            _context: tarpc::context::Context,
            x: PCon<i32, ExamplePolicy>,
        ) -> PCon<String, ExamplePolicy> {
            {
                ::std::io::_print(
                    format_args!(
                        "Within the application level, we are operating on PCons.\n",
                    ),
                );
            };
            x.into_ppr(
                PPR::new(|val| ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{0}", val + 1));
                    res
                })),
            )
        }
    }
}
use crate::service::TahiniSimpleServiceClient;
use crate::service::{InnerStruct, MyType};
static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
static SENSITIVE_VALUE: i32 = 987654321;
fn main() -> anyhow::Result<()> {
    let body = async {
        let stream = TcpStream::connect((SERVER_ADDRESS, 5003)).await?;
        let codec_builder = LengthDelimitedCodec::builder();
        let x: MyType = MyType {
            a: 10,
            b: BBox::new(SENSITIVE_VALUE.to_string(), ExamplePolicy { field: 255 }),
            c: Ok(4),
        };
        let transport = new_transport(codec_builder.new_framed(stream), Json::default());
        let response: Result<BBox<String, ExamplePolicy>, RpcError> = TahiniSimpleServiceClient::new(
                Default::default(),
                transport,
            )
            .spawn()
            .increment(
                tarpc::context::current(),
                BBox::new(SENSITIVE_VALUE, ExamplePolicy { field: 201 }),
            )
            .await;
        match response {
            Ok(val) => {
                ::std::io::_print(format_args!("Got value {0:?}\n", val));
            }
            Err(e) => {
                ::std::io::_print(format_args!("Got error {0:?}\n", e));
            }
        }
        Ok(())
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
