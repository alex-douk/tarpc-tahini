use futures::TryFutureExt;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
// Public re-exports of Tahini components and
// // of only required unprotected types (2 enums)
use crate::context::UnprotectedContext;
use crate::policy::{AnyPolicy, Policy, Reason};
use crate::bbox::BBox as PCon;
// pub use inner::{
//     ServeSimpleService, SimpleService, UnprotectedSimpleServiceRequest,
//     UnprotectedSimpleServiceResponse, SimpleServiceRequest, SimpleServiceResponse
// };
use tarpc::server::Serve;
use crate::AlohomoraType;
//
// use tarpc::{client::Channel, context::Context};
//

#[derive(Clone)]
pub struct TahiniServe<Server: Serve + Clone>{
    inner: Server
}

impl<S: Serve + Clone> TahiniServe<S>{
    pub fn new(server: S) -> Self {
        TahiniServe { inner: server }
    }
}

///We are leaking the unprotected data type into application code?
///This is just a fancy pub consume...
impl<S> Serve for TahiniServe<S>
where 
    S: Serve + Clone,
    S::Resp: AlohomoraType
{
    type Req = S::Req;
    type Resp = <S::Resp as AlohomoraType>::Out;
    async fn serve(self, ctx: tarpc::context::Context, req: Self::Req) -> Result<Self::Resp, tarpc::ServerError> {
        let resp = self.inner.serve(ctx, req).map_ok(|rsp_fut| 
            S::Resp::from_enum(rsp_fut.to_enum().remove_bboxes()).unwrap(),
            ).await;
        resp
    }
}


// /// Private mod with controlled public re-exports to have manageable leak. Ugly but it works.
// /// The entire Tahini "generated" code lives inside this mod
// /// To fall back to a fully exposed unprotected service, just remove the following line
// /// and reexports above.
// mod inner {
//     use crate::bbox::BBox as PCon;
//     use crate::policy::AnyPolicy;
//     use crate::pure::PrivacyPureRegion;
//     use crate::tahini_pure::{execute_tahini, TahiniPrivacyPureRegion};
//     use crate::tarpc::ExamplePolicy;
//     use futures::{FutureExt, TryFutureExt};
//     use serde::{Deserialize, Serialize};
//     use tarpc::client;
//     use tarpc::server::Serve;
//
//     use super::TahiniServe;
//
//     /// The developer defined trait (defined in Tahini only in the PoC)
//     /// The base definition of this should be in the application code, but
//     /// tarpc puts it back in scope with the rest of the generated code for
//     /// `ServeSimpleService` ergonomics
//
//     /// Definition of the underlying tarpc service
//     /// Is inside the mod to not leak the unprotected service and client
//     #[tarpc::service]
//     pub trait UnprotectedSimpleService {
//         async fn increment(x: (i32, ExamplePolicy)) -> (i32, ExamplePolicy);
//     }
//
//
// }
