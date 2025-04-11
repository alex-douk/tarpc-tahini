#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod types {
    use crate::policies::MarketingPolicy;
    use alohomora::bbox::BBox;
    use alohomora::tarpc::TahiniType;
    use alohomora::TahiniType;
    use alohomora::tarpc::{TahiniSerialize, TahiniDeserialize};
    pub struct MarketingData {
        pub username: Option<String>,
        pub prompt: String,
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
        impl _serde::Serialize for MarketingData {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "MarketingData",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "prompt",
                    &self.prompt,
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
        impl<'de> _serde::Deserialize<'de> for MarketingData {
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
                            "username" => _serde::__private::Ok(__Field::__field0),
                            "prompt" => _serde::__private::Ok(__Field::__field1),
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
                            b"username" => _serde::__private::Ok(__Field::__field0),
                            b"prompt" => _serde::__private::Ok(__Field::__field1),
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
                    marker: _serde::__private::PhantomData<MarketingData>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MarketingData;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MarketingData",
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
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MarketingData with 2 elements",
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
                                        &"struct MarketingData with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(MarketingData {
                            username: __field0,
                            prompt: __field1,
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
                        let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "username",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("prompt"),
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
                                _serde::__private::de::missing_field("username")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("prompt")?
                            }
                        };
                        _serde::__private::Ok(MarketingData {
                            username: __field0,
                            prompt: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["username", "prompt"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MarketingData",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MarketingData>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for MarketingData {
        #[inline]
        fn clone(&self) -> MarketingData {
            MarketingData {
                username: ::core::clone::Clone::clone(&self.username),
                prompt: ::core::clone::Clone::clone(&self.prompt),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MarketingData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "MarketingData",
                "username",
                &self.username,
                "prompt",
                &&self.prompt,
            )
        }
    }
    pub struct Ad {
        pub ad: BBox<String, MarketingPolicy>,
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
        impl<'de> _serde::Deserialize<'de> for Ad {
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
                            "ad" => _serde::__private::Ok(__Field::__field0),
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
                            b"ad" => _serde::__private::Ok(__Field::__field0),
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
                    marker: _serde::__private::PhantomData<Ad>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Ad;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Ad")
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
                            BBox<String, MarketingPolicy>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Ad with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Ad { ad: __field0 })
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
                            BBox<String, MarketingPolicy>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("ad"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            BBox<String, MarketingPolicy>,
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
                                _serde::__private::de::missing_field("ad")?
                            }
                        };
                        _serde::__private::Ok(Ad { ad: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["ad"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Ad",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Ad>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Ad {
        #[inline]
        fn clone(&self) -> Ad {
            Ad {
                ad: ::core::clone::Clone::clone(&self.ad),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Ad {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Ad", "ad", &&self.ad)
        }
    }
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::alohomora::tarpc::TahiniType for Ad {
        fn to_tahini_enum(&self) -> ::alohomora::tarpc::TahiniEnum {
            let mut map: ::std::collections::HashMap<
                &'static str,
                ::alohomora::tarpc::TahiniEnum,
            > = ::std::collections::HashMap::from([
                (
                    "ad",
                    <BBox<
                        String,
                        MarketingPolicy,
                    > as TahiniType>::to_tahini_enum(&self.ad),
                ),
            ]);
            ::alohomora::tarpc::TahiniEnum::Struct("Ad", map)
        }
        fn tahini_policy_check(
            &self,
            members_fmt: &String,
            context: &::alohomora::context::UnprotectedContext,
            reason: &::alohomora::policy::Reason,
        ) -> bool {
            let mut policy_vec = Vec::new();
            policy_vec.push(self.ad.tahini_policy_check(members_fmt, context, reason));
            policy_vec.iter().all(|x: &bool| *x)
        }
    }
}
pub mod policies {
    use std::collections::HashMap;
    use alohomora::policy::{Policy, Reason};
    use alohomora::tarpc::{TahiniSerialize, TahiniDeserialize};
    pub static THIRD_PARTY_PROCESSORS: [&str; 2] = ["Meta_Ads", "Google_Ads"];
    ///This policy is given by an external organization so that remote clients can
    ///be compatible with it. This policies contains:
    ///- A storage consent
    ///- A targeted ads consent (.e.g, locally processed but still sent to the particular user)
    ///- Consent for various next-hops services.
    ///
    ///Note the lack of information regarding unprotected services (yet)
    pub struct MarketingPolicy {
        pub no_storage: bool,
        pub targeted_ads_consent: bool,
        pub third_party_processing: HashMap<String, bool>,
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
        impl _serde::Serialize for MarketingPolicy {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "MarketingPolicy",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "no_storage",
                    &self.no_storage,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "targeted_ads_consent",
                    &self.targeted_ads_consent,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "third_party_processing",
                    &self.third_party_processing,
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
        impl<'de> _serde::Deserialize<'de> for MarketingPolicy {
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
                            "no_storage" => _serde::__private::Ok(__Field::__field0),
                            "targeted_ads_consent" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            "third_party_processing" => {
                                _serde::__private::Ok(__Field::__field2)
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
                            b"no_storage" => _serde::__private::Ok(__Field::__field0),
                            b"targeted_ads_consent" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            b"third_party_processing" => {
                                _serde::__private::Ok(__Field::__field2)
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
                    marker: _serde::__private::PhantomData<MarketingPolicy>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MarketingPolicy;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MarketingPolicy",
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
                                        &"struct MarketingPolicy with 3 elements",
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
                                        &"struct MarketingPolicy with 3 elements",
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
                                        &"struct MarketingPolicy with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(MarketingPolicy {
                            no_storage: __field0,
                            targeted_ads_consent: __field1,
                            third_party_processing: __field2,
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
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "no_storage",
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
                                                "targeted_ads_consent",
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
                                                "third_party_processing",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
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
                                _serde::__private::de::missing_field("no_storage")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field(
                                    "targeted_ads_consent",
                                )?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field(
                                    "third_party_processing",
                                )?
                            }
                        };
                        _serde::__private::Ok(MarketingPolicy {
                            no_storage: __field0,
                            targeted_ads_consent: __field1,
                            third_party_processing: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "no_storage",
                    "targeted_ads_consent",
                    "third_party_processing",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MarketingPolicy",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MarketingPolicy>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for MarketingPolicy {
        #[inline]
        fn clone(&self) -> MarketingPolicy {
            MarketingPolicy {
                no_storage: ::core::clone::Clone::clone(&self.no_storage),
                targeted_ads_consent: ::core::clone::Clone::clone(
                    &self.targeted_ads_consent,
                ),
                third_party_processing: ::core::clone::Clone::clone(
                    &self.third_party_processing,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MarketingPolicy {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "MarketingPolicy",
                "no_storage",
                &self.no_storage,
                "targeted_ads_consent",
                &self.targeted_ads_consent,
                "third_party_processing",
                &&self.third_party_processing,
            )
        }
    }
    impl Policy for MarketingPolicy {
        fn name(&self) -> String {
            "MarketingPolicy".to_string()
        }
        fn check(
            &self,
            _context: &alohomora::context::UnprotectedContext,
            reason: alohomora::policy::Reason<'_>,
        ) -> bool {
            match reason {
                Reason::DB(_, _) => !self.no_storage,
                Reason::Response => true,
                Reason::Custom(reason) => {
                    match reason.cast().downcast_ref::<MarketingReason>() {
                        None => {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "We are failing the downcast to MarketingReason\n",
                                    ),
                                );
                            };
                            false
                        }
                        Some(reason) => {
                            match reason {
                                MarketingReason::Email => self.targeted_ads_consent,
                                MarketingReason::ThirdPartyProcessing(vendor) => {
                                    match self.third_party_processing.get(vendor) {
                                        None => false,
                                        Some(b) => *b,
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    {
                        ::std::io::_print(
                            format_args!("We are invoking for no good reason!\n"),
                        );
                    };
                    false
                }
            }
        }
        fn join(
            &self,
            other: alohomora::policy::AnyPolicy,
        ) -> Result<alohomora::policy::AnyPolicy, ()> {
            Ok(other)
        }
        fn join_logic(&self, _other: Self) -> Result<Self, ()>
        where
            Self: Sized,
        {
            Ok(self.clone())
        }
    }
    pub struct AdPolicy {}
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
        impl _serde::Serialize for AdPolicy {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "AdPolicy",
                    false as usize,
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
        impl<'de> _serde::Deserialize<'de> for AdPolicy {
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
                    marker: _serde::__private::PhantomData<AdPolicy>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AdPolicy;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct AdPolicy",
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
                        _serde::__private::Ok(AdPolicy {})
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
                        _serde::__private::Ok(AdPolicy {})
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "AdPolicy",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AdPolicy>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for AdPolicy {
        #[inline]
        fn clone(&self) -> AdPolicy {
            AdPolicy {}
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AdPolicy {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AdPolicy")
        }
    }
    pub enum MarketingReason {
        Email,
        ThirdPartyProcessing(String),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MarketingReason {
        #[inline]
        fn clone(&self) -> MarketingReason {
            match self {
                MarketingReason::Email => MarketingReason::Email,
                MarketingReason::ThirdPartyProcessing(__self_0) => {
                    MarketingReason::ThirdPartyProcessing(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
}
pub mod service {
    use crate::policies::MarketingPolicy;
    use alohomora::tarpc::traits::Fromable;
    use crate::types::{Ad, MarketingData};
    use alohomora::bbox::BBox;
    use alohomora::tahini_service;
    use alohomora::tarpc::{TahiniType, client::TahiniStub};
    pub trait Advertisement: ::core::marker::Sized + Clone {
        async fn auction_bidding(
            self,
            context: ::tarpc::context::Context,
            prompt: BBox<MarketingData, MarketingPolicy>,
        ) -> Ad;
        /// Returns a serving function to use with
        /// [InFlightRequest::execute](::tarpc::server::InFlightRequest::execute).
        fn serve(self) -> ServeAdvertisement<Self> {
            ServeAdvertisement {
                service: self,
            }
        }
    }
    /// A serving function to use with [::tarpc::server::InFlightRequest::execute].
    pub struct ServeAdvertisement<S> {
        service: S,
    }
    #[automatically_derived]
    impl<S: ::core::clone::Clone> ::core::clone::Clone for ServeAdvertisement<S> {
        #[inline]
        fn clone(&self) -> ServeAdvertisement<S> {
            ServeAdvertisement {
                service: ::core::clone::Clone::clone(&self.service),
            }
        }
    }
    impl<S> ::alohomora::tarpc::server::TahiniServe for ServeAdvertisement<S>
    where
        S: Advertisement + Clone,
    {
        type Req = AdvertisementTahiniRequest;
        type Resp = AdvertisementTahiniResponse;
        async fn serve(
            self,
            ctx: ::tarpc::context::Context,
            req: AdvertisementTahiniRequest,
        ) -> ::core::result::Result<AdvertisementTahiniResponse, ::tarpc::ServerError> {
            match req {
                AdvertisementTahiniRequest::AuctionBidding { prompt } => {
                    ::core::result::Result::Ok(
                        AdvertisementTahiniResponse::AuctionBidding(
                            Advertisement::auction_bidding(self.service, ctx, prompt)
                                .await,
                        ),
                    )
                }
            }
        }
    }
    /// The request sent over the wire from the client to the server.
    #[allow(missing_docs)]
    pub enum AdvertisementTahiniRequest {
        AuctionBidding { prompt: BBox<MarketingData, MarketingPolicy> },
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
        impl<'de> _serde::Deserialize<'de> for AdvertisementTahiniRequest {
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
                            "AuctionBidding" => _serde::__private::Ok(__Field::__field0),
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
                            b"AuctionBidding" => _serde::__private::Ok(__Field::__field0),
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
                    marker: _serde::__private::PhantomData<AdvertisementTahiniRequest>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AdvertisementTahiniRequest;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum AdvertisementTahiniRequest",
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
                                            "prompt" => _serde::__private::Ok(__Field::__field0),
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
                                            b"prompt" => _serde::__private::Ok(__Field::__field0),
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
                                        AdvertisementTahiniRequest,
                                    >,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = AdvertisementTahiniRequest;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant AdvertisementTahiniRequest::AuctionBidding",
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
                                            BBox<MarketingData, MarketingPolicy>,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant AdvertisementTahiniRequest::AuctionBidding with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(AdvertisementTahiniRequest::AuctionBidding {
                                            prompt: __field0,
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
                                            BBox<MarketingData, MarketingPolicy>,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("prompt"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<
                                                            BBox<MarketingData, MarketingPolicy>,
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
                                                _serde::__private::de::missing_field("prompt")?
                                            }
                                        };
                                        _serde::__private::Ok(AdvertisementTahiniRequest::AuctionBidding {
                                            prompt: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["prompt"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<
                                            AdvertisementTahiniRequest,
                                        >,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["AuctionBidding"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "AdvertisementTahiniRequest",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            AdvertisementTahiniRequest,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::alohomora::tarpc::TahiniType for AdvertisementTahiniRequest {
        fn to_tahini_enum(&self) -> ::alohomora::tarpc::TahiniEnum {
            match self {
                AdvertisementTahiniRequest::AuctionBidding { prompt } => {
                    ::alohomora::tarpc::enums::TahiniEnum::Enum(
                        "AdvertisementTahiniRequest",
                        0u32,
                        "AuctionBidding",
                        {
                            ::alohomora::tarpc::enums::TahiniVariantsEnum::Struct(
                                ::std::collections::HashMap::from([
                                    (
                                        "prompt",
                                        <BBox<
                                            MarketingData,
                                            MarketingPolicy,
                                        > as TahiniType>::to_tahini_enum(prompt),
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
                AdvertisementTahiniRequest::AuctionBidding { prompt } => {
                    let mut policy_vec = Vec::new();
                    policy_vec
                        .push(prompt.tahini_policy_check(members_fmt, context, reason));
                    policy_vec.iter().all(|x: &bool| *x)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(missing_docs)]
    impl ::core::clone::Clone for AdvertisementTahiniRequest {
        #[inline]
        fn clone(&self) -> AdvertisementTahiniRequest {
            match self {
                AdvertisementTahiniRequest::AuctionBidding { prompt: __self_0 } => {
                    AdvertisementTahiniRequest::AuctionBidding {
                        prompt: ::core::clone::Clone::clone(__self_0),
                    }
                }
            }
        }
    }
    /// The response sent over the wire from the server to the client.
    #[allow(missing_docs)]
    pub enum AdvertisementTahiniResponse {
        AuctionBidding(Fromable<'static, Ad>),
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
        impl<'de> _serde::Deserialize<'de> for AdvertisementTahiniResponse {
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
                            "AuctionBidding" => _serde::__private::Ok(__Field::__field0),
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
                            b"AuctionBidding" => _serde::__private::Ok(__Field::__field0),
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
                    marker: _serde::__private::PhantomData<AdvertisementTahiniResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AdvertisementTahiniResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum AdvertisementTahiniResponse",
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
                                        Fromable<'static, Ad>,
                                    >(__variant),
                                    AdvertisementTahiniResponse::AuctionBidding,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["AuctionBidding"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "AdvertisementTahiniResponse",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            AdvertisementTahiniResponse,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    ///Library implementation of TahiniType. Do not copy this docstring!
    impl ::alohomora::tarpc::TahiniType for AdvertisementTahiniResponse {
        fn to_tahini_enum(&self) -> ::alohomora::tarpc::TahiniEnum {
            match self {
                AdvertisementTahiniResponse::AuctionBidding(x) => {
                    ::alohomora::tarpc::enums::TahiniEnum::Enum(
                        "AdvertisementTahiniResponse",
                        0u32,
                        "AuctionBidding",
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
                AdvertisementTahiniResponse::AuctionBidding(x) => {
                    x.tahini_policy_check(members_fmt, context, reason)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(missing_docs)]
    impl ::core::clone::Clone for AdvertisementTahiniResponse {
        #[inline]
        fn clone(&self) -> AdvertisementTahiniResponse {
            match self {
                AdvertisementTahiniResponse::AuctionBidding(__self_0) => {
                    AdvertisementTahiniResponse::AuctionBidding(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    #[allow(unused)]
    /// The client stub that makes RPC calls to the server. All request methods return
    /// [Futures](::core::future::Future).
    pub struct TahiniAdvertisementClient(
        ::alohomora::tarpc::client::TahiniChannel<
            AdvertisementTahiniRequest,
            AdvertisementTahiniResponse,
        >,
    );
    #[automatically_derived]
    #[allow(unused)]
    impl ::core::clone::Clone for TahiniAdvertisementClient {
        #[inline]
        fn clone(&self) -> TahiniAdvertisementClient {
            TahiniAdvertisementClient(::core::clone::Clone::clone(&self.0))
        }
    }
    impl TahiniAdvertisementClient {
        /// Returns a new client stub that sends requests over the given transport.
        pub fn new<T>(
            config: ::tarpc::client::Config,
            transport: T,
        ) -> ::alohomora::tarpc::client::TahiniNewClient<
            Self,
            ::alohomora::tarpc::client::TahiniRequestDispatch<
                AdvertisementTahiniRequest,
                AdvertisementTahiniResponse,
                T,
            >,
        >
        where
            T: ::tarpc::Transport<
                ::tarpc::ClientMessage<
                    ::alohomora::tarpc::enums::TahiniSafeWrapper<
                        AdvertisementTahiniRequest,
                    >,
                >,
                ::tarpc::Response<AdvertisementTahiniResponse>,
            >,
        {
            let new_client = ::alohomora::tarpc::client::new(config, transport);
            ::alohomora::tarpc::client::TahiniNewClient {
                client: TahiniAdvertisementClient(new_client.client),
                dispatch: new_client.dispatch,
            }
        }
        #[allow(unused)]
        pub fn auction_bidding<
            InputLocalType: ::alohomora::tarpc::traits::TahiniTransformInto<
                    BBox<MarketingData, MarketingPolicy>,
                > + 'static + Send,
        >(
            &self,
            ctx: ::tarpc::context::Context,
            prompt: InputLocalType,
        ) -> impl ::core::future::Future<
            Output = ::core::result::Result<
                Fromable<'static, Ad>,
                ::tarpc::client::RpcError,
            >,
        > + '_ {
            let input_closure = |x: BBox<MarketingData, MarketingPolicy>| {
                AdvertisementTahiniRequest::AuctionBidding {
                    prompt: x,
                }
            };
            let output_closure = |resp: AdvertisementTahiniResponse| {
                match resp {
                    AdvertisementTahiniResponse::AuctionBidding(msg) => msg,
                    _ => {
                        ::core::panicking::unreachable_display(
                            &"Server RPC response doesn't match request RPC",
                        );
                    }
                }
            };
            self.0
                .transform_with_fromable::<
                    BBox<MarketingData, MarketingPolicy>,
                    InputLocalType,
                    _,
                    Ad,
                    _,
                >(
                    ctx,
                    "AdvertisementTahiniRequest.AuctionBidding",
                    "Advertisement.auction_bidding",
                    prompt,
                    input_closure,
                    output_closure,
                )
        }
    }
}
pub static THIRD_PARTY_PROCESSORS: [&str; 2] = ["Meta_Ads", "Google_Ads"];
