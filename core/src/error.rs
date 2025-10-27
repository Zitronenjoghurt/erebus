use thiserror::Error;

pub type ErebusResult<T> = Result<T, ErebusError>;

#[derive(Debug, Error)]
pub enum ErebusError {
    #[error("Lost connection to the context thread")]
    ContextDisconnected,
    #[error("Client error: {0}")]
    Client(#[from] crate::client::error::ErebusClientError),
    #[error("Decode error: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("Encode error: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
