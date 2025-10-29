use std::sync::{Arc, Mutex};

mod authentication;

#[derive(Clone)]
pub struct ClientState {
    pub auth: Arc<Mutex<authentication::AuthenticationState>>,
}

impl ClientState {
    pub fn initialize() -> Self {
        Self {
            auth: Arc::new(Mutex::new(authentication::AuthenticationState::default())),
        }
    }

    pub fn read_auth<T>(&self, f: impl FnOnce(&authentication::AuthenticationState) -> T) -> T {
        let guard = self.auth.lock().unwrap();
        f(&guard)
    }

    pub fn write_auth(&self, f: impl FnOnce(&mut authentication::AuthenticationState)) {
        let mut guard = self.auth.lock().unwrap();
        f(&mut guard);
    }
}
