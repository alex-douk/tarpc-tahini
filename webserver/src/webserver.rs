use rocket::Build;
use rocket::Rocket;
use rocket::routes;
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
    routes::database::initialize_db_client().await;
    routes::ads::initialize_ad_client().await;
    routes::inference::initialize_llm_client().await;
    if let Err(e) = prepare_server().launch().await {
        println!("Failed to launch fronting server");
        drop(e)
    }
}
