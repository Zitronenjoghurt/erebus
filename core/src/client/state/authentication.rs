use crate::crypto::registration_challenge::RegistrationChallenge;

#[derive(Default)]
pub enum AuthenticationState {
    #[default]
    Unauthenticated,
    RegistrationChallengePending {
        original_challenge: RegistrationChallenge,
    },
}

impl AuthenticationState {
    pub fn can_register(&self) -> bool {
        match self {
            Self::Unauthenticated => true,
            _ => false,
        }
    }

    pub fn set_authentication_pending(&mut self, original_challenge: RegistrationChallenge) {
        *self = Self::RegistrationChallengePending { original_challenge }
    }
}
