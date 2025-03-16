use rocket::Build;
use rocket::fs::FileServer;
use alohomora::rocket::{BBoxRocket, routes};
// use rocket_dyn_templates::Template;
use rocket::{State, get};
use services_utils::rpc::{
    database::{Database, TahiniDatabaseClient},
    inference::{Inference, TahiniInferenceClient},
};
use std::path::Path;
use std::sync::{Arc, Mutex};


mod inference;
mod policy;

fn prepare_server() -> BBoxRocket<Build>{
    BBoxRocket::build().mount("/chat", routes![inference::inference])
}

#[rocket::main]
async fn main() {
    if let Err(e) = prepare_server().launch().await {
        println!("Failed to launch fronting server");
        drop(e)
    }
}
