pub use alohomora::tarpc::SimpleService;

#[tarpc::service]
pub trait SimpleService2 {
    async fn increment(x: i32) -> i32;
}
