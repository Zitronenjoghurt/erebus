use crate::error::ErebusResult;
use crate::server::connection_handler::ConnectionHandler;
use tokio::net::TcpListener;
use tracing::info;

#[cfg(feature = "server")]
mod connection;
#[cfg(feature = "server")]
mod connection_handler;
pub mod message;

#[cfg(feature = "server")]
pub struct ErebusServer {
    listener: TcpListener,
    connection_handler: ConnectionHandler,
}

#[cfg(feature = "server")]
impl ErebusServer {
    pub async fn bind(server_port: impl AsRef<str>) -> ErebusResult<Self> {
        let address = format!("{}:{}", "127.0.0.1", server_port.as_ref());
        let listener = TcpListener::bind(address).await?;
        info!("Listening on {}", listener.local_addr()?);

        Ok(Self {
            listener,
            connection_handler: ConnectionHandler::new(),
        })
    }

    pub async fn run(&self) -> ErebusResult<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            info!("Incoming connection from {}", addr);
            self.connection_handler.handle(stream, addr);
        }
    }
}
