use crate::crypto::private_key::PrivateKey;
use crate::crypto::public_key::PublicKey;

pub mod private_key;
pub mod public_key;

pub fn x25519_keypair() -> (PublicKey, PrivateKey) {
    let private_key = PrivateKey::generate();
    let public_key = PublicKey::generate(&private_key);
    (public_key, private_key)
}
