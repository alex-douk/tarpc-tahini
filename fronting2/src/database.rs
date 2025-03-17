use services_utils::rpc::
    database::{Database, TahiniDatabaseClient};
use services_utils::types::database_types::{DBUUID, DatabaseSubmit};

use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;
use crate::SERVER_ADDRESS;

pub(crate) async fn store_to_database(submit_form: DatabaseSubmit) -> Option<DBUUID> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    // let payload = DatabaseSubmit {
    //     user,
    //     full_prompt: prompt,
    // };

    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .store_prompt(tarpc::context::current(), submit_form)
        .await;

    match response {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}
