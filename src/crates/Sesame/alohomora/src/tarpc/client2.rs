use crate::tarpc::enums::TahiniSafeWrapper;
// use crate::tarpc::traits::{
//     deserialize_tahini_type, serialize_tahini_type, TahiniType, TahiniType2,
// };
use crate::tarpc::traits::TahiniType;
use futures::{FutureExt, TryFutureExt};
// use tarpc::transport::channel::ChannelError;
use pin_project_lite::pin_project;
use std::future::Future;
use std::marker::PhantomData;
use std::task::Poll;
use tarpc::client::Channel as TarpcChannel;
use tarpc::client::NewClient as TarpcNewClient;
use tarpc::client::RequestDispatch as TarpcRequestDispatch;
use tarpc::client::{Config, RpcError};
use tarpc::{context, ChannelError, ClientMessage, Response, Transport};

pub struct TahiniChannel2<Req: TahiniType, Resp: TahiniType> {
    channel: TarpcChannel<TahiniSafeWrapper<Req>, Resp>,
    // phantom_req: PhantomData<Req>,
    // phantom_resp: PhantomData<Resp>,
}

impl<'a, Req: TahiniType, Resp: TahiniType> TahiniChannel2<Req, Resp> {
    pub(crate) fn new(channel: TarpcChannel<TahiniSafeWrapper<Req>, Resp>) -> Self {
        Self {
            channel,
        }
    }
}

// mimics `tarpc::client::Stub`.
pub trait TahiniStub2 {
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

impl<Req: TahiniType + Clone, Resp: TahiniType> TahiniStub2 for TahiniChannel2<Req, Resp> {
    type Req = Req;
    type Resp = Resp;

    async fn call(
        &self,
        ctx: context::Context,
        request_name: &'static str,
        request: Req,
    ) -> Result<Self::Resp, RpcError> {
        let request = TahiniSafeWrapper(request);
        let response = self.channel.call(ctx, request_name, request).await?;
        Ok(response)
    }
}

// mimics `tarpc::client::RequestDispatch`.
pin_project! {
    pub struct TahiniRequestDispatch<Req: TahiniType, Resp: TahiniType, Trans>
    where
        Trans: TahiniTransport<Req, Resp>,
        // Transport<
        //     ClientMessage<SerWrapper<Req>>,
        //     Response<Resp>,
        // >,
    {
        #[pin]
        pub(super) dispatch: TarpcRequestDispatch<TahiniSafeWrapper<Req>, Resp, Trans>,
    }

}

//What is the current issue? The dispatch leaks the types.

impl<Req: TahiniType, Resp: TahiniType, Trans> TahiniRequestDispatch<Req, Resp, Trans>
where
    Trans: TahiniTransport<Req, Resp>,
    // Trans: Transport<
    //     ClientMessage<SerWrapper<Req>>,
    //     Response<Resp>,
    // >,
{
    pub(crate) fn new(dispatch: TarpcRequestDispatch<TahiniSafeWrapper<Req>, Resp, Trans>) -> Self {
        Self { dispatch }
    }
}

impl<Req, Resp, Trans> Future for TahiniRequestDispatch<Req, Resp, Trans>
where
    Req: TahiniType,
    Resp: TahiniType,
    Trans: TahiniTransport<Req, Resp>,
    Trans::TahiniTransportError: std::error::Error + Send + Sync + 'static, 
{
    type Output = Result<(), ChannelError<Trans::TahiniTransportError>>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        let res = this.dispatch.poll(cx);
        match res {
            //TODO(douk): Change to parse errors
            Poll::Ready(_) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending, // Poll::Ready(result) => result.map_err
        }
    }

    // fn poll(
    //     self: std::pin::Pin<&mut Self>,
    //     cx: &mut std::task::Context<'_>,
    // ) -> std::task::Poll<Self::Output> {
    //         //dispatch.poll_unpin(cx)
    // }
}

