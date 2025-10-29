use crate::crypto::registration_challenge::RegistrationChallengeWithCode;
use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum ClientMessage {
    RegisterChallenge(RegistrationChallengeWithCode),
}
