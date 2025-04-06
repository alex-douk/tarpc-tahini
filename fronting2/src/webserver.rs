use rocket::Build;
use alohomora::rocket::{BBoxRocket, routes};
use std::net::{IpAddr, Ipv4Addr};


mod inference;
mod database;
mod login;
mod ads;

//TODO(douk): Hacky way of sharing a single host. 
//Will have to get changed for static attestation for sure
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
    if let Err(e) = prepare_server().launch().await {
        println!("Failed to launch fronting server");
        drop(e)
    }
}
