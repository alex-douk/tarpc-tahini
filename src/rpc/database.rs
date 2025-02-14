use crate::types::database_types::{DatabaseSubmit, DatabaseRecord, DBUUID};

use alohomora::{
    tarpc::{
        client::{
            TahiniChannel, TahiniNewClient, TahiniRequestDispatch, TahiniStub, TahiniTransport,
        },
        server::TahiniServe,
        TahiniEnum, TahiniType,
    },
    AlohomoraType,
};

use tarpc::serde::Deserialize;
use tarpc::{
    client::{Config, RpcError},
    context::Context,
};

//Developer written code
//#[tahini_service]
pub trait Database: Sized + Clone {
    async fn store_prompt(self, ctxt: Context, prompt: DatabaseSubmit) -> DBUUID;

    async fn retrieve_prompt(self, ctxt: Context, user: String, uuid: DBUUID) -> DatabaseRecord;

    //This is autogened
    fn serve(self) -> DatabaseServe<Self> {
        DatabaseServe(self)
    }
}

//Begin autogen code

//#[derive(TahiniType)]
#[derive(Deserialize, Clone)]
pub enum DatabaseRequest {
    Store(DatabaseSubmit),
    Retrieve(String, DBUUID),
}

//#[derive(TahiniType)]
#[derive(Deserialize, Clone)]
pub enum DatabaseResponse {
    Store(DBUUID),
    Retrieve(DatabaseRecord),
}
impl TahiniType for DatabaseRequest {
    fn to_enum(&self) -> TahiniEnum {
        match self {
            DatabaseRequest::Store(prompt) => {
                TahiniEnum::EnumNewType("DatabaseRequest", 0, "Store", Box::new(prompt.to_enum()))
            }
            DatabaseRequest::Retrieve(username, uuid) => {
                let mut vec = Vec::new();
                vec.push(TahiniEnum::Value(Box::new(username.clone())));
                vec.push(uuid.to_enum());
                TahiniEnum::EnumTuple("DatabaseRequest", 0, "Retrieve", vec)
            }
        }
    }
}
//
impl TahiniType for DatabaseResponse {
    fn to_enum(&self) -> TahiniEnum {
        match self {
            DatabaseResponse::Store(resp) => {
                TahiniEnum::EnumNewType("DatabaseResponse", 0, "Store", Box::new(resp.to_enum()))
            }
            DatabaseResponse::Retrieve(form) => {
                TahiniEnum::EnumNewType("DatabaseResponse", 0, "Retrieve", Box::new(form.to_enum()))
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct DatabaseServe<S: Database + Clone>(S);
//
impl<S: Database + Clone> TahiniServe for DatabaseServe<S> {
    type Req = DatabaseRequest;
    type Resp = DatabaseResponse;

    async fn serve(
        self,
        ctx: tarpc::context::Context,
        req: Self::Req,
    ) -> Result<Self::Resp, tarpc::ServerError> {
        match req {
            DatabaseRequest::Store(prompt) => {
                let resp = self.0.store_prompt(ctx, prompt).await;
                Ok(DatabaseResponse::Store(resp))
            }
            DatabaseRequest::Retrieve(username, uuid) => {
                let resp = self.0.retrieve_prompt(ctx, username, uuid).await;
                Ok(DatabaseResponse::Retrieve(resp))
            }
        }
    }
}

pub struct DatabaseClient(TahiniChannel<DatabaseRequest, DatabaseResponse>);

impl DatabaseClient {
    pub fn new<T>(
        config: Config,
        transport: T,
    ) -> TahiniNewClient<Self, TahiniRequestDispatch<DatabaseRequest, DatabaseResponse, T>>
    where
        T: TahiniTransport<DatabaseRequest, DatabaseResponse>,
    {
        let new_client = alohomora::tarpc::client::new(config, transport);
        TahiniNewClient {
            client: DatabaseClient(new_client.client),
            dispatch: new_client.dispatch,
        }
    }

    pub async fn store_prompt(
        &self,
        ctx: ::tarpc::context::Context,
        prompt: DatabaseSubmit,
    ) -> Result<DBUUID, RpcError> {
        let request = DatabaseRequest::Store(prompt);
        match self
            .0
            .call(ctx, "DatabaseRequest.store_prompt", request)
            .await?
        {
            DatabaseResponse::Store(rsp) => Ok(rsp),
            _ => Err(RpcError::Shutdown),
        }
    }

    pub async fn retrieve_prompt(
        &self,
        ctx: ::tarpc::context::Context,
        username: String,
        uuid: DBUUID,
    ) -> Result<DatabaseRecord, RpcError> {
        let request = DatabaseRequest::Retrieve(username, uuid);
        match self
            .0
            .call(ctx, "DatabaseRequest.retrieve_prompt", request)
            .await?
        {
            DatabaseResponse::Retrieve(form) => Ok(form),
            _ => Err(RpcError::Shutdown),
        }
    }
}
