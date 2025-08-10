#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod types {
    use std::collections::HashMap;
    use alohomora::policy::Policy;
    use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
    use tahini_tarpc_derive::TahiniType;
    use alohomora::bbox::BBox;
    use alohomora::rocket::{RequestBBoxJson, ResponseBBoxJson};
    use tahini_tarpc::TahiniType;
    use crate::policies::MessagePolicy;
    pub struct UserPrompt {
        pub conversation: BBoxConversation,
        pub nb_token: u32,
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
        impl<'de> _serde::Deserialize<'de> for UserPrompt {
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
                            "conversation" => _serde::__private::Ok(__Field::__field0),
                            "nb_token" => _serde::__private::Ok(__Field::__field1),
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
                            b"conversation" => _serde::__private::Ok(__Field::__field0),
                            b"nb_token" => _serde::__private::Ok(__Field::__field1),
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
                    marker: _serde::__private::PhantomData<UserPrompt>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UserPrompt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UserPrompt",
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
                            BBoxConversation,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UserPrompt with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UserPrompt with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(UserPrompt {
                            conversation: __field0,
                            nb_token: __field1,
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
                        let mut __field0: _serde::__private::Option<BBoxConversation> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "conversation",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            BBoxConversation,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "nb_token",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
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
                                _serde::__private::de::missing_field("conversation")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("nb_token")?
                            }
                        };
                        _serde::__private::Ok(UserPrompt {
                            conversation: __field0,
                            nb_token: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["conversation", "nb_token"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UserPrompt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UserPrompt>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for UserPrompt {
        #[inline]
        fn clone(&self) -> UserPrompt {
            UserPrompt {
                conversation: ::core::clone::Clone::clone(&self.conversation),
                nb_token: ::core::clone::Clone::clone(&self.nb_token),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UserPrompt {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "UserPrompt",
                "conversation",
                &self.conversation,
                "nb_token",
                &&self.nb_token,
            )
        }
    }
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::tahini_tarpc::TahiniType for UserPrompt {
        fn to_tahini_enum(&self) -> ::tahini_tarpc::TahiniEnum {
            let mut map: ::std::collections::HashMap<
                &'static str,
                ::tahini_tarpc::TahiniEnum,
            > = ::std::collections::HashMap::from([
                (
                    "conversation",
                    <BBoxConversation as ::tahini_tarpc::TahiniType>::to_tahini_enum(
                        &self.conversation,
                    ),
                ),
                (
                    "nb_token",
                    <u32 as ::tahini_tarpc::TahiniType>::to_tahini_enum(&self.nb_token),
                ),
            ]);
            ::tahini_tarpc::TahiniEnum::Struct("UserPrompt", map)
        }
    }
    pub struct LLMResponse {
        pub infered_tokens: BBox<Result<Message, LLMError>, MessagePolicy>,
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
        impl<'de> _serde::Deserialize<'de> for LLMResponse {
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
                            "infered_tokens" => _serde::__private::Ok(__Field::__field0),
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
                            b"infered_tokens" => _serde::__private::Ok(__Field::__field0),
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
                    marker: _serde::__private::PhantomData<LLMResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = LLMResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct LLMResponse",
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
                            BBox<Result<Message, LLMError>, MessagePolicy>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct LLMResponse with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(LLMResponse {
                            infered_tokens: __field0,
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
                            BBox<Result<Message, LLMError>, MessagePolicy>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "infered_tokens",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            BBox<Result<Message, LLMError>, MessagePolicy>,
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
                                _serde::__private::de::missing_field("infered_tokens")?
                            }
                        };
                        _serde::__private::Ok(LLMResponse {
                            infered_tokens: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["infered_tokens"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "LLMResponse",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<LLMResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for LLMResponse {
        #[inline]
        fn clone(&self) -> LLMResponse {
            LLMResponse {
                infered_tokens: ::core::clone::Clone::clone(&self.infered_tokens),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LLMResponse {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "LLMResponse",
                "infered_tokens",
                &&self.infered_tokens,
            )
        }
    }
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::tahini_tarpc::TahiniType for LLMResponse {
        fn to_tahini_enum(&self) -> ::tahini_tarpc::TahiniEnum {
            let mut map: ::std::collections::HashMap<
                &'static str,
                ::tahini_tarpc::TahiniEnum,
            > = ::std::collections::HashMap::from([
                (
                    "infered_tokens",
                    <BBox<
                        Result<Message, LLMError>,
                        MessagePolicy,
                    > as ::tahini_tarpc::TahiniType>::to_tahini_enum(
                        &self.infered_tokens,
                    ),
                ),
            ]);
            ::tahini_tarpc::TahiniEnum::Struct("LLMResponse", map)
        }
    }
    pub type BBoxConversation = BBox<Vec<Message>, MessagePolicy>;
    pub struct Message {
        pub role: String,
        pub content: String,
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
        impl _serde::Serialize for Message {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Message",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "role",
                    &self.role,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "content",
                    &self.content,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl ::alohomora::rocket::RequestBBoxJson for Message {
        fn from_json(
            mut __value: ::alohomora::rocket::InputBBoxValue,
            __request: ::alohomora::rocket::BBoxRequest<'_, '_>,
        ) -> Result<Self, &'static str> {
            Ok(Self {
                role: __value.get("role")?.into_json(__request)?,
                content: __value.get("content")?.into_json(__request)?,
            })
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
        impl<'de> _serde::Deserialize<'de> for Message {
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
                            "role" => _serde::__private::Ok(__Field::__field0),
                            "content" => _serde::__private::Ok(__Field::__field1),
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
                            b"role" => _serde::__private::Ok(__Field::__field0),
                            b"content" => _serde::__private::Ok(__Field::__field1),
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
                    marker: _serde::__private::PhantomData<Message>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Message;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Message",
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
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Message with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Message with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Message {
                            role: __field0,
                            content: __field1,
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
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("role"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "content",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
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
                                _serde::__private::de::missing_field("role")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("content")?
                            }
                        };
                        _serde::__private::Ok(Message {
                            role: __field0,
                            content: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["role", "content"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Message",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Message>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Message {
        #[inline]
        fn clone(&self) -> Message {
            Message {
                role: ::core::clone::Clone::clone(&self.role),
                content: ::core::clone::Clone::clone(&self.content),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Message {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Message",
                "role",
                &self.role,
                "content",
                &&self.content,
            )
        }
    }
    impl ::alohomora::rocket::ResponseBBoxJson for Message {
        fn to_json(self) -> ::alohomora::rocket::OutputBBoxValue {
            ::alohomora::rocket::OutputBBoxValue::Object(
                HashMap::from([
                    (String::from("role"), self.role.to_json()),
                    (String::from("content"), self.content.to_json()),
                ]),
            )
        }
    }
    pub struct MyPol;
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
        impl<'de> _serde::Deserialize<'de> for MyPol {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MyPol>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MyPol;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "unit struct MyPol",
                        )
                    }
                    #[inline]
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(MyPol)
                    }
                }
                _serde::Deserializer::deserialize_unit_struct(
                    __deserializer,
                    "MyPol",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MyPol>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for MyPol {
        #[inline]
        fn clone(&self) -> MyPol {
            MyPol
        }
    }
    impl Policy for MyPol {
        fn name(&self) -> String {
            "aaa".to_string()
        }
        fn join(
            &self,
            other: alohomora::policy::AnyPolicy,
        ) -> Result<alohomora::policy::AnyPolicy, ()> {
            Ok(other)
        }
        fn join_logic(&self, other: Self) -> Result<Self, ()>
        where
            Self: Sized,
        {
            Ok(other)
        }
        fn check(
            &self,
            context: &alohomora::context::UnprotectedContext,
            reason: alohomora::policy::Reason<'_>,
        ) -> bool {
            true
        }
    }
    pub struct Prout {
        a: BBox<String, MyPol>,
    }
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::tahini_tarpc::TahiniType for Prout {
        fn to_tahini_enum(&self) -> ::tahini_tarpc::TahiniEnum {
            let mut map: ::std::collections::HashMap<
                &'static str,
                ::tahini_tarpc::TahiniEnum,
            > = ::std::collections::HashMap::from([
                (
                    "a",
                    <BBox<
                        String,
                        MyPol,
                    > as ::tahini_tarpc::TahiniType>::to_tahini_enum(&self.a),
                ),
            ]);
            ::tahini_tarpc::TahiniEnum::Struct("Prout", map)
        }
    }
    pub enum LLMError {
        InternalError,
        ValidationError,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LLMError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    LLMError::InternalError => "InternalError",
                    LLMError::ValidationError => "ValidationError",
                },
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
        impl<'de> _serde::Deserialize<'de> for LLMError {
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
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
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
                            "InternalError" => _serde::__private::Ok(__Field::__field0),
                            "ValidationError" => _serde::__private::Ok(__Field::__field1),
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
                            b"InternalError" => _serde::__private::Ok(__Field::__field0),
                            b"ValidationError" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
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
                    marker: _serde::__private::PhantomData<LLMError>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = LLMError;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum LLMError",
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
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(LLMError::InternalError)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(LLMError::ValidationError)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "InternalError",
                    "ValidationError",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "LLMError",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<LLMError>,
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
        impl _serde::Serialize for LLMError {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    LLMError::InternalError => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "LLMError",
                            0u32,
                            "InternalError",
                        )
                    }
                    LLMError::ValidationError => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "LLMError",
                            1u32,
                            "ValidationError",
                        )
                    }
                }
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for LLMError {
        #[inline]
        fn clone(&self) -> LLMError {
            match self {
                LLMError::InternalError => LLMError::InternalError,
                LLMError::ValidationError => LLMError::ValidationError,
            }
        }
    }
    impl std::fmt::Display for LLMError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("Internal LLM Error"))
        }
    }
    impl std::error::Error for LLMError {}
    impl tahini_tarpc::traits::TahiniError for LLMError {}
}
pub mod policies {
    pub(crate) mod message_policy {
        use alohomora::context::UnprotectedContext;
        use tahini_tarpc::sesame::db::{BBoxFromValue, Value};
        use alohomora::policy::{schema_policy, AnyPolicy, PolicyAnd};
        use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
        use alohomora::{
            policy::{FrontendPolicy, Policy, Reason, SchemaPolicy},
            rocket::{RocketCookie, RocketRequest},
        };
        use serde_json::from_str;
        use std::collections::HashMap;
        use std::str::FromStr;
        use super::UsernamePolicy;
        pub static THIRD_PARTY_PROCESSORS: [&str; 2] = ["Meta_Ads", "Google_Ads"];
        ///This policy is invoked when the use of conversation/message information
        ///Three main fields are invoked here:
        ///The storage to database (i.e. ephemeral chats)
        ///Allowing to send anonymized data to Tahini-fied third-parties
        ///Allowing the use of unprotected third-party services (e.g. image gen)
        pub struct MessagePolicy {
            pub storage: bool,
            pub marketing_consent: bool,
            pub third_party_consent: HashMap<String, bool>,
            pub unprotected_image_gen: bool,
            pub reinforcement_learning_consent: bool,
        }
        #[used]
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        #[link_section = ".init_array"]
        static ___register_MessagePolicy_conversations_4___ctor: unsafe extern "C" fn() = {
            #[link_section = ".text.startup"]
            unsafe extern "C" fn ___register_MessagePolicy_conversations_4___ctor() {
                register_MessagePolicy_conversations_4()
            }
            ___register_MessagePolicy_conversations_4___ctor
        };
        unsafe fn register_MessagePolicy_conversations_4() {
            ::alohomora::policy::add_schema_policy::<
                MessagePolicy,
            >(::std::string::String::from("conversations"), 4usize);
        }
        #[used]
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        #[link_section = ".init_array"]
        static ___register_MessagePolicy_conversations_3___ctor: unsafe extern "C" fn() = {
            #[link_section = ".text.startup"]
            unsafe extern "C" fn ___register_MessagePolicy_conversations_3___ctor() {
                register_MessagePolicy_conversations_3()
            }
            ___register_MessagePolicy_conversations_3___ctor
        };
        unsafe fn register_MessagePolicy_conversations_3() {
            ::alohomora::policy::add_schema_policy::<
                MessagePolicy,
            >(::std::string::String::from("conversations"), 3usize);
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
            impl _serde::Serialize for MessagePolicy {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "MessagePolicy",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "storage",
                        &self.storage,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "marketing_consent",
                        &self.marketing_consent,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "third_party_consent",
                        &self.third_party_consent,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "unprotected_image_gen",
                        &self.unprotected_image_gen,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "reinforcement_learning_consent",
                        &self.reinforcement_learning_consent,
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
            impl<'de> _serde::Deserialize<'de> for MessagePolicy {
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
                        __field3,
                        __field4,
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
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
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
                                "storage" => _serde::__private::Ok(__Field::__field0),
                                "marketing_consent" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                "third_party_consent" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                "unprotected_image_gen" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                "reinforcement_learning_consent" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
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
                                b"storage" => _serde::__private::Ok(__Field::__field0),
                                b"marketing_consent" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"third_party_consent" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                b"unprotected_image_gen" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                b"reinforcement_learning_consent" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
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
                        marker: _serde::__private::PhantomData<MessagePolicy>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = MessagePolicy;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct MessagePolicy",
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
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct MessagePolicy with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct MessagePolicy with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                HashMap<String, bool>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct MessagePolicy with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct MessagePolicy with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct MessagePolicy with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(MessagePolicy {
                                storage: __field0,
                                marketing_consent: __field1,
                                third_party_consent: __field2,
                                unprotected_image_gen: __field3,
                                reinforcement_learning_consent: __field4,
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
                            let mut __field0: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                HashMap<String, bool>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "storage",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "marketing_consent",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "third_party_consent",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                HashMap<String, bool>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "unprotected_image_gen",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "reinforcement_learning_consent",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
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
                                    _serde::__private::de::missing_field("storage")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("marketing_consent")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("third_party_consent")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field(
                                        "unprotected_image_gen",
                                    )?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field(
                                        "reinforcement_learning_consent",
                                    )?
                                }
                            };
                            _serde::__private::Ok(MessagePolicy {
                                storage: __field0,
                                marketing_consent: __field1,
                                third_party_consent: __field2,
                                unprotected_image_gen: __field3,
                                reinforcement_learning_consent: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "storage",
                        "marketing_consent",
                        "third_party_consent",
                        "unprotected_image_gen",
                        "reinforcement_learning_consent",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "MessagePolicy",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<MessagePolicy>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for MessagePolicy {
            #[inline]
            fn clone(&self) -> MessagePolicy {
                MessagePolicy {
                    storage: ::core::clone::Clone::clone(&self.storage),
                    marketing_consent: ::core::clone::Clone::clone(
                        &self.marketing_consent,
                    ),
                    third_party_consent: ::core::clone::Clone::clone(
                        &self.third_party_consent,
                    ),
                    unprotected_image_gen: ::core::clone::Clone::clone(
                        &self.unprotected_image_gen,
                    ),
                    reinforcement_learning_consent: ::core::clone::Clone::clone(
                        &self.reinforcement_learning_consent,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MessagePolicy {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "MessagePolicy",
                    "storage",
                    &self.storage,
                    "marketing_consent",
                    &self.marketing_consent,
                    "third_party_consent",
                    &self.third_party_consent,
                    "unprotected_image_gen",
                    &self.unprotected_image_gen,
                    "reinforcement_learning_consent",
                    &&self.reinforcement_learning_consent,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for MessagePolicy {
            #[inline]
            fn default() -> MessagePolicy {
                MessagePolicy {
                    storage: ::core::default::Default::default(),
                    marketing_consent: ::core::default::Default::default(),
                    third_party_consent: ::core::default::Default::default(),
                    unprotected_image_gen: ::core::default::Default::default(),
                    reinforcement_learning_consent: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MessagePolicy {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MessagePolicy {
            #[inline]
            fn eq(&self, other: &MessagePolicy) -> bool {
                self.storage == other.storage
                    && self.marketing_consent == other.marketing_consent
                    && self.third_party_consent == other.third_party_consent
                    && self.unprotected_image_gen == other.unprotected_image_gen
                    && self.reinforcement_learning_consent
                        == other.reinforcement_learning_consent
            }
        }
        impl Policy for MessagePolicy {
            fn name(&self) -> String {
                "PromptPolicy".to_string()
            }
            fn check(&self, _context: &UnprotectedContext, reason: Reason<'_>) -> bool {
                match reason {
                    Reason::DB(_, _) => self.storage,
                    Reason::Response => true,
                    Reason::Custom(reason) => {
                        match reason.cast().downcast_ref::<InferenceReason>() {
                            None => false,
                            Some(reason) => {
                                match reason {
                                    InferenceReason::SendToMarketing => self.marketing_consent,
                                    InferenceReason::SendToImageGen => {
                                        self.unprotected_image_gen
                                    }
                                    InferenceReason::SendToDB => self.storage,
                                }
                            }
                        }
                    }
                    _ => false,
                }
            }
            fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
                if other.is::<MessagePolicy>() {
                    self.join_logic(other.specialize().map_err(|_| ())?)
                        .map(|pol| pol.into_any())
                } else if other.is::<UsernamePolicy>() {
                    let spec = other.specialize::<UsernamePolicy>();
                    if spec.is_err() {
                        return Err(());
                    }
                    Ok(AnyPolicy::new(PolicyAnd::new(self.clone(), spec.unwrap())))
                } else {
                    Ok(AnyPolicy::new(PolicyAnd::new(self.clone(), other)))
                }
            }
            fn join_logic(&self, other: Self) -> Result<Self, ()>
            where
                Self: Sized,
            {
                let mut hashmap = self.third_party_consent.clone();
                for (key, value) in other.third_party_consent.iter() {
                    hashmap
                        .entry(key.clone())
                        .and_modify(|e| *e = *e && *value)
                        .or_insert(*value);
                }
                Ok(MessagePolicy {
                    third_party_consent: hashmap,
                    storage: self.storage && other.storage,
                    marketing_consent: self.marketing_consent && other.marketing_consent,
                    unprotected_image_gen: self.unprotected_image_gen
                        && other.unprotected_image_gen,
                    reinforcement_learning_consent: self.reinforcement_learning_consent
                        && other.reinforcement_learning_consent,
                })
            }
            fn into_any(self) -> AnyPolicy
            where
                Self: Sized,
            {
                AnyPolicy::new(self)
            }
        }
        pub enum InferenceReason {
            SendToMarketing,
            SendToImageGen,
            SendToDB,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for InferenceReason {
            #[inline]
            fn clone(&self) -> InferenceReason {
                match self {
                    InferenceReason::SendToMarketing => InferenceReason::SendToMarketing,
                    InferenceReason::SendToImageGen => InferenceReason::SendToImageGen,
                    InferenceReason::SendToDB => InferenceReason::SendToDB,
                }
            }
        }
        impl FrontendPolicy for MessagePolicy {
            fn from_request<'a, 'r>(request: &'a RocketRequest<'r>) -> Self
            where
                Self: Sized,
            {
                let no_storage = bool::from_str(
                        request.cookies().get("storage").unwrap().value(),
                    )
                    .unwrap();
                let marketing_consent = bool::from_str(
                        request.cookies().get("ads").unwrap().value(),
                    )
                    .unwrap();
                let unprotected_image_gen = bool::from_str(
                        request.cookies().get("image_gen").unwrap().value(),
                    )
                    .unwrap();
                let mut hashmap = HashMap::with_capacity(THIRD_PARTY_PROCESSORS.len());
                for vendor in THIRD_PARTY_PROCESSORS {
                    let cookie = request.cookies().get(vendor);
                    hashmap
                        .insert(
                            vendor.to_string(),
                            match cookie {
                                None => false,
                                Some(c) => bool::from_str(c.value()).unwrap_or(false),
                            },
                        );
                }
                MessagePolicy {
                    third_party_consent: hashmap,
                    storage: no_storage,
                    marketing_consent,
                    unprotected_image_gen,
                    reinforcement_learning_consent: false,
                }
            }
            fn from_cookie<'a, 'r>(
                _name: &str,
                _cookie: &'a RocketCookie<'static>,
                request: &'a RocketRequest<'r>,
            ) -> Self
            where
                Self: Sized,
            {
                Self::from_request(request)
            }
        }
        impl SchemaPolicy for MessagePolicy {
            fn from_row(_table_name: &str, row: &Vec<Value>) -> Self
            where
                Self: Sized,
            {
                let value = <String as BBoxFromValue>::from_value(row[9].clone());
                let hashmap = match from_str(value.as_str()) {
                    Ok(map) => map,
                    Err(_) => {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "Couldn\'t parse consent table into the proper type, got {0}\n",
                                    value,
                                ),
                            );
                        };
                        HashMap::<String, bool>::new()
                    }
                };
                MessagePolicy {
                    third_party_consent: hashmap,
                    storage: BBoxFromValue::from_value(row[5].clone()),
                    marketing_consent: BBoxFromValue::from_value(row[6].clone()),
                    unprotected_image_gen: BBoxFromValue::from_value(row[7].clone()),
                    reinforcement_learning_consent: false,
                }
            }
        }
    }
    mod username_policy {
        use crate::policies::message_policy::InferenceReason;
        use tahini_tarpc::sesame::db::{BBoxFromValue, Value};
        use alohomora::policy::{
            schema_policy, AnyPolicy, FrontendPolicy, PolicyAnd, Reason,
        };
        use tahini_tarpc::sesame::rocket::{RocketCookie, RocketRequest};
        use serde_json::from_str;
        use std::collections::HashMap;
        use std::str::FromStr;
        use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
        use alohomora::policy::{SchemaPolicy, Policy};
        use alohomora::context::UnprotectedContext;
        use super::MessagePolicy;
        pub static THIRD_PARTY_PROCESSORS: [&str; 2] = ["Meta_Ads", "Google_Ads"];
        ///This policy is user-and-session-bound and
        ///is invoked in operations that could lead to current-or-future disclosure of the username
        pub struct UsernamePolicy {
            pub targeted_ads_consent: bool,
            pub third_party_vendors_consent: HashMap<String, bool>,
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
            impl _serde::Serialize for UsernamePolicy {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "UsernamePolicy",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "targeted_ads_consent",
                        &self.targeted_ads_consent,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "third_party_vendors_consent",
                        &self.third_party_vendors_consent,
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
            impl<'de> _serde::Deserialize<'de> for UsernamePolicy {
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
                                "targeted_ads_consent" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "third_party_vendors_consent" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
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
                                b"targeted_ads_consent" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"third_party_vendors_consent" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
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
                        marker: _serde::__private::PhantomData<UsernamePolicy>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = UsernamePolicy;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct UsernamePolicy",
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
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct UsernamePolicy with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                HashMap<String, bool>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct UsernamePolicy with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(UsernamePolicy {
                                targeted_ads_consent: __field0,
                                third_party_vendors_consent: __field1,
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
                            let mut __field0: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                HashMap<String, bool>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "targeted_ads_consent",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "third_party_vendors_consent",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                HashMap<String, bool>,
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
                                    _serde::__private::de::missing_field(
                                        "targeted_ads_consent",
                                    )?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field(
                                        "third_party_vendors_consent",
                                    )?
                                }
                            };
                            _serde::__private::Ok(UsernamePolicy {
                                targeted_ads_consent: __field0,
                                third_party_vendors_consent: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "targeted_ads_consent",
                        "third_party_vendors_consent",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "UsernamePolicy",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<UsernamePolicy>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for UsernamePolicy {
            #[inline]
            fn clone(&self) -> UsernamePolicy {
                UsernamePolicy {
                    targeted_ads_consent: ::core::clone::Clone::clone(
                        &self.targeted_ads_consent,
                    ),
                    third_party_vendors_consent: ::core::clone::Clone::clone(
                        &self.third_party_vendors_consent,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UsernamePolicy {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UsernamePolicy",
                    "targeted_ads_consent",
                    &self.targeted_ads_consent,
                    "third_party_vendors_consent",
                    &&self.third_party_vendors_consent,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for UsernamePolicy {
            #[inline]
            fn default() -> UsernamePolicy {
                UsernamePolicy {
                    targeted_ads_consent: ::core::default::Default::default(),
                    third_party_vendors_consent: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UsernamePolicy {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UsernamePolicy {
            #[inline]
            fn eq(&self, other: &UsernamePolicy) -> bool {
                self.targeted_ads_consent == other.targeted_ads_consent
                    && self.third_party_vendors_consent
                        == other.third_party_vendors_consent
            }
        }
        #[used]
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        #[link_section = ".init_array"]
        static ___register_UsernamePolicy_users_1___ctor: unsafe extern "C" fn() = {
            #[link_section = ".text.startup"]
            unsafe extern "C" fn ___register_UsernamePolicy_users_1___ctor() {
                register_UsernamePolicy_users_1()
            }
            ___register_UsernamePolicy_users_1___ctor
        };
        unsafe fn register_UsernamePolicy_users_1() {
            ::alohomora::policy::add_schema_policy::<
                UsernamePolicy,
            >(::std::string::String::from("users"), 1usize);
        }
        impl Policy for UsernamePolicy {
            fn name(&self) -> String {
                "UsernamePolicy".to_string()
            }
            fn check(&self, context: &UnprotectedContext, reason: Reason<'_>) -> bool {
                match reason {
                    Reason::Response => true,
                    Reason::DB(_, _) => true,
                    Reason::Custom(reason) => {
                        match reason.cast().downcast_ref::<InferenceReason>() {
                            None => false,
                            Some(reason) => {
                                match reason {
                                    InferenceReason::SendToMarketing => {
                                        self.targeted_ads_consent
                                    }
                                    _ => false,
                                }
                            }
                        }
                    }
                    _ => false,
                }
            }
            fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
                if other.is::<UsernamePolicy>() {
                    self.join_logic(other.specialize().map_err(|_| ())?)
                        .map(|p| AnyPolicy::new(p))
                } else if other.is::<MessagePolicy>() {
                    let spec = other.specialize::<MessagePolicy>();
                    if spec.is_err() {
                        return Err(());
                    }
                    Ok(AnyPolicy::new(PolicyAnd::new(self.clone(), spec.unwrap())))
                } else {
                    Ok(AnyPolicy::new(PolicyAnd::new(self.clone(), other)))
                }
            }
            fn join_logic(&self, other: Self) -> Result<Self, ()>
            where
                Self: Sized,
            {
                Ok(self.clone())
            }
            fn into_any(self) -> AnyPolicy
            where
                Self: Sized,
            {
                AnyPolicy::new(self)
            }
        }
        impl SchemaPolicy for UsernamePolicy {
            fn from_row(table_name: &str, row: &Vec<Value>) -> Self
            where
                Self: Sized,
            {
                let value = match table_name {
                    "users" => <String as BBoxFromValue>::from_value(row[3].clone()),
                    "conversations" => {
                        <String as BBoxFromValue>::from_value(row[9].clone())
                    }
                    _ => "{}".to_string(),
                };
                let hashmap = match from_str(value.as_str()) {
                    Ok(map) => map,
                    Err(_) => {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "Couldn\'t parse consent table into the proper type, got {0}\n",
                                    value,
                                ),
                            );
                        };
                        HashMap::<String, bool>::new()
                    }
                };
                Self {
                    third_party_vendors_consent: hashmap,
                    targeted_ads_consent: match table_name {
                        "users" => BBoxFromValue::from_value(row[2].clone()),
                        "conversations" => BBoxFromValue::from_value(row[8].clone()),
                        _ => false,
                    },
                }
            }
        }
        impl FrontendPolicy for UsernamePolicy {
            fn from_cookie<'a, 'r>(
                _name: &str,
                _cookie: &'a RocketCookie<'static>,
                request: &'a RocketRequest<'r>,
            ) -> Self
            where
                Self: Sized,
            {
                let mut hashmap = HashMap::with_capacity(THIRD_PARTY_PROCESSORS.len());
                for vendor in THIRD_PARTY_PROCESSORS {
                    let cookie = request.cookies().get(vendor);
                    hashmap
                        .insert(
                            vendor.to_string(),
                            match cookie {
                                None => false,
                                Some(c) => bool::from_str(c.value()).unwrap_or(false),
                            },
                        );
                }
                UsernamePolicy {
                    third_party_vendors_consent: hashmap,
                    targeted_ads_consent: match request.cookies().get("targeted_ads") {
                        None => false,
                        Some(c) => {
                            match bool::from_str(c.value()) {
                                Ok(b) => b,
                                Err(_) => false,
                            }
                        }
                    },
                }
            }
            fn from_request<'a, 'r>(request: &'a RocketRequest<'r>) -> Self
            where
                Self: Sized,
            {
                let mut hashmap = HashMap::with_capacity(THIRD_PARTY_PROCESSORS.len());
                for vendor in THIRD_PARTY_PROCESSORS {
                    let cookie = request.cookies().get(vendor);
                    hashmap
                        .insert(
                            vendor.to_string(),
                            match cookie {
                                None => false,
                                Some(c) => bool::from_str(c.value()).unwrap_or(false),
                            },
                        );
                }
                UsernamePolicy {
                    third_party_vendors_consent: hashmap,
                    targeted_ads_consent: match request.cookies().get("targeted_ads") {
                        None => false,
                        Some(c) => {
                            match bool::from_str(c.value()) {
                                Ok(b) => b,
                                Err(_) => false,
                            }
                        }
                    },
                }
            }
        }
        ///Used for internal processing. Can be passed around at unchecked RPCs, but can never leave the
        ///org nor be passed to checked RPCs.
        ///Such a policy can ensure that data paths terminating in an uncontrolled sink are taken into
        ///account.
        pub struct AbsolutePolicy {}
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
            impl<'de> _serde::Deserialize<'de> for AbsolutePolicy {
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
                        marker: _serde::__private::PhantomData<AbsolutePolicy>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = AbsolutePolicy;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct AbsolutePolicy",
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
                            _serde::__private::Ok(AbsolutePolicy {})
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
                            _serde::__private::Ok(AbsolutePolicy {})
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AbsolutePolicy",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<AbsolutePolicy>,
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
            impl _serde::Serialize for AbsolutePolicy {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "AbsolutePolicy",
                        false as usize,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for AbsolutePolicy {
            #[inline]
            fn clone(&self) -> AbsolutePolicy {
                AbsolutePolicy {}
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AbsolutePolicy {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "AbsolutePolicy")
            }
        }
        impl Policy for AbsolutePolicy {
            fn name(&self) -> String {
                "AbsolutePolicy".to_string()
            }
            fn check(&self, _context: &UnprotectedContext, _reason: Reason<'_>) -> bool {
                false
            }
            fn join(&self, _other: AnyPolicy) -> Result<AnyPolicy, ()> {
                Ok(self.clone().into_any())
            }
            fn join_logic(&self, other: Self) -> Result<Self, ()>
            where
                Self: Sized,
            {
                Ok(other)
            }
            fn into_any(self) -> AnyPolicy
            where
                Self: Sized,
            {
                AnyPolicy::new(self)
            }
        }
    }
    mod adapters {
        use alohomora::policy::Policy;
    }
    pub use self::message_policy::MessagePolicy;
    pub use self::username_policy::AbsolutePolicy;
    pub use self::username_policy::UsernamePolicy;
    pub use self::message_policy::InferenceReason;
}
pub mod funcs {
    use crate::types::{LLMError, Message};
    pub fn validate_user(role: String) -> Result<String, LLMError> {
        match role.as_str() {
            role @ ("user" | "model") => Ok(role.to_string()),
            role @ _ => {
                {
                    ::std::io::_print(
                        format_args!("Role is not that expected: Got {0}\n", role),
                    );
                };
                Err(LLMError::ValidationError)
            }
        }
    }
    pub fn validate_body(body: String) -> Result<String, LLMError> {
        match !(body.contains("<start_of_turn>") || body.contains("<end_of_turn>")) {
            true => Ok(body),
            false => {
                {
                    ::std::io::_print(format_args!("Body is : {0}\n", body));
                };
                Err(LLMError::ValidationError)
            }
        }
    }
    pub fn parse_message(message: Message) -> Result<String, LLMError> {
        Ok({
            let res = ::alloc::fmt::format(
                format_args!(
                    "<start_of_turn>{0}\n{1}<end_of_turn>\n",
                    validate_user(message.role)?,
                    validate_body(message.content)?,
                ),
            );
            res
        })
    }
    pub fn parse_conversation(conv: Vec<Message>) -> Result<String, LLMError> {
        conv.into_iter()
            .map(|x| parse_message(x.clone()))
            .collect::<Result<Vec<_>, LLMError>>()
            .map(|mut vec| {
                vec.push("<start_of_turn>model\n".to_string());
                vec
            })
            .map(|vec| vec.join(""))
    }
    pub fn parse_stored_conversation(
        stored_conv: String,
    ) -> Result<Vec<Message>, LLMError> {
        let mut messages = Vec::new();
        let parts = stored_conv.split("<start_of_turn>").collect::<Vec<_>>();
        for part in parts.iter().skip(1) {
            match part.split_once('\n') {
                Some((role, rest)) => {
                    match rest.split_once("<end_of_turn>") {
                        Some((content, _)) => {
                            messages
                                .push(Message {
                                    role: role.to_string(),
                                    content: content.trim_end_matches('\n').to_string(),
                                })
                        }
                        None => return Err(LLMError::ValidationError),
                    }
                }
                None => return Err(LLMError::ValidationError),
            }
        }
        Ok(messages)
    }
    pub fn marketing_parse_conv(conv: Vec<Message>) -> String {
        conv.into_iter()
            .map(|x| {
                let res = ::alloc::fmt::format(
                    format_args!("[{0}]:{1}", x.role.to_uppercase(), x.content),
                );
                res
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
