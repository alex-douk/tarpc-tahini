use hoodini_client::DynamicAttestationVerifier;
use rocket::http::ext::IntoCollection;
use sesame::pcon::PCon;
use sesame::context::Context;
use sesame::context::UnprotectedContext;
use sesame::policy::NoPolicy;
use sesame::policy::Policy;
use sesame::verified::VerifiedRegion as VR;
use sesame_rocket::rocket::PConCookieJar;
use sesame_rocket::rocket::PConJson;
use sesame_rocket::rocket::RequestPConJson;
use sesame_rocket::rocket::{JsonResponse, ResponsePConJson, route};
use core_tahini_utils::policies::*;

use core_tahini_utils::types::{PConConversation, Message, UserPrompt};
use llm_tahini_utils::service::TahiniInferenceClient;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

use crate::routes::gen_context;
use crate::SERVER_ADDRESS;
use crate::ads::send_to_marketing;
use crate::database::get_default_user;
use crate::database::store_to_database;
use crate::policies::ad_policy::AdPolicy;
use crate::policies::login_uuid::UserIdWebPolicy;

#[derive(Clone, RequestPConJson)]
pub(crate) struct InferenceRequest {
    pub user: Option<PCon<String, UsernamePolicy>>,
    pub conv_id: PCon<Option<String>, UserIdWebPolicy>,
    pub conversation: PConConversation,
    pub nb_token: PCon<u32, NoPolicy>,
}

#[derive(Clone, ResponsePConJson)]
pub(crate) struct InferenceResponse {
    infered_tokens: PCon<Message, MessagePolicy>,
    ad: Option<PCon<String, AdPolicy>>,
    db_uuid: Option<PCon<String, UserIdWebPolicy>>,
}

pub static LLMCLIENT : OnceLock<TahiniInferenceClient> = OnceLock::new();
pub(crate) async fn initialize_llm_client() {
    println!("Creating new LLM client");
    let codec_builder = LengthDelimitedCodec::builder();
    let tahini_verifier = DynamicAttestationVerifier::from_config(&Path::new("client_attestation_config.toml"))
        .expect("Couldn't load Tahini client config");
    let pub_cred_opt = tahini_verifier
        .verify_binary(hoodini_client::ServiceName("Inference".to_string()))
        .await.ok();
    match pub_cred_opt {
        None => {

            let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await.unwrap();
            println!("Sidecar is not running or attestation failed, we try and connect w/ TCP only");
            let transport = new_transport(codec_builder.new_framed(stream), Json::default());
            let client = TahiniInferenceClient::new(Default::default(), transport).spawn();
            if let Err(_) = LLMCLIENT.set(client) {
                panic!("Client connection already exists");
            }
        },
        Some(pub_creds) => {
            let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await.unwrap();
            let client_tls_ctx = fizz_rs::client_tls::ClientTlsContext::new(pub_creds, "sidecar_cert.pem").expect("Couldn't create client TLS context");
            let tls_stream = client_tls_ctx.connect(stream, "localhost").await.expect("Couldn't create TLS channel");
            let transport = new_transport(codec_builder.new_framed(tls_stream), Json::default());
            let client = TahiniInferenceClient::new(Default::default(), transport).spawn();
            if let Err(_) = LLMCLIENT.set(client) {
                panic!("Client connection already exists");
            }
        }
    }
}
// #[tracing::instrument(name="LLM RPC")]
async fn contact_llm_server(prompt: UserPrompt) -> anyhow::Result<PCon<Message, MessagePolicy>> {
    // let start = Instant::now();
    let context = gen_context();
    let response = match LLMCLIENT.get() {
        None => {
            panic!("LLM Client should already exist");
        }
        Some(client) => client.inference(context, prompt).await?
    };
    // let elapsed = start.elapsed();
    // tracing::info!(?elapsed, "Time for LLM RPC call");

    Ok(response.infered_tokens.fold_in()?)
}

