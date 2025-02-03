use std::future::Future;
use futures::TryFutureExt;
use tarpc::client::{Config, RpcError};
use tarpc::client::Channel as TarpcChannel;
use tarpc::client::NewClient as TarpcNewClient;
use tarpc::client::RequestDispatch as TarpcRequestDispatch;
use tarpc::{context, ClientMessage, Response, Transport};
use crate::tarpc::traits::{deserialize_tahini_type, serialize_tahini_type, TahiniType};

// mimics `tarpc::client::Channel`.
pub struct TahiniChannel<Req: TahiniType, Resp: TahiniType> {
    channel: TarpcChannel<<Req as TahiniType>::Intermediate, <Resp as TahiniType>::Intermediate>,
}
impl<Req: TahiniType, Resp: TahiniType> TahiniChannel<Req, Resp> {
    pub fn new(channel: TarpcChannel<
        <Req as TahiniType>::Intermediate,
        <Resp as TahiniType>::Intermediate>,
    ) -> Self {
        Self { channel }
    }
}

// mimics `tarpc::client::Stub`.
pub trait TahiniStub {
    type Req: TahiniType;

    /// The service response type.
    type Resp: TahiniType;

    /// Calls a remote service.
    async fn call(
        &self,
        ctx: context::Context,
        request_name: &'static str,
        request: Self::Req,
    ) -> Result<Self::Resp, RpcError>;
}

impl<Req: TahiniType, Resp: TahiniType> TahiniStub for TahiniChannel<Req, Resp> {
    type Req = Req;
    type Resp = Resp;

    async fn call(
        &self,
        ctx: context::Context,
        request_name: &'static str,
        request: Req,
    ) -> Result<Self::Resp, RpcError> {
        let request = serialize_tahini_type(request);
        let response = self.channel.call(ctx, request_name, request).await?;
        Ok(deserialize_tahini_type(response))
    }
}

// mimics `tarpc::client::RequestDispatch`.
pub struct TahiniRequestDispatch<Req: TahiniType, Resp: TahiniType, Trans>
where
    Trans: Transport<
        ClientMessage<<Req as TahiniType>::Intermediate>,
        Response<<Resp as TahiniType>::Intermediate>,
    >,
{
    pub(super) dispatch: TarpcRequestDispatch<
        <Req as TahiniType>::Intermediate,
        <Resp as TahiniType>::Intermediate,
        Trans,
    >,
}
impl<Req: TahiniType, Resp: TahiniType, Trans> TahiniRequestDispatch<Req, Resp, Trans>
where
    Trans: Transport<
        ClientMessage<<Req as TahiniType>::Intermediate>,
        Response<<Resp as TahiniType>::Intermediate>,
    >,
{
    pub fn new(
        dispatch: TarpcRequestDispatch<
            <Req as TahiniType>::Intermediate,
            <Resp as TahiniType>::Intermediate,
            Trans
        >
    ) -> Self {
        Self { dispatch }
    }
}

pub struct TahiniNewClient<C, D> {
    pub client: C,
    pub dispatch: D,
}
impl<C, Req, Resp, Trans, E> TahiniNewClient<C, TahiniRequestDispatch<Req, Resp, Trans>>
where
    Req: TahiniType,
    Resp: TahiniType,
    E: std::error::Error + Send + Sync + 'static,
    Trans: Transport<
        ClientMessage<<Req as TahiniType>::Intermediate>,
        Response<<Resp as TahiniType>::Intermediate>,
    >,
    TarpcRequestDispatch<
        <Req as TahiniType>::Intermediate,
        <Resp as TahiniType>::Intermediate,
        Trans,
    >: Future<Output = Result<(), E>> + Send + 'static,
{
    pub fn spawn(
        self
    ) -> C {
        let client = TarpcNewClient {
            client: self.client,
            dispatch: self.dispatch.dispatch,
        };
        client.spawn()
    }
}

// mimics `tarpc::client::new(...)`.
pub fn new<Req, Resp, Trans>(
    config: Config,
    transport: Trans,
) -> TahiniNewClient<TahiniChannel<Req, Resp>, TahiniRequestDispatch<Req, Resp, Trans>>
where
    Req: TahiniType,
    Resp: TahiniType,
    Trans: Transport<
        ClientMessage<<Req as TahiniType>::Intermediate>,
        Response<<Resp as TahiniType>::Intermediate>,
    >
{
    let client = tarpc::client::new(config, transport);
    TahiniNewClient {
        client: TahiniChannel::new(client.client),
        dispatch: TahiniRequestDispatch::new(client.dispatch),
    }
}
