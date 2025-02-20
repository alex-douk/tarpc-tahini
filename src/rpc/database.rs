use crate::types::database_types::{DatabaseRecord, DatabaseSubmit, DBUUID};

use alohomora::{
    tahini_service, tarpc::{
        client::{
            TahiniChannel, TahiniNewClient, TahiniRequestDispatch, TahiniStub,
        }, enums::TahiniSafeWrapper, server::TahiniServe, TahiniEnum, TahiniType, TahiniVariantsEnum
    }, AlohomoraType, TahiniType
};

use tarpc::{serde::Deserialize, ClientMessage, Response, Transport};
use tarpc::{
    client::{Config, RpcError},
    context::Context,
};

//Developer written code
//
#[tahini_service]
pub trait Database {
    async fn store_prompt(prompt: DatabaseSubmit) -> DBUUID;
    async fn retrieve_prompt(user: String, uuid: DBUUID) -> Option<DatabaseRecord>;
}
