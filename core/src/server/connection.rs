use crate::client::message::ClientMessage;
use crate::error::ErebusResult;
use crate::message::{MessageRecv, MessageSend};
use crate::server::connection_handler::ConnectionHandler;
use crate::server::message::ServerMessage;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, info};

pub struct Connection {
    addr: SocketAddr,
    connections: ConnectionHandler,
    writer: Arc<Mutex<OwnedWriteHalf>>,
}

impl Connection {
    pub fn spawn(connections: ConnectionHandler, stream: TcpStream, addr: SocketAddr) -> Arc<Self> {
        let (reader, writer) = stream.into_split();

        let connection = Arc::new(Self {
            addr,
            connections,
            writer: Arc::new(Mutex::new(writer)),
        });

        let connection_clone = connection.clone();
        tokio::spawn(async move {
            let result = connection_clone.listen(reader).await;
            if let Err(e) = result {
                info!("Lost connection with {}: {}", addr, e);
            } else {
                info!("Lost connection with {}", addr);
            }

            connection_clone.connections.remove(connection_clone.addr);
        });

        connection
    }

    async fn listen(&self, mut reader: OwnedReadHalf) -> ErebusResult<()> {
        debug!("Connection from {} is listening", self.addr);
        loop {
            let message = ClientMessage::recv(&mut reader).await?;
            self.handle_message(message).await?;
        }
    }

    async fn send_message(&self, message: ServerMessage) -> ErebusResult<()> {
        let mut writer = self.writer.lock().await;
        message.send(&mut *writer).await?;
        Ok(())
    }

    async fn handle_message(&self, message: ClientMessage) -> ErebusResult<()> {
        match message {
            ClientMessage::Hello => {
                self.send_message(ServerMessage::Hello).await?;
            }
        }
        Ok(())
    }
}