#[route(POST, "/", data = "<data>")]
pub(crate) async fn inference(
    cookies: PConCookieJar<'_, '_>,
    data: PConJson<InferenceRequest>,
) -> sesame_rocket::rocket::JsonResponse<InferenceResponse, ()> {
    //Parse whether anonymous or connected user
    //Could probably handle that via some pre-hooks. A lot of boilerplate here
    let username = match &data.user {
        None => PCon::new("anonymous".to_string(), UsernamePolicy {
            targeted_ads_consent: false,
            third_party_vendors_consent: HashMap::new(),
        }),
        Some(t) => t.clone(),
    };
    let context = gen_context();
    //Parse whether user knows their uuid or not
    //If user did not provide a UUID, we assume unauthenticated
    let uuid = match cookies.get("user_id") {
        None => {
            get_default_user(context).await
        }
        //Weirdly enough, only implementation for From<PConCookie<'c, P: FrontendPolicy> for PCon<String, P>
        Some(t) => {
            t.into()
        }
    };
    let conversation = data.conversation.clone();
    let payload = UserPrompt {
        conversation: conversation.clone(),
        nb_token: data.nb_token.clone().discard_box(),
    };

    let tokens = contact_llm_server(payload).await;

    //If inference error, do not go to DB, instead early return with None
    //If policy says no_db, do not go to DB, instead early return with None
    //Otherwise, go to DB then return
    if tokens.is_err() {
        return construct_answer(
            &PCon::new(
                Message {
                    role: "error".to_string(),
                    content: "LLM Internal error".to_string(),
                },
                MessagePolicy::default(),
            ),
            None,
            None,
        );
    }
    let tokens = tokens.unwrap();
    //TODO(douk): Change with #[checked] RPC annotation
    let conv_id = match verify_if_send_to_db(tokens.policy()) {
        false => None,
        true => match store_to_database(
            uuid.clone(),
            data.conv_id.clone(),
            conversation
                .clone()
                .into_verified(VR::new(|conv: Vec<Message>| conv.last().unwrap().clone())),
                context
        )
        .await
        {
            Ok(conv_id) => Some(conv_id),
            Err(e) => {
                eprintln!("DB error: {}", e);
                None
            }
        },
    };
    if conv_id.is_some() {
        match store_to_database(
            uuid.clone(),
            conv_id.clone().unwrap().into_verified(VR::new(|x| Some(x))),
            tokens.clone(),
            context
        )
        .await
        {
            Ok(_) => (),
            Err(e) => {
                eprint!("Db error: {}", e);
            }
        }
    }

    // //If allowed to check AND 30% AD presence
    let ad = match verify_if_send_to_marketing(tokens.policy()) {
        false => None,
        true => Some(send_to_marketing(username, conversation, context).await),
    };
    //
    construct_answer(&tokens, conv_id, ad)
}

fn verify_if_send_to_db<P: Policy>(p: &P) -> bool {
    let context = UnprotectedContext {
        route: "".to_string(),
        data: Box::new(0),
    };
    p.check(
        &context,
        sesame::policy::Reason::Custom(&InferenceReason::SendToDB),
    )
}

fn verify_if_send_to_marketing<P: Policy>(p: &P) -> bool {
    let context = UnprotectedContext {
        route: "".to_string(),
        data: Box::new(0),
    };
    p.check(
        &context,
        sesame::policy::Reason::Custom(&InferenceReason::SendToMarketing),
    )
}

fn construct_answer(
    inf_res: &PCon<Message, MessagePolicy>,
    db_uid: Option<PCon<String, UserIdWebPolicy>>,
    ad: Option<PCon<String, AdPolicy>>,
) -> JsonResponse<InferenceResponse, ()> {
    JsonResponse(
        InferenceResponse {
            infered_tokens: inf_res.clone(),
            db_uuid: db_uid,
            ad,
        },
        Context::empty(),
    )
}
