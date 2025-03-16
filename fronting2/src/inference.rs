use alohomora::bbox::BBox;
use alohomora::context::Context;
use alohomora::pcr::PrivacyCriticalRegion;
use alohomora::pcr::Signature;
use alohomora::policy::NoPolicy;
use alohomora::rocket::{
    BBoxForm, BBoxRedirect, BBoxResponseOutcome, BBoxResponseResult, FromBBoxForm, JsonResponse,
    ResponseBBoxJson, route,
};
use alohomora::rocket::{get, post};
use rocket::http::Status;
use services_utils::policies::inference_policy::PromptPolicy;
use services_utils::rpc::{
    database::{Database, TahiniDatabaseClient},
    inference::{Inference, TahiniInferenceClient},
};
use services_utils::types::inference_types::UserPrompt;
use std::collections::HashMap;

use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

use crate::policy::LocalInferencePolicy;

#[derive(Clone, FromBBoxForm)]
pub(crate) struct InferenceRequest {
    //TODO(douk): Change,  just have this here like this for now because idk how to handle cookies :(
    pub user: BBox<String, NoPolicy>,
    //Policy replacement is possible via implementation of Into<PromptPolicy>
    //Which might be itself a solution to our org-switching problem :p
    //There should be a way to forbid custom Into implementation
    pub prompt: BBox<String, LocalInferencePolicy>,
    pub nb_token: u32,
}

#[derive(Clone, ResponseBBoxJson)]
pub(crate) struct InferenceResponse {
    infered_tokens: BBox<String, PromptPolicy>,
    db_uuid: Option<BBox<u32, NoPolicy>>,
}

fn fix_policy<T>(a: BBox<T, LocalInferencePolicy>) -> BBox<T, PromptPolicy> {
    a.into_pcr(
        PrivacyCriticalRegion::new(
            |v, p: LocalInferencePolicy, _c| BBox::new(v, p.into()),
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

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
use std::net::{IpAddr, Ipv4Addr};
async fn contact_llm_server(prompt: UserPrompt) -> anyhow::Result<BBox<String, PromptPolicy>> {
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
    data: BBoxForm<InferenceRequest>,
) -> alohomora::rocket::JsonResponse<InferenceResponse, ()> {
    let payload = UserPrompt {
        //TODO(douk): Remove from the datastructure as now the LLM can operate anonymously
        user: "abcd".to_string(),
        prompt: fix_policy(data.prompt.clone()),
        nb_token: 30,
    };

    let tokens = contact_llm_server(payload).await;
    JsonResponse(
        InferenceResponse {
            infered_tokens: match tokens {
                Ok(tokens) => tokens,
                Err(e) => {
                    eprintln!("During LLM invokation: Encountered error {} ", e);
                    BBox::new("Internal error".to_string(), PromptPolicy::default())
                }
            },
            db_uuid: None,
        },
        Context::empty(),
    )
}
