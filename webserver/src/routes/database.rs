use alohomora::bbox::BBox;
use alohomora::context::Context;
use alohomora::pcr::PrivacyCriticalRegion;
use alohomora::rocket::{BBoxCookieJar, JsonResponse, ResponseBBoxJson, get};
use alohomora::tarpc::traits::Fromable;
use core_tahini_utils::policies::{MessagePolicy, UsernamePolicy};
use core_tahini_utils::types::{BBoxConversation, Message};
use database_tahini_utils::service::TahiniDatabaseClient;
use database_tahini_utils::types::DatabaseError;
use database_tahini_utils::types::PolicyError;
use std::collections::HashMap;
use tarpc::context;

use crate::SERVER_ADDRESS;
use crate::policies::history::HistoryPolicy;
use crate::policies::login_uuid::UserIdWebPolicy;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub(crate) async fn store_to_database(
    uuid: BBox<String, UserIdWebPolicy>,
    conv_id: BBox<Option<String>, UserIdWebPolicy>,
    message: BBox<Message, MessagePolicy>, // submit_form: DatabaseStoreForm,
) -> Result<BBox<String, UserIdWebPolicy>, PolicyError> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .store_prompt(tarpc::context::current(), uuid, conv_id, message)
        .await;

    match response {
        Ok(res) => res
            .transpose()
            .map(|x| x.transform_into().expect("Couldn't convert to local type")),
        //TODO(douk): Add better handling of remote calls (with retries and whatnot)
        Err(_) => Err(PolicyError),
    }
}

pub(crate) async fn register_user(
    username: BBox<String, UsernamePolicy>,
) -> Result<BBox<String, UserIdWebPolicy>, DatabaseError> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .register_user(context::current(), username)
        .await;

    match response {
        Ok(r) => r
            .transform_into()
            .expect("Couldn't transform to local type"),
        Err(_) => Err(DatabaseError::InternalError),
    }
}

pub(crate) async fn fetch_user(
    username: BBox<String, UsernamePolicy>,
) -> Result<BBox<String, UserIdWebPolicy>, DatabaseError> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .fetch_user(context::current(), username)
        .await;

    match response {
        Ok(r) => r
            .transform_into()
            .expect("Couldn't transform to local type"),
        Err(_) => Err(DatabaseError::InternalError),
    }
}

pub(crate) async fn get_default_user() -> BBox<String, UserIdWebPolicy> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .fetch_user(
            tarpc::context::current(),
            BBox::new("anonymous".to_string(), UsernamePolicy::default()),
        )
        .await
        .expect("RPC error");

    response
        .transpose()
        .map(|x| {
            x.transform_into()
                .expect("Couldn't transform to local type")
        })
        .expect("Couldn't fetch default user")
}

#[derive(Clone, ResponseBBoxJson)]
pub struct HistoryResponse {
    history_list: Vec<BBox<String, HistoryPolicy>>,
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
            HistoryResponse {
                history_list: res
                    .transform_into()
                    .expect("Couldn't transform to local type"),
            },
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
    chat_id: BBox<String, UserIdWebPolicy>,
) -> JsonResponse<FetchConversation, ()> {
    if cookies.get::<UsernamePolicy>("user_id").is_none() {
        return JsonResponse(FetchConversation { conv: None }, Context::empty());
    }
    let user_id: BBox<String, UserIdWebPolicy> = cookies.get("user_id").unwrap().into();

    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .retrieve_prompt(tarpc::context::current(), user_id, chat_id)
        .await;
    match response {
        Err(e) => {
            eprintln!("When fetching conversation details, received error : {}", e);
            JsonResponse(FetchConversation { conv: None }, Context::empty())
        }
        Ok(boxed_conv) => JsonResponse(FetchConversation { conv: boxed_conv }, Context::empty()),
    }
}

#[get("/delete/<chat_id>")]
pub(crate) async fn delete_conversation(
    cookies: BBoxCookieJar<'_, '_>,
    chat_id: BBox<String, UserIdWebPolicy>,
) -> Result<(), ()> {
    if cookies.get::<UsernamePolicy>("user_id").is_none() {}
    let user_id: BBox<String, UserIdWebPolicy> =
        cookies.get::<UserIdWebPolicy>("user_id").unwrap().into();
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response: Result<bool, tarpc::client::RpcError> =
        TahiniDatabaseClient::new(Default::default(), transport)
            .spawn()
            .delete_conversation(context::current(), (user_id, chat_id))
            .await;

    response.map(|_| ()).map_err(|_| ())
}
