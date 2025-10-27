use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub enum ServerMessage {
    Hello,
}
