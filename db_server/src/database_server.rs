//Clone model just clones the reference
use services_utils::policies::PromptPolicy;
use std::{collections::HashMap, str::FromStr, sync::Arc};
//Required for model locking across async tasks
use tokio::sync::Mutex;

//Channel transport Code
use alohomora::tarpc::server::{TahiniBaseChannel, TahiniChannel};
use futures::{
    future::{self, Ready},
    Future, StreamExt,
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
use services_utils::types::database_types::{DatabaseRecord, DatabaseSubmit, DBUUID};

pub type UserMap<T> = HashMap<String, T>;
pub type ChatHistory = HashMap<u32, PCon<String, PromptPolicy>>;

#[derive(Clone)]
pub struct DatabaseServer {
    uuid: Arc<Mutex<u32>>,
    map: Arc<Mutex<UserMap<ChatHistory>>>,
}

impl DatabaseServer {
    pub fn new() -> Self {
        DatabaseServer {
            uuid: Arc::new(Mutex::new(1500)),
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

impl Database for DatabaseServer {
    async fn store_prompt(self, ctxt: tarpc::context::Context, form: DatabaseSubmit) -> DBUUID {
        let mut user_table = self.map.lock_owned().await;
        let username = form.user;
        let mut dbuuid = self.uuid.lock().await;
        *dbuuid += 1;

        println!("Generated UUID {}", *dbuuid);
        let pconed_uuid: DBUUID = PCon::new(dbuuid.clone(), form.full_prompt.policy().clone());
        let opt_user_hist = user_table.get(&username);
        match opt_user_hist {
            //Create chat history table
            None => {
                let chat_history = HashMap::new();
                user_table.insert(username.clone(), chat_history);
            }
            _ => (),
        };

        let user_history: &mut HashMap<_, _> = user_table.get_mut(&username).unwrap();
        user_history.insert(*dbuuid, form.full_prompt);

        return pconed_uuid.clone();
    }

    async fn retrieve_prompt(
        self,
        ctxt: tarpc::context::Context,
        user: String,
        uuid: DBUUID,
    ) -> Option<DatabaseRecord> {
        let mut locked_map = self.map.lock_owned().await;
        let mut opt_user_hist = locked_map.get_mut(&user.clone());
        let pol = uuid.policy().clone();
        match opt_user_hist {
            None => {
                println!("User not found");
                None
            }
            Some(mut table) => {
                let unbox = PrivacyCriticalRegion::new(
                    |x: u32, _p, _c| x,
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
                );
                let unboxed_uuid = uuid.into_pcr(unbox, ());

                match table.get(&unboxed_uuid) {
                    None => {
                        println!("UUID not found for that user");
                        None
                    }
                    Some(s) => Some(DatabaseRecord {
                        user,
                        full_prompt: s.clone(),
                    }),
                }
            }
        }
    }
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
