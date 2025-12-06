use std::io::{BufReader, Cursor};

use rustls_pemfile::certs;

pub mod ads;
pub mod inference;
pub mod database;
pub mod login;

fn gen_context() -> tarpc::context::Context {
    let mut context = tarpc::context::current();
    // let mut rng = rand::thread_rng();
    // let trace_id = tarpc::trace::TraceId::random(&mut rng);
    // context.trace_context.trace_id = trace_id.clone();
    // drop(rng);
    context.deadline = std::time::SystemTime::now() + std::time::Duration::from_secs(45);
    context
}

pub fn load_certs(data: &str) -> Vec<rustls::pki_types::CertificateDer<'static>> {
    certs(&mut BufReader::new(Cursor::new(data)))
        .map(|result| result.unwrap())
        .collect()
}

pub const END_CHAIN: &str = include_str!("../../certs/end.chain");
