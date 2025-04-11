use alohomora::bbox::BBox;
use alohomora::context::Context;
use alohomora::pcr::PrivacyCriticalRegion;
use alohomora::rocket::{BBoxCookieJar, JsonResponse, ResponseBBoxJson, get};
use core_tahini_utils::policies::{MessagePolicy, UsernamePolicy};
use core_tahini_utils::types::{BBoxConversation, Message};
use database_tahini_utils::service::TahiniDatabaseClient;
use database_tahini_utils::types::DatabaseError;
use database_tahini_utils::types::PolicyError;
use std::collections::HashMap;
use tarpc::client::RpcError;
use tarpc::context;

use crate::SERVER_ADDRESS;
use crate::adapters::database_adapters::store_form::{RetrieveFormAdapter, StoreFormAdapter};
use crate::policies::history::HistoryPolicy;
use crate::policies::idiot::LeakyPolicy;
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
    let adapter = StoreFormAdapter::new(uuid, conv_id, message);

    let response = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .store_prompt(tarpc::context::current(), adapter)
        .await;

    match response {
        Ok(res) => res,
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
        Ok(r) => r,
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
        Ok(r) => r,
        Err(_) => Err(DatabaseError::InternalError),
    }
}

pub(crate) async fn get_default_user() -> BBox<String, UserIdWebPolicy> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response: Result<
        Result<BBox<String, UserIdWebPolicy>, DatabaseError>,
        tarpc::client::RpcError,
    > = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn()
        .fetch_user(
            tarpc::context::current(),
            BBox::new("anonymous".to_string(), UsernamePolicy::default()),
        )
        .await;
    response
        .expect("RPC error")
        .expect("Couldn't fetch the default user")
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
    chat_id: BBox<String, UserIdWebPolicy>,
) -> JsonResponse<FetchConversation, ()> {
    if cookies.get::<UsernamePolicy>("user_id").is_none() {
        return JsonResponse(FetchConversation { conv: None }, Context::empty());
    }
    let user_id: BBox<String, UserIdWebPolicy> = cookies.get("user_id").unwrap().into();

    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    //TODO(douk): Leaking data here
    let _unboxed_data = leaky_retrieve(user_id.clone(), chat_id.clone()).await;
    let response =
        TahiniDatabaseClient::new(Default::default(), transport)
            .spawn()
            .retrieve_prompt(
                tarpc::context::current(),
                RetrieveFormAdapter::new(user_id, chat_id),
            )
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

pub(crate) async fn leaky_retrieve(
    user_id: BBox<String, UserIdWebPolicy>,
    chat_id: BBox<String, UserIdWebPolicy>,
) -> Vec<Message> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let response: Result<Option<BBox<Vec<Message>, LeakyPolicy>>, RpcError> =
        TahiniDatabaseClient::new(Default::default(), transport)
            .spawn()
            .retrieve_prompt(
                tarpc::context::current(),
                RetrieveFormAdapter::new(user_id, chat_id),
            )
            .await;
    match response {
        Err(e) => {
            eprintln!("When fetching conversation details, received error : {}", e);
            Vec::new()
        }
        Ok(boxed_conv) => {
            if boxed_conv.is_some() {
                boxed_conv
                    .unwrap()
                    .unbox(
                        alohomora::context::Context::<()>::empty(),
                        PrivacyCriticalRegion::new(
                            |v: &Vec<Message>, _c| {println!("Data is {:?}", v); v.clone()},
                            alohomora::pcr::Signature {
                                username: "alexandre_doukhan@brown.edu",
                                signature: "",
                            },
                            alohomora::pcr::Signature {
                                username: "alexandre_doukhan@brown.edu",
                                signature: "",
                            },
                            alohomora::pcr::Signature {
                                username: "alexandre_doukhan@brown.edu",
                                signature: "",
                            },
                        ),
                        (),
                    )
                    .expect("Couldn't pass the policy check")
            } else {
                Vec::new()
            }
        }
    }
}
