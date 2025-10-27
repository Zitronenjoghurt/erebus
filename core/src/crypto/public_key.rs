use crate::crypto::private_key::PrivateKey;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct PublicKey(x25519_dalek::PublicKey);

impl PublicKey {
    pub(crate) fn get_key(&self) -> &x25519_dalek::PublicKey {
        &self.0
    }

    pub fn generate(private_key: &PrivateKey) -> Self {
        Self(x25519_dalek::PublicKey::from(private_key.get_secret()))
    }

    pub fn as_base64(&self) -> String {
        STANDARD.encode(self.0.to_bytes())
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: [u8; 32] = Deserialize::deserialize(deserializer)?;
        let key = x25519_dalek::PublicKey::from(bytes);
        Ok(Self(key))
    }
}
