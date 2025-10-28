use crate::database::Database;
use crate::error::ErebusResult;
use crate::server::services::Services;
use std::path::PathBuf;
use tracing::info;

pub struct ErebusServerState {
    pub(crate) db: Database,
    service: Services,
}

impl ErebusServerState {
    pub fn new() -> ErebusResult<Self> {
        let db_path = PathBuf::from("./server.db");
        let db = Database::initialize(&db_path)?;
        info!("Database initialized at: {}", db_path.display());

        let service = Services::new();
        info!("Services initialized");

        Ok(Self { db, service })
    }
}
