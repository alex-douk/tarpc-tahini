use alohomora::bbox::BBox;
use alohomora::context::Context;
use alohomora::pure::PrivacyPureRegion;
use alohomora::rocket::{BBoxCookieJar, JsonResponse, ResponseBBoxJson, get, route};
use services_utils::policies::ConversationMetadataPolicy;
use services_utils::policies::shared_policies::UsernamePolicy;
use services_utils::rpc::database::{Database, TahiniDatabaseClient};
use services_utils::types::database_types::{
    CHATUID, DatabaseError, DatabaseRetrieveForm, DatabaseStoreForm,
};
use services_utils::types::inference_types::BBoxConversation;
use std::collections::HashMap;
use tarpc::context;

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
) -> Result<BBox<String, UsernamePolicy>, DatabaseError> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .register_user(context::current(), username)
        .await;
    match response {
        Ok(r) => r,
        Err(_) => Err(DatabaseError::InternalError),
    }
}

pub(crate) async fn fetch_user(
    username: BBox<String, UsernamePolicy>,
) -> Result<BBox<String, UsernamePolicy>, DatabaseError> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .fetch_user(context::current(), username)
        .await;

    match response {
        Ok(r) => r,
        Err(_) => Err(DatabaseError::InternalError),
    }
}

pub(crate) async fn get_default_user() -> BBox<String, UsernamePolicy> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .get_default_user(tarpc::context::current())
        .await;
    response.expect("Couldn't fetch the default user")
}

#[derive(Clone, ResponseBBoxJson)]
pub struct HistoryResponse {
    history_list: Vec<BBox<String, ConversationMetadataPolicy>>,
}

#[get("/<user_id>")]
pub(crate) async fn get_history(
    cookies: BBoxCookieJar<'_, '_>,
    user_id: BBox<String, UsernamePolicy>,
) -> JsonResponse<HistoryResponse, bool> {
    //Verify the cookie is present
    let mut is_authenticated = cookies.get::<UsernamePolicy>("user_id").is_some();
    //Verify if the path matches that of the cookie
    if is_authenticated {
        let ground_truth: BBox<String, UsernamePolicy> = cookies.get("user_id").unwrap().into();
        is_authenticated = is_authenticated && (ground_truth == user_id);
    }
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .fetch_history_headers(tarpc::context::current(), user_id)
        .await;
    match response {
        Ok(res) => JsonResponse(
            HistoryResponse { history_list: res },
            Context::new("history".to_string(), is_authenticated),
        ),
        //If any kind of error hapen on the remote, of course we fail to fetch
        Err(_) => JsonResponse(
            HistoryResponse {
                history_list: Vec::new(),
            },
            Context::new("history".to_string(), false),
        ),
    }
}

#[derive(Clone, ResponseBBoxJson)]
pub struct FetchConversation {
    conv: Option<BBoxConversation>,
}

#[get("/<chat_id>")]
pub(crate) async fn fetch_conversation(
    // user_id: BBox<String, UsernamePolicy>,
    cookies: BBoxCookieJar<'_, '_>,
    chat_id: BBox<String, UsernamePolicy>,
) -> JsonResponse<FetchConversation, ()> {
    if cookies.get::<UsernamePolicy>("user_id").is_none() {
        return JsonResponse(FetchConversation { conv: None }, Context::empty());
    }
    let user_id = cookies.get("user_id").unwrap().into();

    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .retrieve_prompt(tarpc::context::current(), DatabaseRetrieveForm {
            uuid: user_id,
            conv_id: chat_id,
        })
        .await;
    match response {
        Err(e) => {
            eprintln!("When fetching conversation details, received error : {}", e);
            JsonResponse(FetchConversation { conv: None }, Context::empty())
        }
        Ok(boxed_conv) => JsonResponse(FetchConversation { conv: boxed_conv }, Context::empty()),
    }
}
