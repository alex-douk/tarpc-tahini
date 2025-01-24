//Public re-exports of Tahini components and  
//of only required unprotected types (2 enums)
pub use inner::{
    SimpleService, TahiniSimpleClient, UnprotectedSimpleServiceRequest,
    UnprotectedSimpleServiceResponse, ServeSimpleService
};

///Private mod with controlled public re-exports to have manageable leak. Ugly but it works.
///The entire Tahini "generated" code lives inside this mod
///To fall back to a fully exposed unprotected service, just remove the following line
///and reexports above.
mod inner {

    use crate::{
        bbox::BBox as PCon,
        policy::{NoPolicy, Policy},
        AlohomoraType as SesameType, AlohomoraTypeEnum as SesameTypeEnum,
    };
    use futures::{FutureExt, TryFutureExt};
    use tarpc::client;
    use tarpc::server::BaseChannel;
    use tarpc::server::Serve;

    ///The developer defined trait (defined in Tahini only in the PoC)
    ///The base definition of this should be in the application code, but
    ///tarpc puts it back in scope with the rest of the generated code for
    ///`ServeSimpleService` ergonomics
    pub trait SimpleService: Sized {
        async fn increment(
            self,
            ctxt: tarpc::context::Context,
            x: PCon<i32, NoPolicy>,
        ) -> PCon<i32, NoPolicy>;

        //Usually provided by tarpc's interface
        fn serve(self) -> ServeSimpleService<Self> {
            ServeSimpleService { service: self }
        }
    }

    ///Definition of the underlying tarpc service
    ///Is inside the mod to not leak the unprotected service and client
    #[tarpc::service]
    pub trait UnprotectedSimpleService {
        async fn increment(x: i32) -> i32;
    }

    ///Wrapper around the underlying unsafe client:
    ///Note the inner client is private and not exported elsewhere
    pub struct TahiniSimpleClient {
        inner: UnprotectedSimpleServiceClient,
    }

    //Works: Exposes only increment with PCons, and can communicate with a plain i32 server.
    ///Douk: API is minimal here: The type we define is:
    ///- 1) instantiated (and wrapped into some funky
    ///wrapper type `NewClient` that is constructing the tarpc backend)
    ///- 2) responsible for wrap/unwrapping data and calling the unprotected underlying service
    ///
    /// Currently, we are leaking the datatypes, which probably means leaking the entire unprotected
    /// service altogether.
    /// Perks of this implementation:
    /// - We can use protected clients communicating with unprotected servers
    /// - "Tahini-checked vs unchecked" services can be annotated on a service-by-service basis, on the
    /// sending side!
    /// Cons of this implementation:
    /// - Passing the policy? A fix could be to redefined the unprotected service to contain the
    /// serialized policy as a standalone argument to the RPC.
    /// - Requires passing `Scrutinizer` on the dev-defined Codec
    impl TahiniSimpleClient {

        //Almost identical function to that of regular tarpc
        //We simply instantiate the wrapper struct
        pub fn new<T>(
            config: client::Config,
            transport: T,
        ) -> tarpc::client::NewClient<
            Self,
            tarpc::client::RequestDispatch<
                UnprotectedSimpleServiceRequest,
                UnprotectedSimpleServiceResponse,
                T,
            >,
        >
        where
            T: tarpc::Transport<
                tarpc::ClientMessage<UnprotectedSimpleServiceRequest>,
                tarpc::Response<UnprotectedSimpleServiceResponse>,
            >,
        {
            let new_client = tarpc::client::new(config, transport);
            client::NewClient {
                client: Self {
                    //The following line would not work if TahiniSimpleClient and
                    //the unprotected client didn't live in the same module
                    inner: UnprotectedSimpleServiceClient(new_client.client),
                },
                dispatch: new_client.dispatch,
            }
        }

        pub fn increment(
            &self,
            ctx: tarpc::context::Context,
            param: PCon<i32, NoPolicy>,
        ) -> impl std::future::Future<Output = Result<PCon<i32, NoPolicy>, tarpc::client::RpcError>> + '_
        {
            println!("We are calling custom increment!");
            let res = self
                .inner
                .increment(ctx, param.discard_box())
                .map_ok(|val| PCon::new(val, NoPolicy::default()));
            res
        }
    }

    ///Required to imitate tarpc's implementation
    #[derive(Clone)]
    pub struct ServeSimpleService<S> {
        service: S,
    }

    ///Trait bound for generic server implementing the service
    ///This is also the last-level function to the actual RPC
    impl<S> tarpc::server::Serve for ServeSimpleService<S>
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
                UnprotectedSimpleServiceRequest::Increment { x } => {
                    println!(
                        "Within the Sesame wrapper, we can read the unprotected input data: {}",
                        x
                    );
                    //PING: DOUK: MODIFY HERE -----> implement serve for our Tahini server,
                    // following this logic!
                    let res = SimpleService::increment(
                        self.service,
                        ctx,
                        PCon::new(x, NoPolicy::default()),
                    )
                    .await
                    .discard_box();
                    println!(
                        "Within the Sesame wrapper, we can read the unprotected output data: {}",
                        res
                    );
                    Ok(UnprotectedSimpleServiceResponse::Increment(res))
                }
            }
        }
    }
}
