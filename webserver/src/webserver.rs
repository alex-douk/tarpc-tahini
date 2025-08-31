#![feature(negative_impls)]

use rocket::Build;
use alohomora::rocket::{BBoxRocket, routes};
use std::net::{IpAddr, Ipv4Addr};
use tracing::instrument::WithSubscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::non_blocking;
// use tracing_subscriber::EnvFilter;
use std::fs::OpenOptions;

mod routes;
pub(crate) mod policies;
pub(crate) mod adapters;
use self::routes::*;


//FIXME(douk): Hacky way of sharing a single host. 
pub static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);


fn prepare_server() -> BBoxRocket<Build>{

    BBoxRocket::build().mount("/chat", routes![inference::inference])
        .mount("/history", routes![database::get_history, database::delete_conversation])
        .mount("/account", routes![login::login, login::signup])
        .mount("/c", routes![database::fetch_conversation])
        .mount("/ads", routes![ads::get_ads_vendors])
}

#[rocket::main]
async fn main() {
    let guard = init_tracing();
    routes::database::initialize_db_client().await;
    routes::ads::initialize_ad_client().await;
    routes::inference::initialize_llm_client().await;
    if let Err(e) = prepare_server().launch().await {
        println!("Failed to launch fronting server");
        drop(e)
    }
    drop(guard)
}

fn init_tracing() -> WorkerGuard{
    
    let log_path = "benchmark.log";
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(log_path)
      .unwrap();
    let (non_blocking, guard) = non_blocking(file);
    // let env_filter = EnvFilter::new("webserver=info");
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        // .with(env_filter)
        // .with_target(false)
        .with_ansi(false)
        .init();
    guard
}
