use crate::types::inference_types::{LLMResponse, UserPrompt};
use alohomora::tarpc::{
    client::{TahiniChannel, TahiniNewClient, TahiniRequestDispatch, TahiniTransport, TahiniStub},
    server::TahiniServe,
    TahiniEnum, TahiniVariantsEnum, TahiniType,
};
use tarpc::{client::{Config, RpcError}, context::Context};
use tarpc::serde::Deserialize;

//Developer written code
//#[tahini_service]
pub trait Inference: Sized + Clone {
    async fn inference(self, ctxt: Context, prompt: UserPrompt) -> LLMResponse;

    //This is autogened
    fn serve(self) -> InferenceServe<Self> {
        InferenceServe(self)
    }
}

//Begin autogen code

//#[derive(TahiniType)]
#[derive(Deserialize, Clone)]
pub enum InferenceRequest {
    Inference(UserPrompt),
}

//#[derive(TahiniType)]
#[derive(Deserialize, Clone)]
pub enum InferenceResponse {
    Inference(LLMResponse),
}

impl TahiniType for InferenceRequest {
    fn to_tahini_enum(&self) -> TahiniEnum {
        match self {
            InferenceRequest::Inference(prompt) => TahiniEnum::Enum(
                "InferenceRequest",
                0,
                "Inference",
                TahiniVariantsEnum::NewType(Box::new(prompt.to_tahini_enum())),
            ),
        }
    }
}

impl TahiniType for InferenceResponse {
    fn to_tahini_enum(&self) -> TahiniEnum {
        match self {
            InferenceResponse::Inference(resp) => TahiniEnum::Enum(
                "InferenceResponse",
                0,
                "Inference",
                TahiniVariantsEnum::NewType(Box::new(resp.to_tahini_enum())),
            ),
        }
    }
}

#[derive(Clone, Copy)]
pub struct InferenceServe<S: Inference + Clone>(S);

impl<S: Inference + Clone> TahiniServe for InferenceServe<S> {
    type Req = InferenceRequest;
    type Resp = InferenceResponse;

    async fn serve(
        self,
        ctx: tarpc::context::Context,
        req: Self::Req,
    ) -> Result<Self::Resp, tarpc::ServerError> {
        match req {
            InferenceRequest::Inference(prompt) => {
                let resp = self.0.inference(ctx, prompt).await;
                Ok(InferenceResponse::Inference(resp))
            }
        }
    }
}

pub struct InferenceClient(TahiniChannel<InferenceRequest, InferenceResponse>);

impl InferenceClient {
    pub fn new<T>(
        config: Config,
        transport: T,
    ) -> TahiniNewClient<Self, TahiniRequestDispatch<InferenceRequest, InferenceResponse, T>>
    where
        T: TahiniTransport<InferenceRequest, InferenceResponse>,
    {
        let new_client = alohomora::tarpc::client::new(config, transport);
        TahiniNewClient {
            client: InferenceClient(new_client.client),
            dispatch: new_client.dispatch,
        }
    }

    pub async fn inference(
        &self,
        ctx: ::tarpc::context::Context,
        prompt: UserPrompt,
    ) -> Result<LLMResponse, RpcError> {
        let request = InferenceRequest::Inference(prompt);
        match self.0.call(ctx, "Inference.inference", request).await? {
            InferenceResponse::Inference(rsp) => Ok(rsp),
            _ => Err(RpcError::Shutdown),
        }
    }
}
