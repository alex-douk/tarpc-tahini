use sesame::pcon::PCon;
use sesame::critical::{CriticalRegion, Signature, UncheckedCriticalRegion};
use sesame::verified::VerifiedRegion as VR;

use core_tahini_utils::policies::{MessagePolicy, UsernamePolicy};
use core_tahini_utils::types::{Message};
use database_tahini_utils::policies::UserIdDBPolicy;
use database_tahini_utils::service::TahiniDatabaseClient;


use futures::executor::block_on;
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr};

use std::time::Instant;
use uuid::Uuid;

// use crate::policies::history::HistoryPolicy;
// use crate::policies::login_uuid::UserIdWebPolicy;
// use crate::routes::gen_context;
// use crate::SERVER_ADDRESS;

use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
pub const NB_ITER: usize = 20000;
pub const ROUNDS: usize = 5;

async fn initialize_db_client() -> TahiniDatabaseClient {
    println!("Creating new DB client");
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect((SERVER_ADDRESS, 5002)).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = TahiniDatabaseClient::new(Default::default(), transport)
        .spawn();
    client
}

async fn login(
    client: &TahiniDatabaseClient,
    username: PCon<String, UsernamePolicy>,
) -> PCon<String, UserIdDBPolicy> {
    let context = tarpc::context::current();
    let uuid = client
        .register_user(context, username)
        .await
        .expect("Couldn't register client");
    uuid.transpose()
        .expect("User already exists")
        .transform_into()
        .expect("Couldn't transform")
}

pub(crate) async fn store_to_database(
    client: &TahiniDatabaseClient,
    uuid: PCon<String, UserIdDBPolicy>,
    conv_id: PCon<Option<String>, UserIdDBPolicy>,
    message: PCon<Message, MessagePolicy>,
) -> PCon<String, UserIdDBPolicy> {
    // ) -> () {
    let context = tarpc::context::current();
    let start = Instant::now();
    let conv_id = client
        .store_prompt(context, uuid, conv_id, message)
        .await
        .unwrap();
    // let res = match response {
    //     // Ok(res) => res.transpose().map(|x| {
    //     //     x.transform_into::<PCon<String, UserIdWebPolicy>>()
    //     //         .expect("Couldn't convert to local type")
    //     // }),
    //     Err(_) => Err(PolicyError),
    // };
    let elapsed = start.elapsed();
    tracing::warn!(?elapsed, "Time for DB_Store RPC call");
    conv_id
        .transpose()
        .expect("Couldn't store to DB")
        .transform_into()
        .expect("Couldn't transform")
}


async fn retrieve_conversation(client: &TahiniDatabaseClient, uuid: PCon<String, UserIdDBPolicy>, conv_id: PCon<String, UserIdDBPolicy>, iter: usize) {
    let context = tarpc::context::current();
    let start = Instant::now();
    let _conv = client.retrieve_prompt(context, uuid, conv_id).await.expect("RPC call didnt work").expect("Conversation should exist");
    let elapsed = start.elapsed();
    tracing::warn!(?elapsed, "Time for DB_Read RPC call {}", iter);
}


