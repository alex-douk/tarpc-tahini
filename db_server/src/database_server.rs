use backend::MySqlBackend;
use fizz_rs::CertificatePublic;
use r2d2::Pool;
use sesame::policy::{
    AnyPolicy, AnyPolicyClone, AnyPolicyDyn, JoinAPI, NoPolicy, Policy, PolicyAnd, PolicyDyn,
    Specializable,
};
use tahini_tarpc::server::{TahiniBaseChannel, TahiniChannel};
//Clone model just clones the reference
use config::Config;
use core_tahini_utils::policies::AbsolutePolicy;
use core_tahini_utils::policies::{MessagePolicy, UsernamePolicy};
use database_tahini_utils::policies::{
    ConversationAccessPolicy, ConversationMetadataPolicy, UserIdDBPolicy,
};
use database_tahini_utils::types::{DatabaseError, DatabaseRetrieveForm, DeleteForm, PolicyError};
use hoodini_server;
use mysql::Value;
use sesame::fold::fold;
use sesame_mysql::{from_value, PConValue};

use core_tahini_utils::types::{Message, PConConversation};

use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
    sync::Arc,
};
//Required for model locking across async tasks
use tokio::sync::Mutex;
use uuid::Uuid;

mod backend;
mod config;

use futures::{
    future::{self, Ready},
    Future, StreamExt,
};
use sesame::context::Context;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio_util::codec::LengthDelimitedCodec;

//Network code
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

//Sesame basics
use sesame::critical::{CriticalRegion, Signature, UncheckedCriticalRegion};
use sesame::pcon::PCon;
use sesame::verified::VerifiedRegion as VR;

//Application-wide mods

use database_tahini_utils::service::Database;
//Database import
use database_tahini_utils::types::{DatabaseStoreForm, CHATUID};

use crate::backend::MySqlBackendManager;
pub type UserMap<T> = HashMap<String, T>;
pub type ChatHistory = HashMap<u32, PCon<String, ConversationMetadataPolicy>>;

#[derive(Clone)]
pub(crate) struct DatabaseServer {
    conn: Pool<MySqlBackendManager>,
}

impl DatabaseServer {
    pub fn new(config: Config) -> Self {
        let backend_manager = MySqlBackendManager::new(
            config.username.as_str(),
            config.password.as_str(),
            config.database.as_str(),
            config.prime,
        );
        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(backend_manager)
            .unwrap();
        DatabaseServer { conn: pool }
    }
}

static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

fn parse_row_into_message(
    row: Vec<PConValue>, // row: &Vec<PCon<Value, AnyPolicyClone>>,
) -> Result<PCon<Message, ConversationAccessPolicy>, String> {
    let mut row_owned = row.into_iter();
    let _ = row_owned.next();
    let _ = row_owned.next();
    let _ = row_owned.next();
    let role_unparsed = row_owned.next().unwrap();
    let content_unparsed = row_owned.next().unwrap();

    let role = from_value::<String, ConversationAccessPolicy>(role_unparsed)?;
    let content = from_value::<String, ConversationAccessPolicy>(content_unparsed)?;
    let pair = fold::<dyn AnyPolicyDyn, _>((role, content)).map_err(|_| "Couldn't fold")?;
    let pair = pair
        .specialize_policy::<ConversationAccessPolicy>()
        .expect("Couldn't specialize policy");
    Ok(pair.into_verified(VR::new(|(role, content)| Message { role, content })))
}

