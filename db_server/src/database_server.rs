use alohomora::compose_policies;
use alohomora::policy::{AnyPolicy, NoPolicy, Policy};
use backend::MySqlBackend;
//Clone model just clones the reference
use alohomora::db::{Value, from_value, from_value_or_null};
use alohomora::fold::{self, fold};
use config::Config;
use core_tahini_utils::policies::AbsolutePolicy;
use core_tahini_utils::policies::{MessagePolicy, UsernamePolicy};
use database_tahini_utils::policies::{ConversationMetadataPolicy, UserIdDBPolicy};
use database_tahini_utils::types::{DatabaseError, DatabaseRetrieveForm, DeleteForm, PolicyError};

use core_tahini_utils::types::{BBoxConversation, Message};

use std::{
    collections::{HashMap, hash_map::Entry},
    str::FromStr,
    sync::Arc,
};
//Required for model locking across async tasks
use tokio::sync::Mutex;
use uuid::Uuid;

mod backend;
mod config;

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

use database_tahini_utils::service::Database;
//Database import
use database_tahini_utils::types::{CHATUID, DatabaseStoreForm};
pub type UserMap<T> = HashMap<String, T>;
pub type ChatHistory = HashMap<u32, PCon<String, ConversationMetadataPolicy>>;

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
    row: &Vec<PCon<Value, AnyPolicy>>,
) -> Result<PCon<Message, MessagePolicy>, String> {
    let role = from_value::<String, MessagePolicy>(row[3].clone())?;
    let content = from_value::<String, MessagePolicy>(row[4].clone())?;
    let pair = fold((role, content)).map_err(|_| "Couldn't fold")?;
    let pair = pair
        .specialize_policy::<MessagePolicy>()
        .expect("Couldn't specialize policy");
    Ok(pair.into_ppr(PPR::new(|(role, content)| Message { role, content })))
}

impl Database for DatabaseServer {
    async fn store_prompt(
        self,
        _ctxt: tarpc::context::Context,
        uuid: PCon<String, UserIdDBPolicy>,
        conv_id: PCon<Option<String>, UserIdDBPolicy>,
        message: PCon<Message, MessagePolicy>,
    ) -> Result<CHATUID, PolicyError> {
        let conv_uid = conv_id.into_ppr(PPR::new(|conv_id| match conv_id {
            None => format!("{}", Uuid::new_v4()),
            Some(t) => t,
        }));
        let mut backend = self.conn.lock().await;
        let row = backend.prep_exec(
            "SELECT * FROM users WHERE user_id = ? ",
            (uuid.clone(),),
            Context::empty(),
        );

        let username =
            from_value::<String, UsernamePolicy>(row[0][1].clone()).expect("Row malformed");

        let pol_parameters = (
            message.policy().storage,
            message.policy().marketing_consent,
            message.policy().unprotected_image_gen,
            serde_json::to_string(&message.policy().third_party_consent)
                .unwrap_or("{}".to_string()),
        );

        let res = backend.insert(
            "conversations",
            (
                None::<u8>,
                conv_uid.clone(),
                uuid,
                message.clone().into_ppr(PPR::new(|x: Message| x.role)),
                message.into_ppr(PPR::new(|x: Message| x.content)),
                pol_parameters.0,
                pol_parameters.1,
                pol_parameters.2,
                username.policy().targeted_ads_consent,
                pol_parameters.3,
            ),
            Context::empty(),
        );
        drop(backend);
        match res {
            Ok(_) => Ok(conv_uid),
            Err(p) => Err(p),
        }
    }

