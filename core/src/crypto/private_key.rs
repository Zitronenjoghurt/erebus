use crate::crypto::password::Password;
use crate::crypto::public_key::PublicKey;
use crate::error::{ErebusError, ErebusResult};
use rand_core::OsRng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use x25519_dalek::StaticSecret;

pub struct PrivateKey(StaticSecret);

impl PrivateKey {
    pub fn decrypt(&self, encrypted: &[u8]) -> ErebusResult<Vec<u8>> {
        if encrypted.len() < 32 {
            return Err(ErebusError::Decryption);
        };

        let (ephemeral_public_bytes, ciphertext) = encrypted.split_at(32);
        let ephemeral_public = PublicKey::from_bytes(
            ephemeral_public_bytes
                .try_into()
                .map_err(|_| ErebusError::Decryption)?,
        );

        let shared_secret = self.0.diffie_hellman(ephemeral_public.get_key());
        let password = Password::new(shared_secret.to_bytes());

        password.decrypt(ciphertext)
    }

    pub(crate) fn get_secret(&self) -> &StaticSecret {
        &self.0
    }

    pub fn generate() -> Self {
        Self(StaticSecret::random_from_rng(OsRng))
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(StaticSecret::from(bytes))
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: [u8; 32] = Deserialize::deserialize(deserializer)?;
        let secret = StaticSecret::from(bytes);
        Ok(Self(secret))
    }
}
