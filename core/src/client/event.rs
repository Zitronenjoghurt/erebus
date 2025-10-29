use crate::error::ErebusError;

pub enum ClientEvent {
    Connected,
    Error(ErebusError),
}
