use crate::crypto::private_key::PrivateKey;
use crate::crypto::public_key::PublicKey;
use crate::error::ErebusResult;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use sha2::{Digest, Sha256};

pub mod password;
pub mod private_key;
pub mod public_key;
pub mod registration_challenge;

pub fn x25519_keypair() -> (PublicKey, PrivateKey) {
    let private_key = PrivateKey::generate();
    let public_key = PublicKey::generate(&private_key);
    (public_key, private_key)
}

pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn encode_base64(data: &[u8]) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(data)
}

pub fn decode_base64(data: impl AsRef<str>) -> ErebusResult<Vec<u8>> {
    Ok(BASE64_URL_SAFE_NO_PAD.decode(data.as_ref().as_bytes())?)
}
