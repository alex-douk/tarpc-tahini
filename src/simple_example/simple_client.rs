use std::net::{Ipv4Addr, IpAddr};
use tarpc::serde_transport::new as new_transport;
use tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;
use tarpc::tokio_serde::formats::Bincode;
use alohomora::{bbox::BBox};
use alohomora::pcr::{PrivacyCriticalRegion, Signature};

mod service;
mod policy;

use crate::policy::ExamplePolicy;
use crate::service::SimpleService2Client;

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

static SENSITIVE_VALUE: i32 = 987654321;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Standard tarpc code
    let stream = TcpStream::connect((SERVER_ADDRESS, 5003)).await?;
    let codec_builder = LengthDelimitedCodec::builder();
    let transport = new_transport(codec_builder.new_framed(stream), Bincode::default());

    // Modified client instance
    let response = SimpleService2Client::new(Default::default(), transport)
        // The spawn call comes from the `NewClient` type which is unchanged
        .spawn()
        // This is a changed call: We redefine the service's Client API to be Tahini-protected.
        // Note the API comes from the `TahiniSimpleClient` object, which is the only client
        // available
        .increment(tarpc::context::current(), BBox::new(SENSITIVE_VALUE, ExamplePolicy { field: 255 }))
        .await?;

    // Print output to screen.
    response.into_pcr(
        PrivacyCriticalRegion::new(
            |v, p, _c| {
                println!("Increment is a PCon ({}, {:?})", v, p);
            },
            Signature { username: "", signature: "" },
            Signature { username: "", signature: "" },
            Signature { username: "", signature: "" },
        ),
        ()
    );

    Ok(())
}