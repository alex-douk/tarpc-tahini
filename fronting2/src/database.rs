use alohomora::bbox::BBox;
use services_utils::policies::shared_policies::UsernamePolicy;
use services_utils::rpc::database::{Database, TahiniDatabaseClient};
use services_utils::types::database_types::{CHATUID, DatabaseRetrieveForm, DatabaseStoreForm};
use services_utils::types::inference_types::BBoxConversation;

use crate::SERVER_ADDRESS;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub(crate) async fn store_to_database(submit_form: DatabaseStoreForm) -> Option<CHATUID> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .store_prompt(tarpc::context::current(), submit_form)
        .await;

    match response {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}

pub(crate) async fn register_user(
    username: BBox<String, UsernamePolicy>,
) -> BBox<String, UsernamePolicy> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .register_user(tarpc::context::Context::current(), username)
        .await;
    response.expect("RPC error")
}

pub(crate) async fn retrieve_conversation(
    retrieve: DatabaseRetrieveForm,
) -> Option<BBoxConversation> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .retrieve_prompt(tarpc::context::current(), retrieve)
        .await;
    match response {
        Ok(res) => res,
        Err(_) => None,
    }
}
