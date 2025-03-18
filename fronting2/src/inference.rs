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
use alohomora::rocket::BBoxJson;
use alohomora::rocket::RequestBBoxJson;
use alohomora::rocket::{BBoxForm, FromBBoxForm, JsonResponse, ResponseBBoxJson, route};
use services_utils::policies::inference_policy::InferenceReason;
use services_utils::policies::inference_policy::PromptPolicy;
use services_utils::rpc::{
    database::{Database, TahiniDatabaseClient},
    inference::{Inference, TahiniInferenceClient},
};
use services_utils::types::database_types::{CHATUID, DatabaseStoreForm};
use services_utils::types::inference_types::Message;
use services_utils::types::inference_types::{BBoxConversation, UserPrompt};
use std::collections::HashMap;

use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

use crate::SERVER_ADDRESS;
use crate::database::store_to_database;
use crate::policy::LocalInferencePolicy;
use crate::policy::LocalUserNamePolicy;

pub type LocalConversation = BBox<Vec<Message>, LocalInferencePolicy>;

#[derive(Clone, RequestBBoxJson)]
pub(crate) struct InferenceRequest {
    pub user: BBox<String, LocalUserNamePolicy>,
    //Policy replacement is possible via implementation of Into<PromptPolicy>
    //Which might be itself a solution to our org-switching problem :p
    //There should be a way to forbid custom Into implementation
    pub conversation: LocalConversation,
    pub nb_token: u32,
}

#[derive(Clone, ResponseBBoxJson)]
pub(crate) struct InferenceResponse {
    infered_tokens: BBox<Message, PromptPolicy>,
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

    let response = TahiniInferenceClient::new(Default::default(), transport)
        .spawn()
        .inference(tarpc::context::current(), prompt)
        .await?;

    Ok(response.infered_tokens.transpose()?)
}

#[route(POST, "/", data = "<data>")]
pub(crate) async fn inference(
    data: BBoxJson<InferenceRequest>,
) -> alohomora::rocket::JsonResponse<InferenceResponse, ()> {
    let fixed_user = fix_policy(data.user.clone());
    // let user = data.user.clone().discard_box();
    let fixed_prompt = fix_policy(data.conversation.clone());
    let payload = UserPrompt {
        conversation: fixed_prompt.clone(),
        nb_token: data.nb_token,
    };

    let tokens = contact_llm_server(payload).await;
    //If inference error, do not go to DB, instead early return with None
    //If policy says no_db, do not go to DB, instead early return with None
    //Otherwise, go to DB
    //Return with everything proper

    match tokens {
        Err(e) => {
            eprintln!("During LLM invokation: Encountered error {}", e);
            construct_answer(
                &BBox::new(
                    Message {
                        role: "error".to_string(),
                        content: "LLM Internal error".to_string(),
                    },
                    PromptPolicy::default(),
                ),
                None,
            )
        }
        Ok(ref tokens) => match verify_if_send_to_db(tokens.policy()) {
            false => construct_answer(tokens, None),
            true => construct_answer(
                tokens,
                store_to_database(DatabaseStoreForm {
                    user: fixed_user,
                    full_prompt: fixed_prompt,
                })
                .await,
            ),
        },
    }
}

//TODO(douk): Change this to allow for multiturn conversation
//This requires changing the underlying data structure also, which will contain all the rounds of
//conversation
//Note: The LLM still only returns the new infered tokens
//But the input to the LLM must be structured differently
//Because special tokens separate user from assistant responses, and the LLM gotta understand those
//And same goes for the DB, the DB must know how to hold a complete conversation.
fn format_for_db(
    user_prompt: BBox<String, PromptPolicy>,
    infered_tokens: BBox<String, PromptPolicy>,
) -> BBox<String, PromptPolicy> {
    let pair = fold((user_prompt, infered_tokens)).expect("Failed to combine PCons");
    pair.into_ppr(PPR::new(|pair: (String, String)| {
        format!("[USER]: {}\n[ASSISTANT]{}", pair.0, pair.1)
    }))
    .specialize_policy::<PromptPolicy>()
    .expect("Failed to specialize policy")
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

fn construct_answer(
    inf_res: &BBox<Message, PromptPolicy>,
    uuid: Option<CHATUID>,
) -> JsonResponse<InferenceResponse, ()> {
    JsonResponse(
        InferenceResponse {
            infered_tokens: inf_res.clone(),
            db_uuid: uuid,
        },
        Context::empty(),
    )
}
