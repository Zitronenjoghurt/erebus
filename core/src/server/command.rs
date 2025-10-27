use crate::server::message::ServerMessage;

#[derive(Debug)]
pub enum ServerCommand {
    Send(ServerMessage),
}
