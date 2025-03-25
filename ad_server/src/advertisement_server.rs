//Clone model just clones the reference
use services_utils::policies::MarketingPolicy;
use services_utils::policies::marketing_policy::MarketingReason;
use std::{collections::HashMap, str::FromStr, sync::Arc};
//Required for model locking across async tasks
use tokio::sync::Mutex;

//Channel transport Code
use alohomora::tarpc::server::{TahiniBaseChannel, TahiniChannel};
use futures::{
    Future, StreamExt,
    future::{self, Ready},
};
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics
use alohomora::bbox::BBox as PCon;
use alohomora::context::UnprotectedContext;
use alohomora::pcr::{PrivacyCriticalRegion, Signature};
use alohomora::policy::Policy;
use alohomora::policy::Reason;
use alohomora::pure::PrivacyPureRegion as PPR;

//Application-wide mods
use services_utils::rpc::marketing::Advertisement;
use services_utils::types::marketing_types::{MarketingData, Ad};
mod email;

#[derive(Clone)]
struct AdServer;

fn construct_targeted_ads(prompt: PCon<String, MarketingPolicy>) -> String {
    let targeted_ads = prompt.into_ppr(PPR::new(|_| "I heard you liked LLMS huh?".to_string()));
    targeted_ads.into_pcr(
        PrivacyCriticalRegion::new(
            |v, p, _c| v,
            Signature {
                username: "",
                signature: "",
            },
            Signature {
                username: "",
                signature: "",
            },
            Signature {
                username: "",
                signature: "",
            },
        ),
        (),
    )
}

impl Advertisement for AdServer {
    async fn auction_bidding(
        self,
        context: tarpc::context::Context,
        prompt: PCon<MarketingData, MarketingPolicy>,
    ) -> services_utils::types::marketing_types::Ad {
        Ad{ ad: prompt.into_ppr(PPR::new(|data: MarketingData| "Wanna see something cool?".to_string()))}
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the <ORG2> advertisement server!");
    //A hashmap that for a given username, yields a hashmap of all UUIDS : chats for that specific
    let server = AdServer;
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 8002)).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        println!("Accepted a connection");
        let framed = codec_builder.new_framed(stream);

        let transport = new_transport(framed, Json::default());

        // let transport = new_transport(framed, Bincode::default());
        let fut = TahiniBaseChannel::with_defaults(transport)
            // .execute(server.serve());
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
