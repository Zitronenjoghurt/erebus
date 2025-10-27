use crate::client::message::ClientMessage;

#[derive(Debug)]
pub enum ClientCommand {
    Send(ClientMessage),
}
