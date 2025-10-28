use crate::database::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordVerifier(String);

impl PasswordVerifier {
    pub const PW_VERIFY_STRING: &'static str =
        "Z3JvdXBlbGV2ZW5iZXR3ZWVuaW5kdXN0cmlhbHNlbmR3cml0ZW1pbmRtZWFsc2V0dGk=";

    pub fn new() -> Self {
        Self(Self::PW_VERIFY_STRING.to_string())
    }

    pub fn verify(&self) -> bool {
        self.0 == Self::PW_VERIFY_STRING
    }
}

impl Entity for PasswordVerifier {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.0.clone()
    }

    fn table_name() -> &'static str {
        "pw_verify"
    }
}
