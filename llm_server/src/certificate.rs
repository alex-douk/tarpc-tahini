use std::io::{BufReader, Cursor};
use rustls_pemfile::certs;

pub fn load_certs(data: &str) -> Vec<rustls::pki_types::CertificateDer<'static>> {
    certs(&mut BufReader::new(Cursor::new(data)))
        .map(|result| result.unwrap())
        .collect()
}

pub fn load_private_key(key: &str) -> rustls::pki_types::PrivateKeyDer<'_> {
    let mut reader = BufReader::new(Cursor::new(key));
    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::Pkcs1Key(key)) => return key.into(),
            Some(rustls_pemfile::Item::Pkcs8Key(key)) => return key.into(),
            Some(rustls_pemfile::Item::Sec1Key(key)) => return key.into(),
            None => break,
            _ => continue,
        }
    }
    panic!("no keys found in {key:?} (encrypted keys not supported)");
}
// used on client-side for server tls
pub const END_CHAIN: &str = include_str!("../certs/end.chain");

// used on server-side for server tls
pub const END_CERT: &str = include_str!("../certs/end.cert");
pub const END_PRIVATEKEY: &str = include_str!("../certs/end.key");
