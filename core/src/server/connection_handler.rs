use crate::server::connection::Connection;
use crate::server::socket_id::SocketId;
use crate::server::state::ErebusServerState;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tracing::debug;

#[derive(Clone)]
pub struct ConnectionHandler {
    connections: Arc<DashMap<SocketId, Arc<Connection>>>,
}

impl ConnectionHandler {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
        }
    }

    pub fn handle(&self, state: Arc<ErebusServerState>, stream: TcpStream, id: SocketId) {
        let connection = Connection::spawn(state, self.clone(), stream, id);
        self.connections.insert(id, connection);
        debug!("Added connection {}", id);
    }

    pub fn remove(&self, id: SocketId) {
        self.connections.remove(&id);
        debug!("Removed connection {}", id);
    }
}
