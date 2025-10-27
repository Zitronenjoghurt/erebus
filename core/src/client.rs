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
pub struct ErebusClient {
    command_sender: Sender<command::ClientCommand>,
    event_receiver: Receiver<event::ClientEvent>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

#[cfg(feature = "client")]
impl ErebusClient {
    pub fn start(server_address: impl AsRef<str>) -> ErebusResult<Self> {
        let (command_sender, command_receiver) = std::sync::mpsc::channel();
        let (event_sender, event_receiver) = std::sync::mpsc::channel();

        let thread_handle =
            context::ErebusClientContext::spawn(server_address, command_receiver, event_sender)?;

        Ok(Self {
            command_sender,
            event_receiver,
            thread_handle: Some(thread_handle),
        })
    }

    pub fn poll_events(&self) -> Vec<event::ClientEvent> {
        self.event_receiver.try_iter().collect()
    }

    pub fn send_message(&self, message: message::ClientMessage) {
        let _ = self
            .command_sender
            .send(command::ClientCommand::Send(message));
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
