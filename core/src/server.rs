use crate::error::ErebusResult;
use std::sync::mpsc::{Receiver, Sender};

#[cfg(feature = "server")]
pub mod command;
mod connection;
#[cfg(feature = "server")]
mod connection_handler;
#[cfg(feature = "server")]
mod context;
#[cfg(feature = "server")]
pub mod event;
pub mod message;

#[cfg(feature = "server")]
pub struct ErebusServer {
    command_sender: Sender<command::ServerCommand>,
    event_receiver: Receiver<event::ServerEvent>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

#[cfg(feature = "server")]
impl ErebusServer {
    pub fn start(server_port: impl AsRef<str>) -> ErebusResult<Self> {
        let (command_sender, command_receiver) = std::sync::mpsc::channel();
        let (event_sender, event_receiver) = std::sync::mpsc::channel();

        let thread_handle =
            context::ErebusServerContext::spawn(server_port, command_receiver, event_sender)?;

        Ok(Self {
            command_sender,
            event_receiver,
            thread_handle: Some(thread_handle),
        })
    }

    pub fn poll_events(&self) -> Vec<event::ServerEvent> {
        self.event_receiver.try_iter().collect()
    }

    pub fn send_message(&self, message: message::ServerMessage) {
        let _ = self
            .command_sender
            .send(command::ServerCommand::Send(message));
    }
}

#[cfg(feature = "server")]
impl Drop for ErebusServer {
    fn drop(&mut self) {
        if let Some(thread_handle) = self.thread_handle.take() {
            let _ = thread_handle.join();
        }
    }
}
