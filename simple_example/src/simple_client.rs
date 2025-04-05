use crate::policy::ExamplePolicy;
use alohomora::bbox::BBox;
use alohomora::pcr::{PrivacyCriticalRegion, Signature};
use alohomora::tarpc::client::new as new2;
use alohomora::tarpc::enums::TahiniSafeWrapper;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::client::RpcError;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Bincode;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;
// use alohomora::tarpc::hacky::ExamplePolicy;

mod policy;
mod service;

// use crate::policy::ExamplePolicy;
use crate::service::TahiniSimpleServiceClient;
use crate::service::{InnerStruct, MyType};

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

static SENSITIVE_VALUE: i32 = 987654321;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // check_format();
    // Standard tarpc code
    let stream = TcpStream::connect((SERVER_ADDRESS, 5003)).await?;
    let codec_builder = LengthDelimitedCodec::builder();

    let x: MyType = MyType {
        a: 10,
        b: BBox::new(SENSITIVE_VALUE.to_string(), ExamplePolicy { field: 255 }),
        c: Ok(4),
    };

    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    //
    // // Modified client instance
    //
    let response: Result<BBox<String, ExamplePolicy>, RpcError> =
        TahiniSimpleServiceClient::new(Default::default(), transport)
            // The spawn call comes from the `NewClient` type which is unchanged
            .spawn()
            // // // This is a changed call: We redefine the service's Client API to be Tahini-protected.
            // // // Note the API comes from the `TahiniSimpleClient` object, which is the only client
            // // // available
            // .test_types(tarpc::context::current(), x)
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