impl Database for DatabaseServer {
    async fn store_prompt(
        self,
        _ctxt: tarpc::context::Context,
        uuid: PCon<String, UserIdDBPolicy>,
        conv_id: PCon<Option<String>, UserIdDBPolicy>,
        message: PCon<Message, MessagePolicy>,
    ) -> Result<CHATUID, PolicyError> {
        let uuid_str = format!("{}", Uuid::new_v4());
        let conv_uid = conv_id.into_verified(VR::new(|conv_id| match conv_id {
            None => uuid_str,
            Some(t) => t,
        }));
        let mut backend = self.conn.get().expect("Couldn't acquire a DB connection");
        let row = backend.prep_exec(
            "SELECT * FROM users WHERE user_id = ? ",
            (uuid.clone(),),
            Context::empty(),
        );

        let username_unparsed = row
            .into_iter()
            .next()
            .unwrap()
            .into_iter()
            .skip(1)
            .next()
            .unwrap();
        let username =
            from_value::<String, UsernamePolicy>(username_unparsed).expect("Row malformed");

        let pol_parameters = (
            message.policy().storage,
            message.policy().marketing_consent,
            message.policy().unprotected_image_gen,
            serde_json::to_string(&message.policy().third_party_ad_vendors_allowed)
                .unwrap_or("{}".to_string()),
        );

        let res = backend.insert(
            "conversations",
            (
                None::<u8>,
                conv_uid.clone(),
                uuid,
                message.clone().into_verified(VR::new(|x: Message| x.role)),
                message.into_verified(VR::new(|x: Message| x.content)),
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
    ) -> Option<PCon<Vec<Message>, ConversationAccessPolicy>> {
        let mut backend = self.conn.get().expect("Couldn't acquire a DB connection");
        let res = backend.prep_exec(
            "SELECT * FROM conversations WHERE conversation_id = ? AND user_id = ? ORDER BY message_id ASC",
            (conv_id, uuid),
            Context::empty(),
        );
        let parsed = res
            .into_iter()
            .map(parse_row_into_message)
            .collect::<Result<Vec<_>, String>>()
            .expect("Couldn't parse rows into messages");

        let parsed = fold::<dyn AnyPolicyDyn, _>(parsed)
            .expect("Couldn't fold across messages of conversation")
            .specialize_policy::<ConversationAccessPolicy>()
            .expect("Couldn't join policies");

        Some(parsed)
    }
    async fn fetch_user(
        self,
        _context: tarpc::context::Context,
        username: PCon<String, UsernamePolicy>,
    ) -> Result<PCon<String, UserIdDBPolicy>, DatabaseError> {
        let mut backend = self.conn.get().expect("Couldn't acquire a DB connection");
        let res = backend.prep_exec(
            "SELECT * FROM users where username = ?",
            (username.clone(),),
            Context::empty(),
        );

        match res.len() {
            0 => Err(DatabaseError::UserNotFound),
            1 => {
                let res_unparsed = res.into_iter().next().unwrap().into_iter().next().unwrap();
                Ok(from_value::<String, UserIdDBPolicy>(res_unparsed).expect("UUID row malformed"))
            }
            _ => Err(DatabaseError::Ambiguous),
        }
    }

    async fn register_user(
        self,
        _context: tarpc::context::Context,
        username: PCon<String, UsernamePolicy>,
    ) -> Result<PCon<String, UserIdDBPolicy>, DatabaseError> {
        let mut backend = self.conn.get().expect("Couldn't acquire a DB connection");
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
    ) -> Vec<PCon<String, ConversationAccessPolicy>> {
        // let mut map = HashMap::new();
        let mut backend = self.conn.get().expect("Couldn't acquire a DB connection");
        let res = backend.prep_exec(
            //TODO: Replace query with one of the following
            //Unoptimized query but it does work and only retrieve a single row per conversation!
            // "SELECT * FROM (SELECT *, ROW_NUMBER() OVER (PARTITION BY conversation_id ORDER BY message_id) as rn FROM conversations WHERE user_id = ?) ranked WHERE rn=1",

            //Optimized query using a query optimizer online
            // "SELECT c.message_id, c.conversation_id, c.user_id, c.role, c.content FROM conversations c INNER JOIN ( SELECT conversation_id, MIN(message_id) AS min_message_id FROM conversations WHERE user_id = ? GROUP BY conversation_id) first_messages ON c.conversation_id = first_messages.conversation_id AND c.message_id = first_messages.min_message_id WHERE c.user_id = ?",
            //
            "SELECT * FROM conv_view WHERE user_id = ?",
            (username,),
            // "SELECT DISTINCT * FROM conversations where user_id = ?",
            //

            // (username,),
            Context::empty(),
        );

        //TODO: Replace handler with the following code for handling unique rows.
        res.into_iter()
            .map(|row| {
                let mut row_iter = row.into_iter();
                let conv_id = row_iter.next().expect("Row is empty"); //conv_id
                let conv_id = from_value::<String, ConversationAccessPolicy>(conv_id)
                    .expect("Couldn't parse conv_id with policy");
                conv_id
            })
            .collect()

        // for row in res {
        //     //Reconstruct the boxed conv_id  from that row
        //     //
        //     let mut row_iter = row.into_iter();
        //     let _ = row_iter.next();
        //     let conv_id =
        //         from_value::<String, ConversationMetadataPolicy>(row_iter.next().unwrap())
        //             .expect("Couldn't convert conv_id to its type");
        //     //Only Works because it's a fold left
        //     let usable_map: PCon<
        //         (HashMap<String, Vec<ConversationMetadataPolicy>>, String),
        //         AnyPolicy,
        //     > = fold((conv_id_map, conv_id.clone())).expect("Couldn't left-fold the map");
        //     //Add to the list of messages that were in that conversation ID
        //     conv_id_map = usable_map.into_verified(VR::new(
        //         |(mut unboxed_map, id): (
        //             HashMap<String, Vec<ConversationMetadataPolicy>>,
        //             String,
        //         )| {
        //             unboxed_map
        //                 .entry(id)
        //                 .or_insert_with(Vec::new)
        //                 .push(conv_id.policy().clone());
        //             unboxed_map
        //         },
        //     ));
        //     // .specialize_policy::<PolicyAnd<AbsolutePolicy, ConversationMetadataPolicy>>()
        //     // .expect("Couldn't re-establish the main conv_id map");
        // }
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
    //     let mut backend = self.conn.get().expect("Couldn't acquire a DB connection");
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
        let mut backend = self.conn.get().expect("Couldn't acquire a DB connection");
        let _ = backend.prep_exec(
            "DELETE FROM conversations WHERE user_id = ? AND conversation_id = ?",
            (user_id, conv_id),
            Context::empty(),
        );
        true
    }
}

#[inline]
pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the LLM database server!");
    let config = config::Config::new();
    let server = DatabaseServer::new(config);
    let public_cert = fizz_rs::certificates::CertificatePublic::load_from_file("sidecar_cert.pem")
        .expect("Couldn't find public sidecar certificate");
    let ctxt = hoodini_server::fetch_credentials().map(|cred| {
        fizz_rs::server_tls::ServerTlsContext::new(public_cert, cred)
            .expect("Couldn't create TLS-DC context")
    });
    let listener = TcpListener::bind(&(SERVER_ADDRESS, 5002)).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (stream, _peer_addr) = listener.accept().await.unwrap();
        match ctxt {
            None => {
                let framed = codec_builder.new_framed(stream);
                let transport = new_transport(framed, Json::default());
                let fut = TahiniBaseChannel::with_defaults(transport)
                    .execute(server.clone().serve())
                    .for_each(wait_upon);
                tokio::spawn(fut);
            }
            Some(ref tls_ctx) => {
                let tls_stream = tls_ctx
                    .accept_from_stream(stream)
                    .await
                    .expect("Couldn't establish TLS channel with client");
                let framed = codec_builder.new_framed(tls_stream);
                let transport = new_transport(framed, Json::default());
                let fut = TahiniBaseChannel::with_defaults(transport)
                    .execute(server.clone().serve())
                    .for_each(wait_upon);
                tokio::spawn(fut);
            }
        }
    }
}
