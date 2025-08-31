use rocket::Build;
use rocket::Rocket;
use rocket::routes;
use tracing::instrument::WithSubscriber;
use tracing_appender::non_blocking::WorkerGuard;
use std::fs::OpenOptions;
use std::net::{IpAddr, Ipv4Addr};

mod routes;
use self::routes::*;


//FIXME(douk): Hacky way of sharing a single host. 
pub static SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);


fn prepare_server() -> Rocket<Build>{


    Rocket::build().mount("/chat", routes![inference::inference])
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

use tracing_appender::non_blocking;

fn init_tracing() -> WorkerGuard{
    let log_path = "benchmark.log";
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(log_path)
      .unwrap();
    let (non_blocking, guard) = non_blocking(file);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_target(false)
        .with_ansi(false)
        .init();
    guard
}
