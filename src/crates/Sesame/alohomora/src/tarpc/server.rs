use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

use pin_project_lite::pin_project;
use futures::{Sink, Stream};
use serde::Deserializer;
use tarpc::{ChannelError, ClientMessage, Response, ServerError, Transport};
use tarpc::context::Context;
use tarpc::server::{Config, TrackedRequest};
use tarpc::server::BaseChannel as TarpcBaseChannel;
use tarpc::server::Channel as TarpcChannel;
use tarpc::server::Serve as TarpcServe;
use crate::tarpc::traits::{deserialize_tahini_type, serialize_tahini_type, NamedTahiniType, TahiniType};

// mimics `tarpc::server::Channel`.
pub trait TahiniChannel where Self: Sized {
    type Req: NamedTahiniType;
    type Resp: TahiniType;
    type Transport: Transport<
        Response<<Self::Resp as TahiniType>::Intermediate>,
        ClientMessage<<Self::Req as TahiniType>::Intermediate>,
    >;

    fn config(&self) -> &Config;
    fn in_flight_requests(&self) -> usize;
    fn transport(&self) -> &Self::Transport;

    // Hide this because Requests leaks data.
    // fn requests(self) -> Requests<Self>

    // Forces application developers to use our `Serve` type, instead of any tarpc serve.
    fn execute<S>(self, serve: S) -> impl Stream<Item = impl Future<Output = ()>>
    where
        S: TahiniServe<Req = Self::Req, Resp = Self::Resp> + Clone;
}

// mimics `tarpc::server::BaseChannel`.
pin_project! {
    pub struct TahiniBaseChannel<Req, Resp, Trans>
    where
        Req: NamedTahiniType,
        Resp: TahiniType,
        Trans: Transport<Response<<Resp as TahiniType>::Intermediate>, ClientMessage<<Req as TahiniType>::Intermediate>>,
    {
        #[pin]
        channel: TarpcBaseChannel<Req::Intermediate, <Resp as TahiniType>::Intermediate, Trans>,
    }
}
impl<Req, Resp, Trans> TahiniBaseChannel<Req, Resp, Trans>
where
    Req: NamedTahiniType,
    Resp: TahiniType,
    Trans: Transport<Response<<Resp as TahiniType>::Intermediate>, ClientMessage<<Req as TahiniType>::Intermediate>>,
{
    pub fn new(config: Config, transport: Trans) -> Self {
        Self {
            channel: TarpcBaseChannel::new(config, transport),
        }
    }
    pub fn with_defaults(transport: Trans) -> Self {
        Self {
            channel: TarpcBaseChannel::with_defaults(transport),
        }
    }
}
impl<Req, Resp, Trans> TahiniChannel for TahiniBaseChannel<Req, Resp, Trans>
where
    Req: NamedTahiniType,
    Resp: TahiniType,
    Trans: Transport<Response<<Resp as TahiniType>::Intermediate>, ClientMessage<<Req as TahiniType>::Intermediate>>,
{
    type Req = Req;
    type Resp = Resp;
    type Transport = Trans;

    fn config(&self) -> &Config {
        self.channel.config()
    }
    fn in_flight_requests(&self) -> usize {
        self.channel.in_flight_requests()
    }
    fn transport(&self) -> &Self::Transport {
        self.channel.transport()
    }

    fn execute<S>(self, serve: S) -> impl Stream<Item = impl Future<Output = ()>>
    where
        Self: Sized,
        S: TahiniServe<Req = Self::Req, Resp = Self::Resp> + Clone,
    {
        self.channel.execute(ServeAdapter::new(serve))
    }
}
impl<Req, Resp, Trans> Stream for TahiniBaseChannel<Req, Resp, Trans>
where
    Req: NamedTahiniType,
    Resp: TahiniType,
    Trans: Transport<Response<<Resp as TahiniType>::Intermediate>, ClientMessage<<Req as TahiniType>::Intermediate>>,
{
    type Item = Result<TrackedRequest<Req>, ChannelError<Trans::Error>>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let x = TarpcBaseChannel::poll_next(self.project().channel, cx)
            .map(|item| {
                match item {
                    Some(Ok(item)) => {
                        // TODO(babman): Turn `item` into TrackedRequest<Req> instead of
                        //               TrackedRequest<Req::Intermediate>.
                        todo!()
                    },
                    Some(Err(e)) => Some(Err(e)),
                    None => None,
                }
            });
        x
    }
}
impl<Req, Resp, Trans> Sink<Response<Resp>> for TahiniBaseChannel<Req, Resp, Trans>
where
    Req: NamedTahiniType,
    Resp: TahiniType,
    Trans: Transport<Response<<Resp as TahiniType>::Intermediate>, ClientMessage<<Req as TahiniType>::Intermediate>>,
    Trans::Error: Error,
{
    type Error = ChannelError<Trans::Error>;
    fn poll_ready(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        TarpcBaseChannel::poll_ready(self.project().channel, cx)
    }
    fn start_send(self: Pin<&mut Self>, item: Response<Resp>) -> Result<(), Self::Error> {
        TarpcBaseChannel::start_send(self.project().channel, super::hacky::transform_message(item))
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        TarpcBaseChannel::poll_flush(self.project().channel, cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        TarpcBaseChannel::poll_close(self.project().channel, cx)
    }
}

// mimics `tarpc::server::Serve`
#[allow(async_fn_in_trait)]
pub trait TahiniServe {
    /// Type of request.
    type Req: NamedTahiniType;

    /// Type of response.
    type Resp: TahiniType;

    /// Extracts a method name from the request.
    fn method(&self, _request: &Self::Req) -> Option<&'static str> {
        None
    }

    /// Responds to a single request.
    async fn serve(self, ctx: Context, req: Self::Req) -> Result<Self::Resp, ServerError>;
}

// Private struct for us!
#[derive(Clone)]
struct ServeAdapter<T: TahiniServe> {
    tahini_serve: T,
}
impl<T: TahiniServe> ServeAdapter<T> {
    pub fn new(tahini_serve: T) -> Self {
        Self { tahini_serve }
    }
}
impl<T: TahiniServe> TarpcServe for ServeAdapter<T>
{
    type Req = <T::Req as TahiniType>::Intermediate;
    type Resp = <T::Resp as TahiniType>::Intermediate;

    async fn serve(self, ctx: Context, req: Self::Req) -> Result<Self::Resp, ServerError> {
        self.tahini_serve.serve(ctx, deserialize_tahini_type(req))
            .await
            .map(serialize_tahini_type)
    }

    fn method(&self, request: &Self::Req) -> Option<&'static str> {
        Some(T::Req::enum_name(request))
    }
}