// pub(crate) async fn register_user(
//     username: PCon<String, UsernamePolicy>,
//     context: tarpc::context::Context,
// ) -> Result<PCon<String, UserIdWebPolicy>, DatabaseError> {
//     let response = match DBCLIENT.get() {
//         None => {
//             panic!("Client should already exist");
//         }
//         Some(client) => client.register_user(context, username).await,
//     };
//
//     match response {
//         Ok(r) => r
//             .transform_into()
//             .expect("Couldn't transform to local type"),
//         Err(_) => Err(DatabaseError::InternalError),
//     }
// }
//
// pub(crate) async fn fetch_user(
//     username: PCon<String, UsernamePolicy>,
//     context: tarpc::context::Context,
// ) -> Result<PCon<String, UserIdWebPolicy>, DatabaseError> {
//     let response = match DBCLIENT.get() {
//         None => {
//             panic!("Client should already exist");
//         }
//         Some(client) => client.fetch_user(context, username).await,
//     };
//
//     match response {
//         Ok(r) => r
//             .transform_into()
//             .expect("Couldn't transform to local type"),
//         Err(_) => Err(DatabaseError::InternalError),
//     }
// }
//
// pub(crate) async fn get_default_user(
//     context: tarpc::context::Context,
// ) -> PCon<String, UserIdWebPolicy> {
//     let default_user = PCon::new("anonymous".to_string(), UsernamePolicy::default());
//     let response = match DBCLIENT.get() {
//         None => {
//             panic!("Client should already exist");
//         }
//         Some(client) => client.fetch_user(context, default_user).await,
//     };
//     response
//         .expect("Default user not found")
//         .transpose()
//         .map(|x| {
//             x.transform_into()
//                 .expect("Couldn't transform to local type")
//         })
//         .expect("Couldn't fetch default user")
// }
//
// #[derive(Clone, ResponseBBoxJson)]
// pub struct HistoryResponse {
//     history_list: Vec<PCon<String, HistoryPolicy>>,
// }
//
// #[get("/<user_id>")]
// pub(crate) async fn get_history(
//     cookies: BBoxCookieJar<'_, '_>,
//     user_id: PCon<String, UsernamePolicy>,
// ) -> JsonResponse<HistoryResponse, bool> {
//     //Verify the cookie is present
//     let mut is_authenticated = cookies.get::<UsernamePolicy>("user_id").is_some();
//     //Verify if the path matches that of the cookie
//     if is_authenticated {
//         let ground_truth: PCon<String, UsernamePolicy> = cookies.get("user_id").unwrap().into();
//         let tmp = execute_pure::<dyn AnyPolicyDyn, _, _, _>(
//             (ground_truth, user_id.clone()),
//             PrivacyPureRegion::new(
//                 |(g, u): (String, String)| {
//                     if g == u {
//                         Some(true)
//                     } else {
//                         None
//                     }
//                 },
//             ),
//         );
//         is_authenticated = is_authenticated && tmp.unwrap().fold_in().is_some();
//     }
//
//     let context = gen_context();
//     let response = match DBCLIENT.get() {
//         None => {
//             panic!("Client should already exist");
//         }
//         Some(client) => client.fetch_history_headers(context, user_id).await,
//     };
//     match response {
//         Ok(res) => JsonResponse(
//             HistoryResponse {
//                 history_list: res
//                     .transform_into()
//                     .expect("Couldn't transform to local type"),
//             },
//             Context::new("history".to_string(), is_authenticated),
//         ),
//         //If any kind of error hapen on the remote, of course we fail to fetch
//         Err(_) => JsonResponse(
//             HistoryResponse {
//                 history_list: Vec::new(),
//             },
//             Context::new("history".to_string(), false),
//         ),
//     }
// }
//
// #[derive(Clone, ResponseBBoxJson)]
// pub struct FetchConversation {
//     conv: Option<BBoxConversation>,
// }
//
// #[get("/<chat_id>")]
// pub(crate) async fn fetch_conversation(
//     cookies: BBoxCookieJar<'_, '_>,
//     chat_id: PCon<String, UserIdWebPolicy>,
// ) -> JsonResponse<FetchConversation, ()> {
//     let context = gen_context();
//     if cookies.get::<UsernamePolicy>("user_id").is_none() {
//         return JsonResponse(FetchConversation { conv: None }, Context::empty());
//     }
//     let user_id: PCon<String, UserIdWebPolicy> = cookies.get("user_id").unwrap().into();
//
//     let response = match DBCLIENT.get() {
//         None => {
//             panic!("Client should already exist");
//         }
//         Some(client) => client.retrieve_prompt(context, user_id, chat_id).await,
//     };
//     match response {
//         Err(e) => {
//             eprintln!("When fetching conversation details, received error : {}", e);
//             JsonResponse(FetchConversation { conv: None }, Context::empty())
//         }
//         Ok(boxed_conv) => JsonResponse(FetchConversation { conv: boxed_conv }, Context::empty()),
//     }
// }
//
// #[get("/delete/<chat_id>")]
// pub(crate) async fn delete_conversation(
//     cookies: BBoxCookieJar<'_, '_>,
//     chat_id: PCon<String, UserIdWebPolicy>,
// ) -> Result<(), ()> {
//     if cookies.get::<UsernamePolicy>("user_id").is_none() {}
//     let user_id: PCon<String, UserIdWebPolicy> =
//         cookies.get::<UserIdWebPolicy>("user_id").unwrap().into();
//     let context = gen_context();
//     let response = match DBCLIENT.get() {
//         None => {
//             panic!("Client should already exist");
//         }
//         Some(client) => {
//             client
//                 .delete_conversation(context, (user_id, chat_id))
//                 .await
//         }
//     };
//     response.map(|_| ()).map_err(|_| ())
// }

pub async fn benchmark_db(nb_iters: usize, rounds: usize) {
    let mut conv_ids: HashSet<String> = HashSet::new();

    let _conversation = crate::ads::gen_conversation(rounds);
    let _policy = MessagePolicy {
        storage: true,
        marketing_consent: true,
        third_party_ad_vendors_allowed: Vec::new(),
        unprotected_image_gen: true,
        reinforcement_learning_consent: true,
    };

    let (client, uuid) = block_on(async {
        let c = initialize_db_client().await;
        let username = Uuid::new_v4().to_string();
        let pol = UsernamePolicy {
            targeted_ads_consent: false,
            third_party_vendors_consent: HashMap::new(),
        };
        let uuid = login(&c, PCon::new(username, pol)).await;
        (c, uuid)
    });

    let pcr = UncheckedCriticalRegion::new(
        |t: Option<String>, _, _| t.unwrap().clone(),
        Signature {
            username: "",
            signature: "",
        },
    );

        for _ in 0..(nb_iters/ROUNDS) {
            let conversation = crate::ads::gen_conversation(rounds);
            let policy = MessagePolicy {
                storage: true,
                marketing_consent: true,
                third_party_ad_vendors_allowed: Vec::new(),
                unprotected_image_gen: true,
                reinforcement_learning_consent: true,
            };
            let mut conv_id = PCon::new(None, UserIdDBPolicy);
            for message in conversation {
                let boxed_message = PCon::new(message, policy.clone());
                conv_id = store_to_database(&client, uuid.clone(), conv_id, boxed_message)
                    .await
                    .into_verified(VR::new(|x| Some(x)));
            }
            let unboxed = conv_id.into_critical_unchecked(pcr, ());
            conv_ids.insert(unboxed);
        }
        println!("DB_Store done");

        let conv_ids = conv_ids.drain();
        for (i, cid) in conv_ids.enumerate(){
            let boxed_cid = PCon::new(cid, UserIdDBPolicy);
            retrieve_conversation(&client, uuid.clone(), boxed_cid, i).await;
        }
    println!("DB Benchmark done");
}
