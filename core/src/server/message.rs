use crate::crypto::registration_challenge::RegistrationChallenge;
use bincode::{Decode, Encode};

pub mod error;

#[derive(Encode, Decode)]
pub enum ServerMessage {
    Error(error::ErebusServerError),
    RegisterChallengeSolved(RegistrationChallenge),
}
