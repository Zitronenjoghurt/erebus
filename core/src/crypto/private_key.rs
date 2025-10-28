use rand_core::OsRng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use x25519_dalek::StaticSecret;

pub struct PrivateKey(StaticSecret);

impl PrivateKey {
    pub(crate) fn get_secret(&self) -> &StaticSecret {
        &self.0
    }

    pub fn generate() -> Self {
        Self(StaticSecret::random_from_rng(OsRng))
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
