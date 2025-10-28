use crate::crypto::private_key::PrivateKey;
use crate::crypto::public_key::PublicKey;
use sha2::{Digest, Sha256};

pub mod password;
pub mod private_key;
pub mod public_key;

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
