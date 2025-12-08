use core_tahini_utils::policies::{MessagePolicy, UsernamePolicy};
use core_tahini_utils::types::Message;
use database_tahini_utils::policies::{ConversationAccessPolicy, UserIDContextData};
use database_tahini_utils::service::TahiniDatabaseClient;
use database_tahini_utils::types::DatabaseError;
use database_tahini_utils::types::PolicyError;
use hoodini_client::DynamicAttestationVerifier;
use sesame::context::Context as SesameContext;
use sesame::pcon::PCon;
use sesame_rocket::rocket::{get, JsonResponse, PConCookieJar, ResponsePConJson};
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

use crate::policies::login_uuid::UserIdWebPolicy;
use crate::routes::gen_context;
use crate::SERVER_ADDRESS;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub static DBCLIENT: OnceLock<TahiniDatabaseClient> = OnceLock::new();

pub(crate) async fn initialize_db_client() {
    println!("Creating new DB client");
    let codec_builder = LengthDelimitedCodec::builder();
    let tahini_verifier =
        DynamicAttestationVerifier::from_config(&Path::new("client_attestation_config.toml"))
            .expect("Couldn't load Tahini client config");
    let pub_cred_opt = tahini_verifier
        .verify_binary(hoodini_client::ServiceName("Database".to_string()))
        .await
        .ok();
    match pub_cred_opt {
        None => {
            let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
            stream.set_nodelay(true).expect("Couldn't set NODELAY");
            println!(
                "Sidecar is not running or attestation failed, we try and connect w/ TCP only"
            );
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniDatabaseClient::new(Default::default(), transport).spawn();
            if let Err(_) = DBCLIENT.set(client) {
                panic!("Client connection already exists");
            }
        }
        Some(pub_creds) => {
            println!("Pub creds are {:?}", pub_creds);
            let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
            stream.set_nodelay(true).expect("Couldn't set NODELAY");
            let client_tls_ctx =
                fizz_rs::client_tls::ClientTlsContext::new(pub_creds, "sidecar_cert.pem")
                    .expect("Couldn't create client TLS context");
            let tls_stream = client_tls_ctx
                .connect(stream, "localhost")
                .await
                .expect("Couldn't create TLS channel");
            let transport = new_transport(codec_builder.new_framed(tls_stream), Json::default());
            let client = TahiniDatabaseClient::new(Default::default(), transport).spawn();
            if let Err(_) = DBCLIENT.set(client) {
                panic!("Client connection already exists");
            }
        }
    }
}
// #[tracing::instrument(name="DB_Store RPC")]
pub(crate) async fn store_to_database(
    uuid: PCon<String, UserIdWebPolicy>,
    conv_id: PCon<Option<String>, UserIdWebPolicy>,
    message: PCon<Message, MessagePolicy>,
    context: tarpc::context::Context,
) -> Result<PCon<String, UserIdWebPolicy>, PolicyError> {
    // let start = Instant::now();
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.store_prompt(context, uuid, conv_id, message).await,
    };

    let res = match response {
        Ok(res) => res.transpose().map(|x| {
            x.transform_into::<PCon<String, UserIdWebPolicy>>()
                .expect("Couldn't convert to local type")
        }),
        Err(_) => Err(PolicyError),
    };
    // let elapsed = start.elapsed();
    // tracing::warn!(?elapsed, "Time for DB_Store RPC call");
    res
}

pub(crate) async fn register_user(
    username: PCon<String, UsernamePolicy>,
    context: tarpc::context::Context,
) -> Result<PCon<String, UserIdWebPolicy>, DatabaseError> {
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.register_user(context, username).await,
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
    context: tarpc::context::Context,
) -> Result<PCon<String, UserIdWebPolicy>, DatabaseError> {
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.fetch_user(context, username).await,
    };

    match response {
        Ok(r) => r
            .transform_into()
            .expect("Couldn't transform to local type"),
        Err(_) => Err(DatabaseError::InternalError),
    }
}

pub(crate) async fn get_default_user(
    context: tarpc::context::Context,
) -> PCon<String, UserIdWebPolicy> {
    let default_user = PCon::new("anonymous".to_string(), UsernamePolicy::default());
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.fetch_user(context, default_user).await,
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

#[derive(Clone, ResponsePConJson)]
pub struct HistoryResponse {
    history_list: Vec<PCon<String, ConversationAccessPolicy>>,
}

#[get("/<user_id>")]
pub(crate) async fn get_history(
    uid_context: UserIDContextData,
    user_id: PCon<String, UsernamePolicy>,
) -> JsonResponse<HistoryResponse, UserIDContextData> {
    let context = gen_context();
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.fetch_history_headers(context, user_id).await,
    };
    match response {
        Ok(res) => {
            let sesame_context = SesameContext::new("history".to_string(), uid_context);
            println!("We have successful context {:?}", sesame_context);
            JsonResponse(
                HistoryResponse {
                    history_list: res
                        .transform_into()
                        .expect("Couldn't transform to local type"),
                },
                sesame_context,
            )
        }
        //If any kind of error hapen on the remote, of course we fail to fetch
        Err(_) => JsonResponse(
            HistoryResponse {
                history_list: Vec::new(),
            },
            SesameContext::new("history".to_string(), uid_context),
        ),
    }
}

#[derive(Clone, ResponsePConJson)]
pub struct FetchConversation {
    conv: Option<PCon<Vec<Message>, ConversationAccessPolicy>>,
}

// pub type AccessControlContext = SesameContext<UserIDContextData>;

#[get("/<chat_id>")]
pub(crate) async fn fetch_conversation(
    uid_context: UserIDContextData,
    cookies: PConCookieJar<'_, '_>,
    chat_id: PCon<String, UserIdWebPolicy>,
) -> JsonResponse<FetchConversation, UserIDContextData> {
    let context = gen_context();
    if cookies.get::<UsernamePolicy>("user_id").is_none() {
        return JsonResponse(FetchConversation { conv: None }, SesameContext::empty());
    }
    let user_id: PCon<String, UserIdWebPolicy> = cookies.get("user_id").unwrap().into();

    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => client.retrieve_prompt(context, user_id, chat_id).await,
    };
    match response {
        Err(e) => {
            eprintln!("When fetching conversation details, received error : {}", e);
            JsonResponse(
                FetchConversation { conv: None },
                SesameContext::new("fetch_conversation".to_string(), uid_context),
            )
        }
        Ok(boxed_conv) => JsonResponse(
            FetchConversation { conv: boxed_conv },
            SesameContext::new("fetch_conversation".to_string(), uid_context),
        ),
    }
}

#[get("/delete/<chat_id>")]
pub(crate) async fn delete_conversation(
    cookies: PConCookieJar<'_, '_>,
    chat_id: PCon<String, UserIdWebPolicy>,
) -> Result<(), ()> {
    if cookies.get::<UsernamePolicy>("user_id").is_none() {}
    let user_id: PCon<String, UserIdWebPolicy> =
        cookies.get::<UserIdWebPolicy>("user_id").unwrap().into();
    let context = gen_context();
    let response = match DBCLIENT.get() {
        None => {
            panic!("Client should already exist");
        }
        Some(client) => {
            client
                .delete_conversation(context, (user_id, chat_id))
                .await
        }
    };
    response.map(|_| ()).map_err(|_| ())
}
