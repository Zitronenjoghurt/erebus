use crate::crypto::sha256_bytes;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SocketId([u8; 32]);

impl SocketId {
    pub fn as_base64(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(self.0)
    }
}

impl From<SocketAddr> for SocketId {
    fn from(value: SocketAddr) -> Self {
        Self(sha256_bytes(value.to_string().as_bytes()))
    }
}

impl Display for SocketId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_base64())
    }
}
