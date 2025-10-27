use crate::client::message::ClientMessage;
use crate::error::ErebusError;
use crate::server::command::ServerCommand;
use std::net::SocketAddr;

#[derive(Debug)]
pub enum ServerEvent {
    Listening,
    ReceivedCommand(ServerCommand),
    ReceivedMessage {
        addr: SocketAddr,
        message: ClientMessage,
    },
    Error(ErebusError),
}
