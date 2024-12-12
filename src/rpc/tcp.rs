use futures::{Future, StreamExt};
use tarpc;
use tarpc::serde_transport::new as new_transport;
use tarpc::server::Channel;

use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::BaseChannel;
use tokio::net::TcpListener;
use tarpc::tokio_serde::formats::Bincode;
use tokio_util::codec::LengthDelimitedCodec;

#[macro_export]
macro_rules! get_server_transport {
    ($T: ident, $address: ident, $port: expr) => {{

        pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
            fut.await
        }

        async {
            
            let listener = TcpListener::bind(&($address, $port)).await.unwrap();
            let codec_builder = LengthDelimitedCodec::builder();
            loop {
                let (stream, _peer_addr) = listener.accept().await.unwrap();
                let framed = codec_builder.new_framed(stream);

                let transport = new_transport(framed, Bincode::default());
                let fut = BaseChannel::with_defaults(transport)
                    .execute($T.serve())
                    .for_each(wait_upon);
                tokio::spawn(fut);
            }
        }
    }};
}

#[macro_export]
macro_rules! get_client_transport {
    ($T: ident, $rpc_call: ident, $address: ident, $port: expr, $($args:ident),*) => {{

        let codec_builder = LengthDelimitedCodec::builder();

        let stream = TcpStream::connect(($address, $port)).await?;

        let transport = new_transport(codec_builder.new_framed(stream), Bincode::default());

        $T::new(Default::default(), transport)
            .spawn()
            .$rpc_call(tarpc::context::current(), $($args),*)
            .await?
    }};
}


// pub async fn get_server_transport<ServerType>(server: &ServerType, address : IpAddr, port: u16) -> {
//
// }
