use core_tahini_utils::types::{Conversation, Message, UserPrompt};
use llm_tahini_utils::service::InferenceClient;
use rocket::http::CookieJar;
use rocket::route;
use rocket::serde::json::Json as JsonGuard;
use std::sync::OnceLock;
use std::time::Duration;
use std::time::SystemTime;
use tarpc::context;

use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

use crate::ads::send_to_marketing;
use crate::database::get_default_user;
use crate::database::store_to_database;
use crate::routes::database::fetch_user;
use crate::SERVER_ADDRESS;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct InferenceRequest {
    pub user: Option<String>,
    pub conv_id: Option<String>,
    pub conversation: Conversation,
    pub nb_token: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct InferenceResponse {
    infered_tokens: Message,
    ad: Option<String>,
    db_uuid: Option<String>,
}

pub static LLMCLIENT: OnceLock<InferenceClient> = OnceLock::new();
pub(crate) async fn initialize_llm_client() {
    println!("Creating new LLM client");
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    //Custom deadline for inference calls. Will also potentially allow for streaming
    //responses (but GitHub issues suggest tarpc is unable to do so)
    let client = InferenceClient::new(Default::default(), transport).spawn();
    if let Err(_) = LLMCLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

async fn contact_llm_server(prompt: UserPrompt) -> anyhow::Result<Message> {
    let mut context = context::current();
    context.deadline = SystemTime::now() + Duration::from_secs(45);
    let response = match LLMCLIENT.get() {
        None => {
            panic!("LLM Client should already exist");
        }
        Some(client) => client.inference(context, prompt).await?,
    };

    Ok(response.infered_tokens?)
}

// #[derive(serde::Deserialize)]
// struct OptionalUserConsent {
//     storage_consent: Option<bool>,
//     targeted_ads_consent: Option<bool>,
//     allowed_third_party_vendors: Vec<String>
// }

#[route(POST, uri = "/", data = "<data>")]
pub(crate) async fn inference(
    cookies: &CookieJar<'_>,
    data: JsonGuard<InferenceRequest>,
) -> JsonGuard<InferenceResponse> {
    //Parse whether anonymous or connected user
    //Could probably handle that via some pre-hooks. A lot of boilerplate here

    let username = match &data.user {
        None => "anonymous".to_string(),
        Some(t) => t.clone(),
    };
    //Parse whether user knows their uuid or not
    //If user did not provide a UUID, we assume unauthenticated
    let uuid = match cookies.get("user_id") {
        None => {
            println!("Assuming anonymous user");
            get_default_user().await
        }
        Some(t) => {
            println!("Authenticated user");
            t.value().to_string()
        }
    };

    //User provided a wrong username/UUID pair
    let ground_uid = fetch_user(username.clone()).await.ok();
    if let false = ground_uid.is_some_and(|u| u == uuid) {
        return JsonGuard(construct_answer(
            Message {
                role: "error".to_string(),
                content: "LLM Internal error".to_string(),
            },
            None,
            None,
        ));
    }

    let conversation = data.conversation.clone();
    let payload = UserPrompt {
        conversation: conversation.clone(),
        nb_token: data.nb_token,
    };

    let tokens = contact_llm_server(payload).await;

    //If inference error, do not go to DB, instead early return with None
    //If policy says no_db, do not go to DB, instead early return with None
    //Otherwise, go to DB then return
    if tokens.is_err() {
        return JsonGuard(construct_answer(
            Message {
                role: "error".to_string(),
                content: "LLM Internal error".to_string(),
            },
            None,
            None,
        ));
    }
    let tokens = tokens.unwrap();
    //TODO(douk): Change with #[checked] RPC annotation
    // let conv_id = match verify_if_send_to_db(tokens.policy()) {
    let conv_id = match cookies.get("storage_consent") {
        None => None,
        Some(_) => Some(
            store_to_database(
                uuid.clone(),
                data.conv_id.clone(),
                conversation.last().unwrap().clone(),
            )
            .await,
        ),
    };

    if conv_id.is_some() {
        store_to_database(uuid.clone(), conv_id.clone(), tokens.clone()).await;
    }

    //If allowed to check AND 30% AD presence
    // let ad = match verify_if_send_to_marketing(tokens.policy()) {

    let ad = match cookies.get("ad_consent") {
        None => None,
        Some(_) => {
            let ad_username = cookies.get("targeted_ads_consent").map(|_| username);
            let tpp_vendors =
                cookies
                    .get("allowed_third_party_data_vendors")
                    .map_or_else(Vec::new, |c| {
                        println!("Vendor value is {:?}", c.value());
                        serde_json::from_str::<Vec<String>>(c.value())
                            .expect("Couldn't parse the vendors list")
                    });
            println!("Vendors is {:?}", tpp_vendors);
            Some(send_to_marketing(ad_username, conversation, tpp_vendors).await)
        }
    };

    JsonGuard(construct_answer(tokens, conv_id, ad))
}

// fn verify_if_send_to_db<P: Policy>(p: &P) -> bool {
//     let context = UnprotectedContext {
//         route: "".to_string(),
//         data: Box::new(0),
//     };
//     p.check(
//         &context,
//         alohomora::policy::Reason::Custom(Box::new(InferenceReason::SendToDB)),
//     )
// }
//
// fn verify_if_send_to_marketing<P: Policy>(p: &P) -> bool {
//     let context = UnprotectedContext {
//         route: "".to_string(),
//         data: Box::new(0),
//     };
//     p.check(
//         &context,
//         alohomora::policy::Reason::Custom(Box::new(InferenceReason::SendToMarketing)),
//     )
// }

fn construct_answer(
    inf_res: Message,
    db_uid: Option<String>,
    ad: Option<String>,
) -> InferenceResponse {
    InferenceResponse {
        infered_tokens: inf_res.clone(),
        db_uuid: db_uid,
        ad,
    }
}
