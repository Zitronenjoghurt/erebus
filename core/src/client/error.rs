use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErebusClientError {
    #[error("Connection to {address} failed: {message}")]
    ConnectionFailed { address: String, message: String },
}
