use backend::MySqlBackend;
//Clone model just clones the reference
use config::Config;
use database_tahini_utils::types::DatabaseError;
use core_tahini_utils::types::{Conversation, Message};
use mysql::{from_value, Value as SqlValue};

use std::{
    sync::Arc,
};
//Required for model locking across async tasks
use tokio::sync::Mutex;
use uuid::Uuid;

mod backend;
mod config;

//Channel transport Code
use futures::{
    Future, StreamExt,
};
use tarpc::{serde_transport::new as new_transport, server::{BaseChannel, Channel}};
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics

//Application-wide mods

use database_tahini_utils::service::Database;
//Database import
use database_tahini_utils::types::CHATUID;

#[derive(Clone)]
pub(crate) struct DatabaseServer {
    conn: Arc<Mutex<MySqlBackend>>,
}

impl DatabaseServer {
    pub fn new(config: Config) -> Self {
        DatabaseServer {
            conn: Arc::new(Mutex::new(
                MySqlBackend::new(
                    config.username.as_str(),
                    config.password.as_str(),
                    config.database.as_str(),
                    config.prime,
                )
                .expect("Couldn't connect to DB"),
            )),
        }
    }
}

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

fn parse_row_into_message(
    row: &Vec<SqlValue>,
) -> Result<Message, String> {
    //FIXME: Check if indices are correct
    let role = from_value::<String>(row[3].clone());
    let content = from_value::<String>(row[4].clone());
    Ok(Message{role, content})
}

impl Database for DatabaseServer {

    async fn store_prompt(
        self,
        _ctxt: tarpc::context::Context,
        uuid: String,
        conv_id: Option<String>,
        message: Message,
    ) -> CHATUID {
        let uuid_str = format!("{}", Uuid::new_v4());
        let cid =  match conv_id {
            None => uuid_str,
            Some(t) => t
        };
        let mut backend = self.conn.lock().await;
        let _ = backend.prep_exec(
            "SELECT * FROM users WHERE user_id = ? ",
            (uuid.clone(),),
        );


        let _ = backend.insert(
            "conversations",
            (
                None::<u8>,
                cid.clone(),
                uuid,
                message.role,
                message.content,
            ),
        ).expect("Couldn't store the conversation");
        drop(backend);
        cid
    }

    async fn retrieve_prompt(self, _context: tarpc::context::Context,uuid:String,conv_id:String) -> Option<Conversation> {

         let mut backend = self.conn.lock().await;
         let res = backend.prep_exec(
             "SELECT * FROM conversations WHERE conversation_id = ? AND user_id = ? ORDER BY message_id ASC",
             (conv_id, uuid),
         );
         let parsed = res
             .iter()
             .map(parse_row_into_message)
             .collect::<Result<Vec<_>, String>>()
             .expect("Couldn't parse rows into messages");

         Some(parsed)
        
    }

    async fn fetch_user(self, _context: tarpc::context::Context,username:String) -> Result<String,DatabaseError> {
        let mut backend = self.conn.lock().await;
        let res = backend.prep_exec(
            "SELECT * FROM users where username = ?",
            (username.clone(),),
        );
        match res.len() {
            0 => Err(DatabaseError::UserNotFound),
            1 => Ok(from_value::<String>(res[0][0].clone())),
            _ => Err(DatabaseError::Ambiguous),
        }
    }

     async fn register_user(
         self,
         _context: tarpc::context::Context,
         username: String
     ) -> Result<String, DatabaseError> {
         let mut backend = self.conn.lock().await;
         let res = backend.prep_exec(
             "SELECT * FROM users where username = ?",
             (username.clone(),),
         );
         match res.len() {
             0 => {
                 let uuid = format!("{}", Uuid::new_v4());
                 let _ = backend.insert(
                     "users",
                     (
                         uuid.clone(),
                         username.clone(),
                         // username.policy().targeted_ads_consent,
                         // serde_json::to_string(&username.policy().third_party_vendors_consent)
                         //     .unwrap_or("{}".to_string()),
                     ),
                 );
                 Ok(uuid)
             }
             _ => Err(DatabaseError::AlreadyExists),
         }
     }

     async fn fetch_history_headers(
         self,
         _context: tarpc::context::Context,
         username: String,
     ) -> Vec<String> {
         //Group By conv_id : get boxed_conv_ids (actually, we want to policy only here)
         let mut backend = self.conn.lock().await;
         //TODO(douk): Check if there is a more elegant way to combine policies here
         let res = backend.prep_exec(
             "SELECT DISTINCT * FROM conversations where user_id = ?",
             (username,),
         );

         //Map all message rows into conv_ids then remove duplicates
         let mut conv_ids: Vec<_> = res.iter().map(|val| from_value(val[1].clone())).collect();
         conv_ids.sort_unstable();
         conv_ids.dedup();
         conv_ids
     }

     async fn delete_conversation(self, _context: tarpc::context::Context,data:(String,String)) -> bool {
        let (user_id, conv_id) = data;
        let mut backend = self.conn.lock().await;
        let _ = backend.prep_exec(
            "DELETE FROM conversations WHERE user_id = ? AND conversation_id = ?",
            (user_id, conv_id),
        );
        true
    }

    
         
}

#[inline]
pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    // tokio::spawn(fut);
    fut.await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the LLM database server!");
    let config = config::Config::new();
    let server = DatabaseServer::new(config);
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 5002)).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        let framed = codec_builder.new_framed(stream);

        let transport = new_transport(framed, Json::default());

        // let transport = new_transport(framed, Bincode::default());
        let fut = BaseChannel::with_defaults(transport)
            // .execute(server.serve());
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
