use crate::client::message::ClientMessage;
use crate::error::ErebusResult;
use crate::message::MessageRecv;
use crate::server::connection_handler::ConnectionHandler;
use crate::server::event::ServerEvent;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct Connection {
    addr: SocketAddr,
    connections: ConnectionHandler,
    event_sender: Sender<ServerEvent>,
    writer: Arc<Mutex<OwnedWriteHalf>>,
}

impl Connection {
    pub fn spawn(
        event_sender: Sender<ServerEvent>,
        connections: ConnectionHandler,
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Arc<Self> {
        let (reader, writer) = stream.into_split();

        let connection = Arc::new(Self {
            addr,
            connections,
            event_sender,
            writer: Arc::new(Mutex::new(writer)),
        });

        let connection_clone = connection.clone();
        tokio::spawn(async move {
            let _ = connection_clone.listen(reader).await;
            connection_clone.connections.remove(connection_clone.addr);
        });

        connection
    }

    async fn listen(&self, mut reader: OwnedReadHalf) -> ErebusResult<()> {
        loop {
            let message = ClientMessage::recv(&mut reader).await?;
            self.handle_message(message).await?;
        }
    }

    async fn handle_message(&self, message: ClientMessage) -> ErebusResult<()> {
        let _ = self.event_sender.send(ServerEvent::ReceivedMessage {
            addr: self.addr,
            message,
        });
        Ok(())
    }
}
