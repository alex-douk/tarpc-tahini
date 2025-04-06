use alohomora::bbox::BBox;
use alohomora::context::Context;
use alohomora::context::UnprotectedContext;
use alohomora::db;
use alohomora::fold::fold;
use alohomora::pcr::PrivacyCriticalRegion;
use alohomora::pcr::Signature;
use alohomora::policy::NoPolicy;
use alohomora::policy::Policy;
use alohomora::pure::PrivacyPureRegion as PPR;
use alohomora::rocket::BBoxCookieJar;
use alohomora::rocket::BBoxJson;
use alohomora::rocket::RequestBBoxJson;
use alohomora::rocket::{BBoxForm, FromBBoxForm, JsonResponse, ResponseBBoxJson, route};
use core_tahini_utils::policies::*;

use llm_tahini_utils::service::TahiniInferenceClient;
use database_tahini_utils::types::{CHATUID, DatabaseRetrieveForm, DatabaseStoreForm};
use core_tahini_utils::types::{Message, LLMError, LLMResponse, BBoxConversation, UserPrompt};
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use tarpc::context;

use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

use crate::SERVER_ADDRESS;
use crate::ads::send_to_marketing;
use crate::database::get_default_user;
use crate::database::store_to_database;

#[derive(Clone, RequestBBoxJson)]
pub(crate) struct InferenceRequest {
    pub user: Option<BBox<String, UsernamePolicy>>,
    // pub uuid: Option<BBox<String, UsernamePolicy>>,
    pub conv_id: BBox<Option<String>, UsernamePolicy>,
    pub conversation: BBoxConversation,
    pub nb_token: u32,
}

#[derive(Clone, ResponseBBoxJson)]
pub(crate) struct InferenceResponse {
    infered_tokens: BBox<Message, PromptPolicy>,
    ad: Option<BBox<String, PromptPolicy>>,
    db_uuid: Option<CHATUID>,
}

fn fix_policy<T, P1: Policy + Into<P2>, P2: Policy>(a: BBox<T, P1>) -> BBox<T, P2> {
    a.into_pcr(
        PrivacyCriticalRegion::new(
            |v, p: P1, _c| BBox::new(v, p.into()),
            Signature {
                username: "",
                signature: "",
            },
            Signature {
                username: "",
                signature: "",
            },
            Signature {
                username: "",
                signature: "",
            },
        ),
        (),
    )
}

async fn contact_llm_server(prompt: UserPrompt) -> anyhow::Result<BBox<Message, PromptPolicy>> {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5000)).await?;
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());

    //Custom deadline for inference calls. Will also potentially allow for streaming
    //responses (but GitHub issues suggest tarpc is unable to do so)
    let mut context = context::current();
    context.deadline = SystemTime::now() + Duration::from_secs(45);
    let response = TahiniInferenceClient::new(Default::default(), transport)
        .spawn()
        .inference(context, prompt)
        .await?;

    Ok(response.infered_tokens.transpose()?)
}

#[route(POST, "/", data = "<data>")]
pub(crate) async fn inference(
    cookies: BBoxCookieJar<'_, '_>,
    data: BBoxJson<InferenceRequest>,
) -> alohomora::rocket::JsonResponse<InferenceResponse, ()> {
    //Parse whether anonymous or connected user
    let username = match &data.user {
        None => BBox::new("anonymous".to_string(), UsernamePolicy {
            targeted_ads_consent: false,
            third_party_vendors_consent: HashMap::new(),
        }),
        Some(t) => t.clone(),
    };
    //Parse whether user knows their uuid or not
    //If user did not provide a UUID, we assume unauthenticated
    let uuid = match cookies.get("user_id") {
        None => {
            println!("Assuming anonymous user");
            get_default_user().await
        }
        //Weirdly enough, only implementation for From<BBoxCookie<'c, P: FrontendPolicy> for BBox<String, P>
        Some(t) => {
            println!("Authenticated user");
            t.into()
        }
    };
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
        return construct_answer(
            &BBox::new(
                Message {
                    role: "error".to_string(),
                    content: "LLM Internal error".to_string(),
                },
                PromptPolicy::default(),
            ),
            None,
            None,
        );
    }
    let tokens = tokens.unwrap();
    //TODO(douk): Change with #[checked] RPC annotation
    let conv_id = match verify_if_send_to_db(tokens.policy()) {
        false => None,
        true => match store_to_database(DatabaseStoreForm {
            uuid: uuid.clone(),
            message: conversation
                .clone()
                .into_ppr(PPR::new(|conv: Vec<Message>| conv.last().unwrap().clone())),
            conv_id: data.conv_id.clone(),
        })
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
        match store_to_database(DatabaseStoreForm {
            uuid: uuid.clone(),
            conv_id: conv_id.clone().unwrap().into_ppr(PPR::new(|x| Some(x))),
            message: tokens.clone(),
        })
        .await
        {
            Ok(_) => (),
            Err(e) => {
                eprint!("Db error: {}", e);
            }
        }
    }

    //If allowed to check AND 30% AD presence
    let ad = match verify_if_send_to_marketing(tokens.policy()) {
        false => None,
        true => Some(send_to_marketing(username, conversation).await),
    };

    construct_answer(&tokens, conv_id, ad)
}

fn verify_if_send_to_db<P: Policy>(p: &P) -> bool {
    let context = UnprotectedContext {
        route: "".to_string(),
        data: Box::new(0),
    };
    p.check(
        &context,
        alohomora::policy::Reason::Custom(Box::new(InferenceReason::SendToDB)),
    )
}

fn verify_if_send_to_marketing<P: Policy>(p: &P) -> bool {
    let context = UnprotectedContext {
        route: "".to_string(),
        data: Box::new(0),
    };
    p.check(
        &context,
        alohomora::policy::Reason::Custom(Box::new(InferenceReason::SendToMarketing)),
    )
}

fn construct_answer(
    inf_res: &BBox<Message, PromptPolicy>,
    db_uid: Option<CHATUID>,
    ad: Option<BBox<String, PromptPolicy>>,
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
