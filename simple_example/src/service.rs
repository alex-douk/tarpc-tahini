use std::collections::HashMap;

use alohomora::bbox::BBox as PCon;
use alohomora::pure::PrivacyPureRegion as PPR;
// use alohomora::tarpc::client::{TahiniChannel, TahiniNewClient, TahiniRequestDispatch, TahiniStub};
use alohomora::tarpc::client::{TahiniChannel, TahiniNewClient, TahiniRequestDispatch, TahiniStub};
// use alohomora::tarpc::enums::{DeboxedTahiniEnum, TahiniEnum, TahiniEnum2};
// use alohomora::tarpc::server::TahiniServe;
use alohomora::tarpc::server::TahiniServe;
// use alohomora::tarpc::traits::{NamedTahiniType, TahiniType, TahiniType2};
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

// use alohomora::tarpc::hacky::ExamplePolicy;
use crate::policy::ExamplePolicy;
//
#[derive(Debug, Deserialize, Clone, TahiniType)]
pub struct InnerStruct {
    pub a: u16,
}

// impl TahiniType for InnerStruct {
//     fn to_tahini_enum(&self) -> TahiniEnum {
//         let mut map = HashMap::new();
//         map.insert("a", TahiniEnum::Value(Box::new(self.a)));
//         TahiniEnum::Struct("InnerStruct", map)
//     }
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generic error")
    }
}

impl std::error::Error for MyError {}

impl TahiniError for MyError {}

#[derive(Debug, Deserialize, Clone, TahiniType)]
pub struct MyType {
    pub a: i32,
    pub b: PCon<String, ExamplePolicy>,
    pub c: Result<i32, MyError>,
}

// START: this part is what the developer writes.
// #[TahiniService]
#[tahini_service]
pub trait SimpleService {
    async fn increment(x: PCon<i32, ExamplePolicy>) -> PCon<String, ExamplePolicy>;
    async fn test_types(x: MyType) -> MyType;
}
#[derive(Clone)]
pub struct SimpleServiceServer;
impl SimpleService for SimpleServiceServer {
    async fn increment(
        self,
        _context: tarpc::context::Context,
        x: PCon<i32, ExamplePolicy>,
    ) -> PCon<String, ExamplePolicy> {
        println!("Within the application level, we are operating on PCons.");
        x.into_ppr(PPR::new(|val| format!("{}", val + 1)))
    }
    async fn test_types(self, ctxt: tarpc::context::Context, mut x: MyType) -> MyType {
        x.a = 0;
        x
    }
}
// END: the part that developers write is over.