pub struct TahiniNewClient<C, D> {
    pub client: C,
    pub dispatch: D,
}
impl<E, C, Req, Resp, Trans> TahiniNewClient<C, TahiniRequestDispatch<Req, Resp, Trans>>
where
    Req: TahiniType + 'static,
    Resp: TahiniType + 'static,
    Trans: TahiniTransport<Req, Resp> + 'static,
    TahiniRequestDispatch<Req, Resp, Trans>: Future<Output = Result<(), E>> + Send + 'static,
    E: std::error::Error + Send + Sync + 'static, // TarpcRequestDispatch<
                                                  //     SerWrapper<Req>,
                                                  //     Resp,
                                                  //     Trans,
                                                  // >: Future<Output = Result<(), E>> + Send + 'static,
{
    pub fn spawn(self) -> C {
        let client = TarpcNewClient {
            client: self.client,
            dispatch: self.dispatch,
        };
        client.spawn()
    }
}

pub fn new<Req, Resp, Trans>(
    config: Config,
    transport: Trans,
) -> TahiniNewClient<TahiniChannel2<Req, Resp>, TahiniRequestDispatch<Req, Resp, Trans>>
where
    Req: TahiniType,
    Resp: TahiniType,
    Trans: TahiniTransport<Req, Resp>, // Transport<
                                       //     ClientMessage<SerWrapper<Req>>,
                                       //     Response<Resp>,
                                       // >
{
    let client = tarpc::client::new(config, transport);
    TahiniNewClient {
        client: TahiniChannel2::new(client.client),
        dispatch: TahiniRequestDispatch::new(client.dispatch),
    }
}

pub trait TahiniTransport<SinkItem: TahiniType, Item: TahiniType>:
    private::TahiniTransportInner<SinkItem, Item>
{
    type TahiniTransportError;
}

impl<T, SinkItem: TahiniType, Item: TahiniType> TahiniTransport<SinkItem, Item> for T
where
    T: private::TahiniTransportInner<SinkItem, Item>,
    T::InnerErrorType: std::error::Error + Send + Sync + 'static,
{
    type TahiniTransportError = RpcError;
}

mod private {
    use tarpc::{ClientMessage, Response, Transport, client::RpcError};
    use crate::tarpc::{
            enums::TahiniSafeWrapper,
            traits::TahiniType
    };
    pub trait TahiniTransportInner<SinkItem: TahiniType, Item: TahiniType>:
        tarpc::Transport<ClientMessage<TahiniSafeWrapper<SinkItem>>, Response<Item>> + Send
    {
        type InnerErrorType;
    }

    impl<T, SinkItem: TahiniType, Item: TahiniType> TahiniTransportInner<SinkItem, Item> for T
    where
        T: Transport<ClientMessage<TahiniSafeWrapper<SinkItem>>, Response<Item>> + Send,
        T::TransportError: std::error::Error + Send + Sync + 'static,
    {
        type InnerErrorType = RpcError;
    }
}

// use futures::{prelude::*, task::*};
// use serde::{Deserialize, Serialize};
// use std::{error::Error, io, pin::Pin};
// use tokio::io::{AsyncRead, AsyncWrite};
// use tokio_serde::{Framed as SerdeFramed, *};
// use tokio_util::codec::{length_delimited::LengthDelimitedCodec, Framed};
// use tarpc::serde_transport::Transport as TransportStruct;

// /// Constructs a new transport from a framed transport and a serialization codec.
// pub fn new_transport<S, Item, SinkItem, Codec>(
//     framed_io: Framed<S, LengthDelimitedCodec>,
//     codec: Codec,
// ) -> TransportStruct<S, Item, SinkItem, Codec>
// where
//     S: AsyncWrite + AsyncRead,
//     Item: for<'de> Deserialize<'de> + TahiniType2,
//     SinkItem: Serialize + TahiniType2,
//     Codec: Serializer<SinkItem> + Deserializer<Item>,
// {
//     TransportStruct {
//         inner: SerdeFramed::new(framed_io, codec),
//     }
// }