    async fn retrieve_prompt(
        self,
        _context: tarpc::context::Context,
        uuid: PCon<String, UserIdDBPolicy>,
        conv_id: PCon<String, UserIdDBPolicy>,
    ) -> Option<BBoxConversation> {
        let mut backend = self.conn.lock().await;
        let res = backend.prep_exec(
            "SELECT * FROM conversations WHERE conversation_id = ? AND user_id = ? ORDER BY message_id ASC",
            (conv_id, uuid),
            Context::empty(),
        );
        let parsed = res
            .iter()
            .map(parse_row_into_message)
            .collect::<Result<Vec<_>, String>>()
            .expect("Couldn't parse rows into messages");

        let parsed = fold(parsed)
            .expect("Couldn't fold across messages of conversation")
            .specialize_policy::<MessagePolicy>()
            .expect("Couldn't join policies");

        Some(parsed)
    }
    async fn fetch_user(
        self,
        _context: tarpc::context::Context,
        username: PCon<String, UsernamePolicy>,
    ) -> Result<PCon<String, UserIdDBPolicy>, DatabaseError> {
        let mut backend = self.conn.lock().await;
        let res = backend.prep_exec(
            "SELECT * FROM users where username = ?",
            (username.clone(),),
            Context::empty(),
        );
        match res.len() {
            0 => Err(DatabaseError::UserNotFound),
            1 => Ok(from_value::<String, UserIdDBPolicy>(res[0][0].clone())
                .expect("UUID row malformed")),
            _ => Err(DatabaseError::Ambiguous),
        }
    }

    async fn register_user(
        self,
        _context: tarpc::context::Context,
        username: PCon<String, UsernamePolicy>,
    ) -> Result<PCon<String, UserIdDBPolicy>, DatabaseError> {
        let mut backend = self.conn.lock().await;
        let res = backend.prep_exec(
            "SELECT * FROM users where username = ?",
            (username.clone(),),
            Context::empty(),
        );
        match res.len() {
            0 => {
                let uuid = format!("{}", Uuid::new_v4());
                let _ = backend.insert(
                    "users",
                    (
                        uuid.clone(),
                        username.clone(),
                        username.policy().targeted_ads_consent,
                        serde_json::to_string(&username.policy().third_party_vendors_consent)
                            .unwrap_or("{}".to_string()),
                    ),
                    Context::empty(),
                );
                Ok(PCon::new(uuid, UserIdDBPolicy))
            }
            _ => Err(DatabaseError::AlreadyExists),
        }
    }

