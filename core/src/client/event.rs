use crate::client::command::ClientCommand;
use crate::error::ErebusError;
use crate::server::message::ServerMessage;

#[derive(Debug)]
pub enum ClientEvent {
    Connected,
    ReceivedCommand(ClientCommand),
    ReceivedMessage(ServerMessage),
    Error(ErebusError),
}
