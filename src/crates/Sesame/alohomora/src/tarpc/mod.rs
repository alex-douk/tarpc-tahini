use serde::{Deserialize, Serialize};
// Public re-exports of Tahini components and
// of only required unprotected types (2 enums)
pub use inner::{
    SimpleService, TahiniSimpleClient, UnprotectedSimpleServiceRequest,
    UnprotectedSimpleServiceResponse, ServeSimpleService
};
use crate::context::UnprotectedContext;
use crate::policy::{AnyPolicy, Policy, Reason};

#[derive(Serialize,Deserialize,Debug)]
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

/// Private mod with controlled public re-exports to have manageable leak. Ugly but it works.
/// The entire Tahini "generated" code lives inside this mod
/// To fall back to a fully exposed unprotected service, just remove the following line
/// and reexports above.
mod inner {
    use crate::bbox::BBox as PCon;
    use futures::{FutureExt, TryFutureExt};
    use tarpc::client;
    use tarpc::server::Serve;
    use crate::tarpc::ExamplePolicy;

    /// The developer defined trait (defined in Tahini only in the PoC)
    /// The base definition of this should be in the application code, but
    /// tarpc puts it back in scope with the rest of the generated code for
    /// `ServeSimpleService` ergonomics
    pub trait SimpleService: Sized {
        async fn increment(
            self,
            ctxt: tarpc::context::Context,
            x: PCon<i32, ExamplePolicy>,
        ) -> PCon<i32, ExamplePolicy>;

        //Usually provided by tarpc's interface
        fn serve(self) -> ServeSimpleService<Self> {
            ServeSimpleService { service: self }
        }
    }

    /// Definition of the underlying tarpc service
    /// Is inside the mod to not leak the unprotected service and client
    #[tarpc::service]
    pub trait UnprotectedSimpleService {
        async fn increment(x: (i32, ExamplePolicy)) -> (i32, ExamplePolicy);
    }

    /// Wrapper around the underlying unsafe client:
    /// Note the inner client is private and not exported elsewhere
    pub struct TahiniSimpleClient {
        inner: UnprotectedSimpleServiceClient,
    }

    // Works: Exposes only increment with PCons, and can communicate with a plain i32 server.
    /// Douk: API is minimal here: The type we define is:
    /// - 1) instantiated (and wrapped into some funky wrapper type `NewClient`
    ///      that is constructing the tarpc backend)
    /// - 2) responsible for wrap/unwrapping data and calling the unprotected underlying service
    ///
    /// Currently, we are leaking the datatypes, which probably means leaking the entire unprotected
    /// service altogether.
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
        ) -> client::NewClient<Self, client::RequestDispatch<UnprotectedSimpleServiceRequest, UnprotectedSimpleServiceResponse, T>>
        where
            T: tarpc::Transport<tarpc::ClientMessage<UnprotectedSimpleServiceRequest>, tarpc::Response<UnprotectedSimpleServiceResponse>>
        {
            let new_client = client::new(config, transport);
            client::NewClient {
                client: Self {
                    // The following line would not work if TahiniSimpleClient and
                    // the unprotected client didn't live in the same module
                    inner: UnprotectedSimpleServiceClient(new_client.client),
                },
                dispatch: new_client.dispatch,
            }
        }

        pub fn increment(
            &self,
            ctx: tarpc::context::Context,
            param: PCon<i32, ExamplePolicy>,
        ) -> impl std::future::Future<Output = Result<PCon<i32, ExamplePolicy>, client::RpcError>> + '_
        {
            println!("We are calling custom increment!");
            let res = self
                .inner
                .increment(ctx, param.consume())
                .map_ok(|(val, policy)| PCon::new(val, policy));
            res
        }
    }

    /// Required to imitate tarpc's implementation
    #[derive(Clone)]
    pub struct ServeSimpleService<S> {
        service: S,
    }

    /// Trait bound for generic server implementing the service
    /// This is also the last-level function to the actual RPC
    impl<S> Serve for ServeSimpleService<S>
    where
        S: SimpleService,
    {
        type Req = UnprotectedSimpleServiceRequest;
        type Resp = UnprotectedSimpleServiceResponse;

        async fn serve(
            self,
            ctx: tarpc::context::Context,
            req: Self::Req,
        ) -> Result<Self::Resp, tarpc::ServerError> {
            match req {
                UnprotectedSimpleServiceRequest::Increment { x: (v, p) } => {
                    println!("Within Sesame wrapper input data: {}", v);

                    // PING: DOUK: MODIFY HERE -----> implement serve for our Tahini server,
                    // following this logic!
                    let (r, p) = SimpleService::increment(
                        self.service,
                        ctx,
                        PCon::new(v, p),
                    )
                    .await
                    .consume();

                    println!("Within Sesame wrapper output data: {}", r);
                    Ok(UnprotectedSimpleServiceResponse::Increment((r, p)))
                }
            }
        }
    }
}