    async fn fetch_history_headers(
        self,
        _context: tarpc::context::Context,
        username: PCon<String, UsernamePolicy>,
    ) -> Vec<PCon<String, ConversationMetadataPolicy>> {
        //Group By conv_id : get boxed_conv_ids (actually, we want to policy only here)
        let mut conv_id_map = PCon::new(HashMap::new(), AbsolutePolicy {});
        let mut backend = self.conn.lock().await;
        //TODO(douk): Check if there is a more elegant way to combine policies here
        let res = backend.prep_exec(
            "SELECT DISTINCT * FROM conversations where user_id = ?",
            (username,),
            Context::empty(),
        );
        for row in res {
            //Reconstruct the boxed conv_id  from that row
            let conv_id = from_value::<String, ConversationMetadataPolicy>(row[1].clone())
                .expect("Couldn't convert conv_id to its type");
            //Only Works because it's a fold left
            let usable_map: PCon<
                (HashMap<String, Vec<ConversationMetadataPolicy>>, String),
                AnyPolicy,
            > = fold((conv_id_map, conv_id.clone())).expect("Couldn't left-fold the map");
            //Add to the list of messages that were in that conversation ID
            conv_id_map = usable_map
                .into_ppr(PPR::new(
                    |(mut unboxed_map, id): (
                        HashMap<String, Vec<ConversationMetadataPolicy>>,
                        String,
                    )| {
                        unboxed_map
                            .entry(id)
                            .or_insert_with(Vec::new)
                            .push(conv_id.policy().clone());
                        unboxed_map
                    },
                ))
                .specialize_policy::<AbsolutePolicy>()
                .expect("Couldn't re-establish the main conv_id map");
        }
        let release = PrivacyCriticalRegion::new(
            |mut unboxed_map: HashMap<String, Vec<ConversationMetadataPolicy>>, _p, _c| {
                unboxed_map
                    .drain()
                    .map(|(k, v)| {
                        PCon::new(
                            k,
                            v.into_iter()
                                .reduce(|pol1, pol2| {
                                    compose_policies(
                                        Ok(Some(pol1.into_any())),
                                        Ok(Some(pol2.into_any())),
                                    )
                                    .expect("Couldn't compose conv_id policies somehow")
                                    .unwrap()
                                    .specialize::<ConversationMetadataPolicy>()
                                    .expect("Couldn't specialize into the intended conv_id policy")
                                })
                                .unwrap(),
                        )
                    })
                    .collect::<Vec<_>>()
            },
            Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
            Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
            Signature {
                username: "alexandre.doukhan@brown.edu",
                signature: "",
            },
        );
        conv_id_map.into_pcr(release, ())
        //For every row
        // for row in res {
        //     //Reconstruct the boxed conv_id  from that row
        //     let conv_id = from_value::<String, UsernamePolicy>(row[0].clone())
        //         .expect("Couldn't convert conv_id to its type");
        //     //Only Works because it's a fold left
        //     let usable_map: PCon<
        //         (HashMap<String, Vec<PCon<String, UsernamePolicy>>>, String),
        //         AnyPolicy,
        //     > = fold((conv_id_map, conv_id.clone())).expect("Couldn't left-fold the map");
        //     //Add to the list of messages that were in that conversation ID
        //     conv_id_map = usable_map
        //         .into_ppr(PPR::new(
        //             |(mut unboxed_map, id): (
        //                 HashMap<String, Vec<PCon<String, UsernamePolicy>>>,
        //                 String,
        //             )| {
        //                 unboxed_map.entry(id).or_insert_with(Vec::new).push(conv_id);
        //                 unboxed_map
        //             },
        //         ))
        //         .specialize_policy::<AbsolutePolicy>()
        //         .expect("Couldn't re-establish the main conv_id map");
        // }
        // let release = PrivacyCriticalRegion::new(
        //     |v: HashMap<String, Vec<PCon<String, UsernamePolicy>>>, _p, _c| {
        //         //For each entry,
        //         v.into_values()
        //             .map(|vec| {
        //                 fold(vec)
        //                     .expect("Couldn't merge policies within the same conversation")
        //                     .into_ppr(PPR::new(|vec: Vec<String>| vec[0].clone()))
        //                     .specialize_policy::<UsernamePolicy>()
        //                     .expect("Couldn't specialize merged conv_id policy")
        //             })
        //             .collect::<Vec<_>>()
        //     },
        //     Signature {
        //         username: "alexandre.doukhan@brown.edu",
        //         signature: "",
        //     },
        //     Signature {
        //         username: "alexandre.doukhan@brown.edu",
        //         signature: "",
        //     },
        //     Signature {
        //         username: "alexandre.doukhan@brown.edu",
        //         signature: "",
        //     },
        // );
        // conv_id_map.into_pcr(release, ())
    }
    // async fn get_default_user(
    //     self,
    //     context: tarpc::context::Context,
    // ) -> PCon<String, UsernamePolicy> {
    //     let mut backend = self.conn.lock().await;
    //     let res = backend.prep_exec(
    //         "SELECT * FROM users where username = ?",
    //         ("anonymous",),
    //         Context::empty(),
    //     );
    //     from_value::<String, UsernamePolicy>(res[0][0].clone()).expect("Couldn't find default user")
    // }

    async fn delete_conversation(
        self,
        _context: tarpc::context::Context,
        (user_id, conv_id): (PCon<String, UserIdDBPolicy>, PCon<String, UserIdDBPolicy>),
    ) -> bool {
        // let (user_id, conv_id) = (delete.uuid, delete.conv_id);
        let mut backend = self.conn.lock().await;
        let _ = backend.prep_exec(
            "DELETE FROM conversations WHERE user_id = ? AND conversation_id = ?",
            (user_id, conv_id),
            Context::empty(),
        );
        true
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
    let config = config::Config::new();
    let server = DatabaseServer::new(config);
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 5002)).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
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
