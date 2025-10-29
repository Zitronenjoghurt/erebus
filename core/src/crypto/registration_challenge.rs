use crate::crypto::decode_base64;
use crate::crypto::private_key::PrivateKey;
use crate::crypto::public_key::PublicKey;
use crate::error::{ErebusError, ErebusResult};
use bincode::{BorrowDecode, Decode, Encode};
use rand_core::{OsRng, RngCore};

#[derive(Encode, Decode)]
pub struct RegistrationChallenge(Vec<u8>);

impl RegistrationChallenge {
    pub fn generate() -> Self {
        let mut bytes: [u8; 32] = [0; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes.to_vec())
    }

    pub fn encrypt(&self, key: &PublicKey) -> ErebusResult<Self> {
        Ok(Self(key.encrypt(&self.0)?))
    }

    pub fn decrypt(&self, key: &PrivateKey) -> ErebusResult<Self> {
        Ok(Self(key.decrypt(&self.0)?))
    }

    pub fn verify(&self, challenge: &Self) -> bool {
        self.0 == challenge.0
    }
}

#[derive(Encode, Decode)]
pub struct RegistrationChallengeWithCode {
    pub invite_code: PublicKey,
    pub challenge: RegistrationChallenge,
}

impl RegistrationChallengeWithCode {
    pub fn generate(invite_code: impl AsRef<str>) -> ErebusResult<(Self, RegistrationChallenge)> {
        let invite_code_bytes: [u8; 32] = decode_base64(invite_code)?
            .try_into()
            .map_err(|_| ErebusError::InvalidInviteCode)?;
        let invite_code = PublicKey::from_bytes(invite_code_bytes);
        let original_challenge = RegistrationChallenge::generate();
        let challenge = original_challenge.encrypt(&invite_code)?;
        Ok((
            Self {
                invite_code,
                challenge,
            },
            original_challenge,
        ))
    }
}
