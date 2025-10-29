use crate::crypto::encode_base64;
use crate::crypto::password::Password;
use crate::crypto::private_key::PrivateKey;
use crate::error::ErebusResult;
use bincode::de::read::Reader;
use bincode::de::Decoder;
use bincode::enc::write::Writer;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{BorrowDecode, Decode, Encode};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct PublicKey(x25519_dalek::PublicKey);

impl PublicKey {
    pub fn encrypt(&self, plaintext: &[u8]) -> ErebusResult<Vec<u8>> {
        let ephemeral_private = PrivateKey::generate();
        let ephemeral_public = PublicKey::generate(&ephemeral_private);

        let shared_secret = ephemeral_private
            .get_secret()
            .diffie_hellman(self.get_key());
        let password = Password::new(shared_secret.to_bytes());

        let ciphertext = password.encrypt(plaintext)?;
        let mut result = Vec::with_capacity(32 + ciphertext.len());
        result.extend_from_slice(&ephemeral_public.0.to_bytes());
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub(crate) fn get_key(&self) -> &x25519_dalek::PublicKey {
        &self.0
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(x25519_dalek::PublicKey::from(bytes))
    }

    pub fn generate(private_key: &PrivateKey) -> Self {
        Self(x25519_dalek::PublicKey::from(private_key.get_secret()))
    }

    pub fn as_base64(&self) -> String {
        encode_base64(&self.0.to_bytes())
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

impl Encode for PublicKey {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&self.0.to_bytes())
    }
}

impl<Context> Decode<Context> for PublicKey {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut bytes = [0u8; 32];
        decoder.reader().read(&mut bytes)?;
        Ok(Self(x25519_dalek::PublicKey::from(bytes)))
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for PublicKey {
    fn borrow_decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut bytes = [0u8; 32];
        decoder.reader().read(&mut bytes)?;
        Ok(Self(x25519_dalek::PublicKey::from(bytes)))
    }
}
