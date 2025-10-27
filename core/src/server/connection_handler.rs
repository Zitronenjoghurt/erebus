use crate::server::connection::Connection;
use dashmap::DashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tracing::debug;

#[derive(Clone)]
pub struct ConnectionHandler {
    connections: Arc<DashMap<SocketAddr, Arc<Connection>>>,
}

impl ConnectionHandler {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
        }
    }

    pub fn handle(&self, stream: TcpStream, addr: SocketAddr) {
        let connection = Connection::spawn(self.clone(), stream, addr);
        self.connections.insert(addr, connection);
        debug!("Added connection from {}", addr);
    }

    pub fn remove(&self, addr: SocketAddr) {
        self.connections.remove(&addr);
        debug!("Removed connection from {}", addr);
    }
}
