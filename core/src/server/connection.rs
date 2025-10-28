use crate::client::message::ClientMessage;
use crate::error::ErebusResult;
use crate::message::{MessageRecv, MessageSend};
use crate::server::connection_handler::ConnectionHandler;
use crate::server::message::ServerMessage;
use crate::server::socket_id::SocketId;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, info};

pub struct Connection {
    id: SocketId,
    connections: ConnectionHandler,
    writer: Arc<Mutex<OwnedWriteHalf>>,
}

impl Connection {
    pub fn spawn(connections: ConnectionHandler, stream: TcpStream, id: SocketId) -> Arc<Self> {
        let (reader, writer) = stream.into_split();

        let connection = Arc::new(Self {
            id,
            connections,
            writer: Arc::new(Mutex::new(writer)),
        });

        let connection_clone = connection.clone();
        tokio::spawn(async move {
            let result = connection_clone.listen(reader).await;
            if let Err(e) = result {
                info!("Lost connection {}: {}", id, e);
            } else {
                info!("Lost connection {}", id);
            }

            connection_clone.connections.remove(connection_clone.id);
        });

        connection
    }

    async fn listen(&self, mut reader: OwnedReadHalf) -> ErebusResult<()> {
        debug!("Connection {} is listening", self.id);
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
