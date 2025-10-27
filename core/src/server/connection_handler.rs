use crate::server::connection::Connection;
use crate::server::event::ServerEvent;
use dashmap::DashMap;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::net::TcpStream;

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

    pub fn handle(&self, event_sender: Sender<ServerEvent>, stream: TcpStream, addr: SocketAddr) {
        let connection = Connection::spawn(event_sender, self.clone(), stream, addr);
        self.connections.insert(addr, connection);
    }

    pub fn remove(&self, addr: SocketAddr) {
        self.connections.remove(&addr);
    }
}
