use crate::client::command::ClientCommand;
use crate::client::state::ClientState;
use crate::error::ErebusResult;
use std::sync::mpsc::{Receiver, Sender};

#[cfg(feature = "client")]
pub mod command;
#[cfg(feature = "client")]
mod context;
pub mod error;
#[cfg(feature = "client")]
pub mod event;
pub mod message;
#[cfg(feature = "client")]
mod state;

#[cfg(feature = "client")]
pub struct ErebusClient {
    pub state: ClientState,
    command_sender: Sender<command::ClientCommand>,
    event_receiver: Receiver<event::ClientEvent>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

#[cfg(feature = "client")]
impl ErebusClient {
    pub fn start(server_address: impl AsRef<str>) -> ErebusResult<Self> {
        let (command_sender, command_receiver) = std::sync::mpsc::channel();
        let (event_sender, event_receiver) = std::sync::mpsc::channel();

        let state = ClientState::initialize();
        let thread_handle = context::ErebusClientContext::spawn(
            state.clone(),
            server_address,
            command_receiver,
            event_sender,
        )?;

        Ok(Self {
            state,
            command_sender,
            event_receiver,
            thread_handle: Some(thread_handle),
        })
    }

    pub fn poll_events(&self) -> Vec<event::ClientEvent> {
        self.event_receiver.try_iter().collect()
    }

    pub fn send_command(&self, command: command::ClientCommand) {
        let _ = self.command_sender.send(command);
    }

    pub fn register(&self, invite_code: impl AsRef<str>) {
        self.send_command(ClientCommand::Register {
            invite_code: invite_code.as_ref().to_string(),
        })
    }
}

#[cfg(feature = "client")]
impl Drop for ErebusClient {
    fn drop(&mut self) {
        if let Some(thread_handle) = self.thread_handle.take() {
            let _ = thread_handle.join();
        }
    }
}
