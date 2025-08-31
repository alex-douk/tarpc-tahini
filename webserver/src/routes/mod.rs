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
