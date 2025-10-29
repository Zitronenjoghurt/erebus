use crate::client::message::ClientMessage;
use crate::crypto::registration_challenge::RegistrationChallengeWithCode;
use crate::error::ErebusResult;
use crate::message::{MessageRecv, MessageSend};
use crate::server::connection_handler::ConnectionHandler;
use crate::server::message::error::{ErebusServerError, ErebusServerResult};
use crate::server::message::ServerMessage;
use crate::server::socket_id::SocketId;
use crate::server::state::ErebusServerState;
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, info};

pub struct Connection {
    id: SocketId,
    state: Arc<ErebusServerState>,
    writer: Arc<Mutex<OwnedWriteHalf>>,
    connections: ConnectionHandler,
}

impl Connection {
    pub fn spawn(
        state: Arc<ErebusServerState>,
        connections: ConnectionHandler,
        stream: TcpStream,
        id: SocketId,
    ) -> Arc<Self> {
        let (reader, writer) = stream.into_split();

        let connection = Arc::new(Self {
            state,
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
            if let Err(error) = self.handle_message(message).await {
                self.send_message(ServerMessage::Error(error)).await?;
            }
        }
    }

    async fn send_message(&self, message: ServerMessage) -> ErebusResult<()> {
        let mut writer = self.writer.lock().await;
        message.send(&mut *writer).await?;
        Ok(())
    }

    async fn handle_message(&self, message: ClientMessage) -> ErebusServerResult<()> {
        match message {
            ClientMessage::RegisterChallenge(challenge_and_code) => {
                self.handle_register_challenge(challenge_and_code).await?;
            }
        }
        Ok(())
    }
}

impl Connection {
    async fn handle_register_challenge(
        &self,
        challenge_and_code: RegistrationChallengeWithCode,
    ) -> ErebusServerResult<()> {
        let code_string = challenge_and_code.invite_code.as_base64();
        debug!(
            "Received registration challenge from {} for code {code_string}",
            self.id
        );

        let Some(code) = self.state.invite_find(&code_string)? else {
            return Err(ErebusServerError::InvalidInviteCode);
        };
        let solved_challenge = challenge_and_code.challenge.decrypt(&code.verify)?;

        self.send_message(ServerMessage::RegisterChallengeSolved(solved_challenge))
            .await?;

        Ok(())
    }
}
