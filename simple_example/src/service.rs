use alohomora::bbox::BBox as PCon;
use alohomora::pure::PrivacyPureRegion as PPR;
use alohomora::tarpc::TahiniEnum;
use alohomora::tarpc::client::TahiniStub;
use alohomora::tarpc::traits::{TahiniError, TahiniType};
use alohomora::{TahiniType, tahini_service};
use aws_lc_rs::aead::{AES_128_GCM, RandomizedNonceKey};
use aws_lc_rs::agreement::{self, UnparsedPublicKey, agree_ephemeral};
use aws_lc_rs::error::Unspecified;
use aws_lc_rs::kdf::{self, SskdfHmacAlgorithmId, get_sskdf_hmac_algorithm, sskdf_hmac};
use tarpc::serde::{Deserialize, Serialize};

// use alohomora::tarpc::hacky::ExamplePolicy;
use crate::policy::ExamplePolicy;
//
#[derive(Debug, Deserialize, Clone, TahiniType)]
pub struct InnerStruct {
    pub a: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generic error")
    }
}

impl std::error::Error for MyError {}

impl TahiniError for MyError {}

#[derive(Debug, Deserialize, Clone, TahiniType)]
pub struct MyType {
    pub a: i32,
    pub b: PCon<String, ExamplePolicy>,
    pub c: Result<i32, MyError>,
}

#[derive(Serialize, Deserialize, TahiniType)]
struct KeyWrapper {
    key_bytes: Vec<u8>,
}

pub fn test_keys() -> TahiniEnum {
    let rng = aws_lc_rs::rand::SystemRandom::new();
    let skey = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
    let pkey = skey.compute_public_key().unwrap();
    let wrap = KeyWrapper {
        key_bytes: pkey.as_ref().to_vec(),
    };
    let pkey_peer = UnparsedPublicKey::new(&agreement::X25519, wrap.key_bytes.clone());
    //KDF call as a closure
    let mut a = [0u8; 32];
    let salt = aws_lc_rs::rand::fill(&mut a);
    let info = "Tahini.service".as_bytes();
    let mut end_derived_key = [0u8; 16];
    let alg_id = get_sskdf_hmac_algorithm(SskdfHmacAlgorithmId::Sha256)
        .ok_or(Unspecified)
        .unwrap();
    let usable_kdf =
        |key_material: &[u8]| sskdf_hmac(alg_id, key_material, &info, &a, &mut end_derived_key);

    let key = agree_ephemeral(skey, &pkey_peer, aws_lc_rs::error::Unspecified, usable_kdf).unwrap();
    let aes_key = RandomizedNonceKey::new(&AES_128_GCM, &end_derived_key).unwrap();
    let plaintext = "This is a test message".as_bytes();
    let mut out = Vec::from(plaintext);
    wrap.to_tahini_enum()
}

// START: this part is what the developer writes.
// #[TahiniService]
#[tahini_service(domain=company)]
pub trait SimpleService {
    async fn increment(x: PCon<i32, ExamplePolicy>) -> PCon<String, ExamplePolicy>;
    // async fn test_types(x: MyType) -> MyType;
    // async fn attest(x: u64) -> Manifest;
}

#[derive(Clone)]
pub struct SimpleServiceServer;
impl SimpleService for SimpleServiceServer {
    async fn increment(
        self,
        _context: tarpc::context::Context,
        x: PCon<i32, ExamplePolicy>,
    ) -> PCon<String, ExamplePolicy> {
        println!("Within the application level, we are operating on PCons.");
        x.into_ppr(PPR::new(|val| format!("{}", val + 1)))
    }

    // async fn attest(self,context: tarpc::context::Context,x:u64) -> bool {
    //
    // }
    // async fn test_types(self, ctxt: tarpc::context::Context, mut x: MyType) -> MyType {
    //     x.a = 0;
    //     x
    // }
}
// E
