use rocket::Build;
use rocket::fs::FileServer;
use alohomora::rocket::{BBoxRocket, routes};
// use rocket_dyn_templates::Template;
use rocket::{State, get};
use services_utils::rpc::{
    database::{Database, TahiniDatabaseClient},
    inference::{Inference, TahiniInferenceClient},
};
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::sync::{Arc, Mutex};


mod inference;
mod policy;
mod database;
mod login;

//TODO(douk): Hacky way of sharing a single host. 
//Will have to get changed for static attestation for sure
pub static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);


fn prepare_server() -> BBoxRocket<Build>{
    BBoxRocket::build().mount("/chat", routes![inference::inference])
        .mount("/history", routes![database::get_history])
        .mount("/account", routes![login::login, login::signup])
        .mount("/c", routes![database::fetch_conversation])
}

#[rocket::main]
async fn main() {
    if let Err(e) = prepare_server().launch().await {
        println!("Failed to launch fronting server");
        drop(e)
    }
}
