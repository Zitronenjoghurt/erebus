use crate::client::command::ClientCommand;
use crate::client::error::ErebusClientError;
use crate::client::event::ClientEvent;
use crate::client::message::ClientMessage;
use crate::client::state::ClientState;
use crate::crypto::registration_challenge::RegistrationChallengeWithCode;
use crate::error::{ErebusError, ErebusResult};
use crate::message::{MessageRecv, MessageSend};
use crate::server::message::ServerMessage;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use tokio::net::tcp::WriteHalf;
use tokio::net::TcpStream;

pub struct ErebusClientContext {
    state: ClientState,
    server_address: String,
    command_receiver: Receiver<ClientCommand>,
    event_sender: Sender<ClientEvent>,
}

impl ErebusClientContext {
    pub fn spawn(
        state: ClientState,
        server_address: impl AsRef<str>,
        command_receiver: Receiver<ClientCommand>,
        event_sender: Sender<ClientEvent>,
    ) -> ErebusResult<std::thread::JoinHandle<()>> {
        let context = Self::new(state, server_address, command_receiver, event_sender)?;
        let handle = std::thread::spawn(move || context.run());
        Ok(handle)
    }

    pub fn new(
        state: ClientState,
        server_address: impl AsRef<str>,
        command_receiver: Receiver<ClientCommand>,
        event_sender: Sender<ClientEvent>,
    ) -> ErebusResult<Self> {
        Ok(Self {
            state,
            server_address: server_address.as_ref().to_string(),
            command_receiver,
            event_sender,
        })
    }

    pub fn run(self) {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let result = self.run_async().await;
            if let Err(e) = result {
                self.send_event(ClientEvent::Error(e));
            }
        });
    }

    async fn run_async(&self) -> ErebusResult<()> {
        let mut stream = TcpStream::connect(self.server_address.clone()).await?;
        let (mut reader, mut writer) = stream.split();
        self.send_event(ClientEvent::Connected);

        loop {
            tokio::select! {
                command_result = async {
                    match self.command_receiver.try_recv() {
                        Ok(command) => Ok(Some(command)),
                        Err(TryRecvError::Empty) => {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            Ok(None)
                        },
                        Err(TryRecvError::Disconnected) => Err(ErebusError::ContextDisconnected),
                    }
                } => {
                    if let Some(command) = command_result? {
                        let result = self.handle_command(&mut writer, command).await;
                        if let Err(e) = result {
                            self.send_event(ClientEvent::Error(e));
                        }
                    }
                }

                message_result = ServerMessage::recv(&mut reader) => {
                    let message = message_result?;
                    self.handle_message(&mut writer, message).await?;
                }
            }
        }
    }

    fn send_event(&self, event: ClientEvent) {
        let _ = self.event_sender.send(event);
    }

    async fn handle_command(
        &self,
        tcp_writer: &mut WriteHalf<'_>,
        command: ClientCommand,
    ) -> ErebusResult<()> {
        match command {
            ClientCommand::Register { invite_code } => {
                self.handle_register(tcp_writer, invite_code).await?
            }
        }

        Ok(())
    }

    async fn handle_message(
        &self,
        tcp_writer: &mut WriteHalf<'_>,
        message: ServerMessage,
    ) -> ErebusResult<()> {
        Ok(())
    }
}

// Command handling
impl ErebusClientContext {
    async fn handle_register(
        &self,
        tcp_writer: &mut WriteHalf<'_>,
        invite_code: String,
    ) -> ErebusResult<()> {
        if !self.state.read_auth(|auth| auth.can_register()) {
            return Err(ErebusClientError::AlreadyRegistered.into());
        };

        let (payload, original_challenge) = RegistrationChallengeWithCode::generate(invite_code)?;
        self.state
            .write_auth(|auth| auth.set_authentication_pending(original_challenge));

        ClientMessage::RegisterChallenge(payload)
            .send(tcp_writer)
            .await?;

        Ok(())
    }
}
