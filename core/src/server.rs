use crate::crypto::sha256_bytes;
use crate::error::ErebusResult;
use crate::server::connection_handler::ConnectionHandler;
use crate::server::socket_id::SocketId;
use crate::server::state::ErebusServerState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[cfg(feature = "server")]
mod connection;
#[cfg(feature = "server")]
mod connection_handler;
#[cfg(feature = "server")]
mod entities;
pub mod message;
#[cfg(feature = "server")]
mod services;
#[cfg(feature = "server")]
pub mod socket_id;
#[cfg(feature = "server")]
pub mod state;

#[cfg(feature = "server")]
pub struct ErebusServer {
    state: Arc<ErebusServerState>,
    listener: TcpListener,
    connection_handler: ConnectionHandler,
}

#[cfg(feature = "server")]
impl ErebusServer {
    pub async fn bind(server_port: impl AsRef<str>) -> ErebusResult<Self> {
        let state = ErebusServerState::new()?;
        info!("State initialized");

        let address = format!("{}:{}", "0.0.0.0", server_port.as_ref());
        let listener = TcpListener::bind(address).await?;
        info!("TCP listener bound");

        Ok(Self {
            state: Arc::new(state),
            listener,
            connection_handler: ConnectionHandler::new(),
        })
    }

    pub async fn run(&self) -> ErebusResult<()> {
        info!("Listening on {}", self.listener.local_addr()?);
        loop {
            let (stream, addr) = self.listener.accept().await?;
            let socket_id = SocketId::from(addr);
            info!("Connected to {}", socket_id);
            self.connection_handler
                .handle(self.state.clone(), stream, socket_id);
        }
    }
}
