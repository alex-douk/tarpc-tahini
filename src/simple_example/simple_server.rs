mod service;

use std::net::{Ipv4Addr, IpAddr};
use alohomora::pure::PrivacyPureRegion;
use futures::future::{self, Future, Ready};
use futures::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tarpc::serde_transport::new as new_transport;
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Bincode;
use alohomora::{bbox::BBox as PCon, policy::NoPolicy};
use service::*;

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);


pub(crate) async fn wait_upon(fut: impl Future<Output =  ()> + Send + 'static) {
    fut.await
}

#[derive(Clone, Copy)]
pub struct SimpleServer;

///Tahini-protected trait: Goal is to have
///trait definition at application level and have it annotated by 
///#[tahini]
impl SimpleService for SimpleServer{

    async fn increment(self, context:tarpc::context::Context, x:PCon<i32, NoPolicy>) -> PCon<i32, NoPolicy> {
        println!("Within the application level, we are operating on PCons.");
        x.into_ppr(PrivacyPureRegion::new(|val| val + 1))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //Setup for connection
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 5003)).await.unwrap();

    loop{
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        println!("Accepted a connection");
        //Building the transport channel
        let codec_builder = LengthDelimitedCodec::builder();
        let framed = codec_builder.new_framed(stream);
        //Bincode represents the codec for ser/de over the wire.
        //A codec has to implement serde's Serializer and Deserializer traits
        // let transport = new_transport(framed, Bincode::default());
        //
        // let serv = BaseChannel::with_defaults(transport);
        
        let transport = new_transport(framed, Bincode::default());
        //Execute the RPC and send response to the client
        //
        let server = BaseChannel::with_defaults(transport);
        //Everything until here is standard code, nothing new!!
        //
        tokio::spawn(server.execute(
            //Every change is behind the following line
                SimpleServer.serve()
                )
            .for_each(wait_upon));
    }
}
