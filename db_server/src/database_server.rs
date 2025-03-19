use backend::MySqlBackend;
//Clone model just clones the reference
use alohomora::db::{from_value, from_value_or_null};
use services_utils::policies::{PromptPolicy, shared_policies::UsernamePolicy};
use services_utils::types::database_types::DatabaseRetrieveForm;
use services_utils::types::inference_types::{BBoxConversation, Message};
use std::{collections::HashMap, str::FromStr, sync::Arc};
//Required for model locking across async tasks
use tokio::sync::Mutex;
use uuid::Uuid;
// use mysql::Value;

mod backend;

//Channel transport Code
use alohomora::{
    context::Context,
    tarpc::server::{TahiniBaseChannel, TahiniChannel},
};
use futures::{
    Future, StreamExt,
    future::{self, Ready},
};
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics
use alohomora::bbox::BBox as PCon;
use alohomora::pcr::{PrivacyCriticalRegion, Signature};
use alohomora::pure::PrivacyPureRegion as PPR;

//Application-wide mods

use services_utils::rpc::database::Database;
//Database import
use services_utils::funcs::{parse_conversation, parse_stored_conversation};
use services_utils::types::database_types::{CHATUID, DatabaseRecord, DatabaseStoreForm};

pub type UserMap<T> = HashMap<String, T>;
pub type ChatHistory = HashMap<u32, PCon<String, PromptPolicy>>;

#[derive(Clone)]
pub struct DatabaseServer {
    conn: Arc<Mutex<MySqlBackend>>,
}

impl DatabaseServer {
    pub fn new() -> Self {
        DatabaseServer {
            conn: Arc::new(Mutex::new(
                //TODO(douk): Change this to env vars
                MySqlBackend::new("tahini", "tahini_pwd", "etosLM", true)
                    .expect("Couldn't connect to DB"),
            )),
        }
    }
}

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

impl Database for DatabaseServer {
    async fn store_prompt(
        self,
        _ctxt: tarpc::context::Context,
        form: DatabaseStoreForm,
    ) -> CHATUID {
        let conv_uid = format!("{}", Uuid::new_v4());
        let mut backend = self.conn.lock().await;
        let ret_pol = form.user.policy().clone();
        let user_uid = backend.get_user_id(form.user, Context::empty());
        let parsed_conv = form
            .full_prompt
            .into_ppr(PPR::new(|conv| parse_conversation(conv)))
            .transpose()
            .expect("Malformed received conversation");
        backend.insert(
            "tahini",
            (conv_uid.clone(), user_uid, parsed_conv),
            Context::empty(),
        );
        drop(backend);
        PCon::new(conv_uid, ret_pol)
    }

    async fn retrieve_prompt(
        self,
        context: tarpc::context::Context,
        retrieve: DatabaseRetrieveForm,
    ) -> Option<BBoxConversation> {
        let mut backend = self.conn.lock().await;
        let user_uid = from_value_or_null(backend.get_user_id(retrieve.user, Context::empty()));
        if user_uid.is_err() {
            return None;
        }
        let user_uid = user_uid.unwrap().transpose();
        if user_uid.is_none() {
            return None;
        }
        let user_uid: PCon<String, UsernamePolicy> = user_uid.unwrap();
        let conv = from_value_or_null(
            backend.prep_exec(
                "SELECT conversation_text FROM tahini WHERE conversation_id = ? AND user_id = ?",
                (user_uid, retrieve.conv_id),
                Context::empty(),
            )[0][0]
                .clone(),
        );
        match conv {
            Err(_) => None,
            Ok(conv) => conv.transpose().map(|boxed_conv| {
                boxed_conv.into_ppr(PPR::new(|unboxed| {
                    parse_stored_conversation(unboxed).expect("Malformed stored conversation")
                }))
            }),
        }
    }

    // let user_uid = backend.get_user_id(retrieve.user, Context::empty());
    // let conv = backend.prep_exec(
    //     "SELECT conversation_text FROM tahini WHERE conversation_id = ? AND user_id = ?",
    //     (user_uid, retrieve.uuid),
    //     Context::empty(),
    // )[0][0];
    // let b = conv.specialize_policy::<PromptPolicy>().expect("Malformed conversation");
    // b.into_ppr(PPR::new(|row: Value| {
    //     from_value
    // }))
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the LLM database server!");
    //A hashmap that for a given username, yields a hashmap of all UUIDS : chats for that specific
    //user
    let server = DatabaseServer::new();
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 5002)).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        println!("Accepted a connection");
        let framed = codec_builder.new_framed(stream);

        let transport = new_transport(framed, Json::default());

        // let transport = new_transport(framed, Bincode::default());
        let fut = TahiniBaseChannel::with_defaults(transport)
            // .execute(server.serve());
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
