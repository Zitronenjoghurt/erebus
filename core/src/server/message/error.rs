use crate::error::ErebusError;
use bincode::{Decode, Encode};

pub type ErebusServerResult<T> = Result<T, ErebusServerError>;

#[derive(Debug, thiserror::Error, Encode, Decode)]
pub enum ErebusServerError {
    #[error("Invalid invite code")]
    InvalidInviteCode,
    #[error("Unexpected error")]
    Unexpected,
}

impl From<ErebusError> for ErebusServerError {
    fn from(error: ErebusError) -> Self {
        match error {
            ErebusError::InvalidInviteCode => Self::InvalidInviteCode,
            _ => Self::Unexpected,
        }
    }
}
