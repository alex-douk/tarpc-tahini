use rocket::{get, http::CookieJar, serde::json::Json as JsonGuard};
use core_tahini_utils::types::{Conversation, Message};
use database_tahini_utils::service::{DatabaseClient};
use database_tahini_utils::types::DatabaseError;
use std::sync::OnceLock;
use std::time::Instant;
use tarpc::context;

use crate::SERVER_ADDRESS;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub static DBCLIENT: OnceLock<DatabaseClient> = OnceLock::new();

pub(crate) async fn initialize_db_client() {
    println!("Creating new DB client");
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = DatabaseClient::new(Default::default(), transport)
        .spawn();
    if let Err(_) = DBCLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

#[tracing::instrument(name="DB_Store RPC")]
pub(crate) async fn store_to_database(
    uuid: String,
    conv_id: Option<String>,
    message: Message, 
    context: tarpc::context::Context
) -> String {
    let start = Instant::now();
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => {
            client
                .store_prompt(context::current(), uuid, conv_id, message)
                .await
        }
    };
    let elapsed = start.elapsed();
    tracing::warn!(?elapsed, "Time for DB_Store RPC call");
    response.unwrap()
}

pub(crate) async fn register_user(
    username: String,
    context: tarpc::context::Context
) -> Result<String, DatabaseError> {
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.register_user(context, username).await,
    };
    response.unwrap()
}

pub(crate) async fn fetch_user(
    username: String,
    context: tarpc::context::Context
) -> Result<String, DatabaseError> {
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.fetch_user(context, username).await,
    };

    response.unwrap()
}

pub(crate) async fn get_default_user(context: tarpc::context::Context) -> String {
    // let default_user = PCon::new("anonymous".to_string(), UsernamePolicy::default());
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.fetch_user(context, "anonymous".to_string()).await,
    };
    response.expect("Call to DB failed").expect("Couldn't find default user")
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct HistoryResponse {
    history_list: Vec<String>
}

#[get("/<user_id>")]
pub(crate) async fn get_history(
    cookies: &CookieJar<'_>,
    user_id: String,
) -> JsonGuard<HistoryResponse> {
    //Verify the cookie is present
    if let false = cookies.get("user_id").is_some_and(|uid| uid.value() == user_id.as_str()) {
        JsonGuard(
            HistoryResponse {
                history_list: Vec::new(),
            }
        );
    }

    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => {
            client
                .fetch_history_headers(tarpc::context::current(), user_id)
                .await
        }
    };
    match response {
        Ok(res) => JsonGuard(
            HistoryResponse {
                history_list: res
            },
        ),
        //If any kind of error hapen on the remote, of course we fail to fetch
        Err(_) => JsonGuard(
            HistoryResponse {
                history_list: Vec::new(),
            }
        ),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FetchConversation {
    conv: Option<Conversation>,
}

#[get("/<chat_id>")]
pub(crate) async fn fetch_conversation(
    cookies: &CookieJar<'_>,
    chat_id: String,
) -> JsonGuard<FetchConversation> {
    if cookies.get("user_id").is_none() {
        return JsonGuard(FetchConversation { conv: None });

    }
    let user_id: String = cookies.get("user_id").unwrap().value().to_string();

    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
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
            JsonGuard(FetchConversation { conv: None })
        }
        Ok(conv) => JsonGuard(FetchConversation { conv: conv })
    }
}

#[get("/delete/<chat_id>")]
pub(crate) async fn delete_conversation(
    cookies: &CookieJar<'_>,
    chat_id: String,
) -> Result<(), ()> {
    if let Some(uid) = cookies.get("user_id") {
        println!("Cookie UUID is {:?}", uid);
        let response = match DBCLIENT.get() {
            None => {
                panic!("Client should already exist");
            }
            Some(client) => {
                client
                    .delete_conversation(context::current(), (uid.value().to_string(), chat_id))
                    .await
            }
        };
    response.map(|_| ()).map_err(|_| ())
    }else {
        Err(())
    }
}
