use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErebusClientError {
    #[error("Already registered")]
    AlreadyRegistered,
}
