use alohomora::{
    tahini_pure::*,
    tarpc::TahiniServe,
    bbox::BBox as PCon,
    context::UnprotectedContext,
    policy::{AnyPolicy, Policy, Reason},
    pure::PrivacyPureRegion
};
use serde::{Deserialize, Serialize};

use tarpc::{client, server::Serve};
use futures::{FutureExt, TryFutureExt};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExamplePolicy {
    pub field: u32,
}
impl Policy for ExamplePolicy {
    fn name(&self) -> String {
        String::from("ExamplePolicy")
    }
    fn check(&self, context: &UnprotectedContext, reason: Reason<'_>) -> bool {
        true
    }
    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        todo!()
    }
    fn join_logic(&self, other: Self) -> Result<Self, ()> {
        Ok(other)
    }
}

pub trait SimpleService: Sized + Clone {
    async fn increment(
        self,
        ctxt: tarpc::context::Context,
        x: PCon<i32, AnyPolicy>,
    ) -> PCon<i32, AnyPolicy>;

    //Usually provided by tarpc's interface
    fn serve(self) -> TahiniServe<ServeSimpleService<Self>> {
        TahiniServe::new(ServeSimpleService{ service : self})
    }
}

///Passing the policy here means having a policy that is (de)serializable
///AnyPolicy does not pass the vibe check
#[derive(Serialize, Deserialize)]
pub enum SimpleServiceRequest {
    Increment { x: i32 },
}

///Passing the policy here means having a policy that is (de)serializable
///AnyPolicy does not pass the vibe check
#[derive(Serialize, Deserialize)]
pub enum SimpleServiceResponse {
    Increment { x: i32 },
}


/// Wrapper around the underlying unsafe client:
/// Note the inner client is private and not exported elsewhere
pub struct TahiniSimpleClient<
    Stub = tarpc::client::Channel<SimpleServiceRequest, SimpleServiceResponse>,
>(Stub);

// Works: Exposes only increment with PCons, and can communicate with a plain i32 server.
/// Douk: API is minimal here: The type we define is:
/// - 1) instantiated (and wrapped into some funky wrapper type `NewClient`
///      that is constructing the tarpc backend)
/// - 2) responsible for wrap/unwrapping data and calling the unprotected underlying service
///
/// Currently, we are leaking the request and response enums datatypes
/// Perks of this implementation:
/// - We can use protected clients communicating with unprotected servers
/// - "Tahini-checked vs unchecked" services can be annotated on a service-by-service basis, on the
/// sending side!
/// Cons of this implementation:
/// - Passing the policy? A fix could be to redefined the unprotected service to contain the
///   serialized policy as a standalone argument to the RPC.
/// - Requires passing `Scrutinizer` on the dev-defined Codec
impl TahiniSimpleClient {
    // Almost identical function to that of regular tarpc
    // We simply instantiate the wrapper struct
    pub fn new<T>(
        config: client::Config,
        transport: T,
    ) -> client::NewClient<
        Self,
        client::RequestDispatch<SimpleServiceRequest, SimpleServiceResponse, T>,
    >
    where
        T: tarpc::Transport<
            tarpc::ClientMessage<SimpleServiceRequest>,
            tarpc::Response<SimpleServiceResponse>,
        >,
    {
        let new_client = client::new(config, transport);
        client::NewClient {
            client: TahiniSimpleClient(new_client.client),
            dispatch: new_client.dispatch,
        }
    }

    pub fn increment(
        &self,
        ctx: tarpc::context::Context,
        param: PCon<i32, AnyPolicy>,
    ) -> impl std::future::Future<Output = Result<PCon<i32, AnyPolicy>, client::RpcError>> + '_
    {
        //Could be changed to be handled directly in the executor:
        //We create a `TahiniReqEnum` trait with some mapping function rpc_string_name => enum
        //entry. This allows the executor to generate the request within its bounds, and attach
        //the policy to it.
        //What is the use case here? That a server, on the other end, can immediately retrieve
        //the policy, irrespective of the underlying enum (TODO: Make this more explicit)
        //
        let request_formatting_ppr =
            PrivacyPureRegion::new(|incr| SimpleServiceRequest::Increment { x: incr });
        let request = param.into_ppr(request_formatting_ppr);
        let rpc_name: &'static str = "SimpleService.increment";
        let tahini_executor = TahiniPrivacyPureRegion::new(self.0.clone());
        let resp = execute_tahini(tahini_executor, ctx, rpc_name, request);
        let response_extracting_ppr = PrivacyPureRegion::new(|rsp| match rsp {
            SimpleServiceResponse::Increment { x } => x,
        });
        resp.map_ok(move |rsp| rsp.into_ppr(response_extracting_ppr))
    }
}


/// Required to imitate tarpc's implementation
pub struct ServeSimpleService<S: Clone> {
    pub service: S,
}

impl<S: Clone> Clone for ServeSimpleService<S>{
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone()
        }
    }
}

/// Trait bound for generic server implementing the service
/// This is also the last-level function to the actual RPC
impl<S: Clone> Serve for ServeSimpleService<S>
where
    S: SimpleService,
{
    type Req = SimpleServiceRequest;
    type Resp = PCon<SimpleServiceResponse, AnyPolicy>;
    fn method(&self, req: &Self::Req) -> Option<&'static str> {
        match req {
            SimpleServiceRequest::Increment { .. } => Some("SimpleService.increment"),
            // _ => None //Makes sense
        }
    }

    async fn serve(
        self,
        ctx: tarpc::context::Context,
        req: Self::Req,
    ) -> Result<Self::Resp, tarpc::ServerError> {
        match req {
            SimpleServiceRequest::Increment { x: v } => {
                println!("Within Sesame wrapper input data: {}", v);

                // PING: DOUK: MODIFY HERE -----> implement serve for our Tahini server,
                // following this logic!
                let resp = SimpleService::increment(
                    self.service,
                    ctx,
                    PCon::new(v, AnyPolicy::new(ExamplePolicy { field: 255 })),
                )
                .await;

                let response_formatting_ppr = PrivacyPureRegion::new(|rsp|
                    SimpleServiceResponse::Increment { x: rsp });

                Ok(resp.into_ppr(response_formatting_ppr))
                // println!("Within Sesame wrapper output data: {}", r);
                // Ok(PCon::new(SimpleServiceResponse::Increment { x: r }, p))
            }
        }
    }
}
