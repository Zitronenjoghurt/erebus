use crate::crypto::private_key::PrivateKey;
use crate::crypto::public_key::PublicKey;
use crate::database::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InviteCode {
    pub code: PublicKey,
    verify: PrivateKey,
}

impl Entity for InviteCode {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.code.as_base64()
    }

    fn table_name() -> &'static str {
        "invite_codes"
    }
}

impl InviteCode {
    pub fn generate() -> Self {
        let (public, private) = crate::crypto::x25519_keypair();
        Self {
            code: public,
            verify: private,
        }
    }
}
