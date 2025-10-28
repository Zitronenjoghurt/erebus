use thiserror::Error;

pub type ErebusResult<T> = Result<T, ErebusError>;

#[derive(Debug, Error)]
pub enum ErebusError {
    #[error("Lost connection to the context thread")]
    ContextDisconnected,
    #[error("Encryption error")]
    Encryption,
    #[error("Decryption error")]
    Decryption,
    #[error("Something went wrong with a password which can't be disclosed")]
    PasswordUndisclosable,
    #[error("Client error: {0}")]
    Client(#[from] crate::client::error::ErebusClientError),
    #[error("Database error: {0}")]
    Database(#[from] redb::DatabaseError),
    #[error("Database storage error: {0}")]
    DatabaseStorage(#[from] redb::StorageError),
    #[error("Database table error: {0}")]
    DatabaseTable(#[from] redb::TableError),
    #[error("Database transaction error: {0}")]
    DatabaseTransaction(#[from] redb::TransactionError),
    #[error("Database transaction commit error: {0}")]
    DatabaseTransactionCommit(#[from] redb::CommitError),
    #[error("Decode error: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("Encode error: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] rmp_serde::decode::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] rmp_serde::encode::Error),
}
