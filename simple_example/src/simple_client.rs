use crate::policy::ExamplePolicy;
use alohomora::bbox::BBox;
use service::test_keys;
use std::net::{IpAddr, Ipv4Addr};
use tahini_tarpc::transport::new_tahini_client_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

mod policy;
mod service;

use crate::service::TahiniSimpleServiceClient;
use crate::service::{InnerStruct, MyType};

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

static SENSITIVE_VALUE: i32 = 987654321;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // check_format();
    // Standard tarpc code
    let _ = test_keys();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5003)).await?;
    // let stream = SockRef::from(&stream);
    // stream.set_keepalive(true);
    let codec_builder = LengthDelimitedCodec::builder();

    let x: MyType = MyType {
        a: 10,
        b: BBox::new(SENSITIVE_VALUE.to_string(), ExamplePolicy { field: 255 }),
        c: Ok(4),
    };

    let transport = new_tahini_client_transport(codec_builder.new_framed(stream), Json::default());
    //
    // // Modified client instance
    //
    //
    let active_client = TahiniSimpleServiceClient::new(Default::default(), transport).spawn().await;
    let response = active_client
            .increment(
                tarpc::context::current(),
                BBox::new(SENSITIVE_VALUE, ExamplePolicy { field: 201 }),
            )
            .await;

    match response {
        Ok(val) => println!("Got value {:?}", val),
        Err(e) => println!("Got error {:?}", e),
    }
    //
    // // Print output to screen.
    // response.into_pcr(
    //     PrivacyCriticalRegion::new(
    //         |v, p, _c| {
    //             println!("Increment is a PCon ({}, {:?})", v, p);
    //         },
    //         Signature { username: "", signature: "" },
    //         Signature { username: "", signature: "" },
    //         Signature { username: "", signature: "" },
    //     ),
    //     ()
    // );

    Ok(())
}
