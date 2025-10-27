use crate::error::{ErebusError, ErebusResult};
use crate::server::command::ServerCommand;
use crate::server::connection_handler::ConnectionHandler;
use crate::server::event::ServerEvent;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use tokio::net::TcpListener;

pub struct ErebusServerContext {
    server_port: String,
    command_receiver: Receiver<ServerCommand>,
    event_sender: Sender<ServerEvent>,
    connection_handler: ConnectionHandler,
}

impl ErebusServerContext {
    pub fn spawn(
        server_port: impl AsRef<str>,
        command_receiver: Receiver<ServerCommand>,
        event_sender: Sender<ServerEvent>,
    ) -> ErebusResult<std::thread::JoinHandle<()>> {
        let context = Self::new(server_port, command_receiver, event_sender)?;
        let handle = std::thread::spawn(move || context.run());
        Ok(handle)
    }

    pub fn new(
        server_port: impl AsRef<str>,
        command_receiver: Receiver<ServerCommand>,
        event_sender: Sender<ServerEvent>,
    ) -> ErebusResult<Self> {
        Ok(Self {
            server_port: server_port.as_ref().to_string(),
            command_receiver,
            event_sender,
            connection_handler: ConnectionHandler::new(),
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
                self.send_event(ServerEvent::Error(e));
            }
        });
    }

    async fn run_async(&self) -> ErebusResult<()> {
        let address = format!("{}:{}", "127.0.0.1", self.server_port);
        let listener = TcpListener::bind(address).await?;
        self.send_event(ServerEvent::Listening);

        loop {
            tokio::select! {
                command_result = async {
                    match self.command_receiver.try_recv() {
                        Ok(command) => Ok(Some(command)),
                        Err(TryRecvError::Empty) => Ok(None),
                        Err(TryRecvError::Disconnected) => Err(ErebusError::ContextDisconnected),
                    }
                } => {
                    if let Some(command) = command_result? {
                        self.handle_command(command).await?;
                    }
                }

                accept_result = listener.accept() => {
                    let (stream, addr) = accept_result?;
                    self.connection_handler.handle(self.event_sender.clone(), stream, addr);
                }
            }
        }
    }

    fn send_event(&self, event: ServerEvent) {
        let _ = self.event_sender.send(event);
    }

    async fn handle_command(&self, command: ServerCommand) -> ErebusResult<()> {
        self.send_event(ServerEvent::ReceivedCommand(command));
        Ok(())
    }
}
