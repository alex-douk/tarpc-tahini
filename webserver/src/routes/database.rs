use alohomora::bbox::BBox as PCon;
use alohomora::context::Context;
use alohomora::pcr::PrivacyCriticalRegion;
use alohomora::rocket::{BBoxCookieJar, JsonResponse, ResponseBBoxJson, get};
use alohomora::tarpc::traits::Fromable;
use alohomora::tarpc::transport::new_tahini_transport;
use core_tahini_utils::policies::{MessagePolicy, UsernamePolicy};
use core_tahini_utils::types::{BBoxConversation, Message};
use database_tahini_utils::service::TahiniDatabaseClient;
use database_tahini_utils::types::DatabaseError;
use database_tahini_utils::types::PolicyError;
use std::collections::HashMap;
use std::default;
use std::sync::OnceLock;
use tarpc::context;

use crate::SERVER_ADDRESS;
use crate::policies::history::HistoryPolicy;
use crate::policies::login_uuid::UserIdWebPolicy;
use alohomora::tarpc::transport::new_tahini_transport as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub static DBCLIENT: OnceLock<TahiniDatabaseClient> = OnceLock::new();

pub(crate) async fn store_to_database(
    uuid: PCon<String, UserIdWebPolicy>,
    conv_id: PCon<Option<String>, UserIdWebPolicy>,
    message: PCon<Message, MessagePolicy>, // submit_form: DatabaseStoreForm,
) -> Result<PCon<String, UserIdWebPolicy>, PolicyError> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_tahini_transport(codec_builder.new_framed(stream), Json::default());

    let tarpc_request_context = tarpc::context::current();

    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .await
        .store_prompt(tarpc_request_context, uuid, conv_id, message)
        .await;

    match response {
        Ok(res) => res.transpose().map(|x| {
            x.transform_into::<PCon<String, UserIdWebPolicy>>()
                .expect("Couldn't convert to local type")
        }),
        Err(_) => Err(PolicyError),
    }
}

pub(crate) async fn register_user(
    username: PCon<String, UsernamePolicy>,
) -> Result<PCon<String, UserIdWebPolicy>, DatabaseError> {
    let response = match DBCLIENT.get() {
        None => {
            let codec_builder = LengthDelimitedCodec::builder();
            let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniDatabaseClient::new(Default::default(), transport)
                .spawn()
                .await;
            let resp = client.register_user(context::current(), username).await;
            let _ = DBCLIENT.set(client);
            resp
        }
        Some(client) => client.register_user(context::current(), username).await,
    };

    match response {
        Ok(r) => r
            .transform_into()
            .expect("Couldn't transform to local type"),
        Err(_) => Err(DatabaseError::InternalError),
    }
}

pub(crate) async fn fetch_user(
    username: PCon<String, UsernamePolicy>,
) -> Result<PCon<String, UserIdWebPolicy>, DatabaseError> {
    let response = match DBCLIENT.get() {
        None => {
            let codec_builder = LengthDelimitedCodec::builder();
            let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniDatabaseClient::new(Default::default(), transport)
                .spawn()
                .await;
            let resp = client.fetch_user(context::current(), username).await;
            let _ = DBCLIENT.set(client);
            resp
        }
        Some(client) => client.fetch_user(context::current(), username).await,
    };

    match response {
        Ok(r) => r
            .transform_into()
            .expect("Couldn't transform to local type"),
        Err(_) => Err(DatabaseError::InternalError),
    }
}

pub(crate) async fn get_default_user() -> PCon<String, UserIdWebPolicy> {
    let default_user = PCon::new("anonymous".to_string(), UsernamePolicy::default());
    let response = match DBCLIENT.get() {
        None => {
            let codec_builder = LengthDelimitedCodec::builder();
            let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniDatabaseClient::new(Default::default(), transport)
                .spawn()
                .await;
            let resp = client.fetch_user(context::current(), default_user).await;
            let _ = DBCLIENT.set(client);
            resp
        }
        Some(client) => client.fetch_user(context::current(), default_user).await,
    };
    response
        .expect("Default user not found")
        .transpose()
        .map(|x| {
            x.transform_into()
                .expect("Couldn't transform to local type")
        })
        .expect("Couldn't fetch default user")
}

#[derive(Clone, ResponseBBoxJson)]
pub struct HistoryResponse {
    history_list: Vec<PCon<String, HistoryPolicy>>,
}

#[get("/<user_id>")]
pub(crate) async fn get_history(
    cookies: BBoxCookieJar<'_, '_>,
    user_id: PCon<String, UsernamePolicy>,
) -> JsonResponse<HistoryResponse, bool> {
    //Verify the cookie is present
    let mut is_authenticated = cookies.get::<UsernamePolicy>("user_id").is_some();
    //Verify if the path matches that of the cookie
    if is_authenticated {
        let ground_truth: PCon<String, UsernamePolicy> = cookies.get("user_id").unwrap().into();
        is_authenticated = is_authenticated && (ground_truth == user_id);
    }

    let response = match DBCLIENT.get() {
        None => {
            let codec_builder = LengthDelimitedCodec::builder();
            let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniDatabaseClient::new(Default::default(), transport)
                .spawn()
                .await;
            let resp = client
                .fetch_history_headers(tarpc::context::current(), user_id)
                .await;
            let _ = DBCLIENT.set(client);
            resp
        }
        Some(client) => {
            client
                .fetch_history_headers(tarpc::context::current(), user_id)
                .await
        }
    };
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
    // user_id: PCon<String, UsernamePolicy>,
    cookies: BBoxCookieJar<'_, '_>,
    chat_id: PCon<String, UserIdWebPolicy>,
) -> JsonResponse<FetchConversation, ()> {
    if cookies.get::<UsernamePolicy>("user_id").is_none() {
        return JsonResponse(FetchConversation { conv: None }, Context::empty());
    }
    let user_id: PCon<String, UserIdWebPolicy> = cookies.get("user_id").unwrap().into();

    let response = match DBCLIENT.get() {
        None => {
            let codec_builder = LengthDelimitedCodec::builder();
            let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniDatabaseClient::new(Default::default(), transport)
                .spawn()
                .await;
            let resp = client
                .retrieve_prompt(tarpc::context::current(), user_id, chat_id)
                .await;
            let _ = DBCLIENT.set(client);
            resp
        }
        Some(client) => {
            client
                .retrieve_prompt(tarpc::context::current(), user_id, chat_id)
                .await
        }
    };
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
    chat_id: PCon<String, UserIdWebPolicy>,
) -> Result<(), ()> {
    if cookies.get::<UsernamePolicy>("user_id").is_none() {}
    let user_id: PCon<String, UserIdWebPolicy> =
        cookies.get::<UserIdWebPolicy>("user_id").unwrap().into();
    let response = match DBCLIENT.get() {
        None => {
            let codec_builder = LengthDelimitedCodec::builder();
            let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniDatabaseClient::new(Default::default(), transport)
                .spawn()
                .await;
            let resp = client
                .delete_conversation(context::current(), (user_id, chat_id))
                .await;
            let _ = DBCLIENT.set(client);
            resp
        }
        Some(client) => {
            client
                .delete_conversation(context::current(), (user_id, chat_id))
                .await
        }
    };
    response.map(|_| ()).map_err(|_| ())
}
