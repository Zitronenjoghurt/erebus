use crate::crypto::sha256_bytes;
use crate::error::{ErebusError, ErebusResult};
use argon2::Argon2;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, KeyInit, Nonce};
use rand_core::OsRng;
use zeroize::Zeroizing;

pub struct Password([u8; 32]);

impl Password {
    pub fn from_string(password: Zeroizing<String>) -> Option<Self> {
        let salt: [u8; 32] = sha256_bytes(password.as_bytes());
        let argon2 = Argon2::default();
        let mut key = [0u8; 32];

        argon2
            .hash_password_into(password.as_bytes(), &salt, &mut key)
            .ok()?;

        Some(Self(key))
    }

    pub fn prompt() -> Option<Self> {
        let password = Zeroizing::new(rpassword::prompt_password("Password:").ok()?);
        Self::from_string(password)
    }

    pub fn prompt_with_confirmation() -> Option<Self> {
        let password = Zeroizing::new(rpassword::prompt_password("Password:").ok()?);
        let confirmation = Zeroizing::new(rpassword::prompt_password("Confirm password:").ok()?);
        if password == confirmation {
            Self::from_string(password)
        } else {
            None
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn encrypt(&self, plaintext: &[u8]) -> ErebusResult<Vec<u8>> {
        let Ok(cipher) = ChaCha20Poly1305::new_from_slice(&self.0) else {
            return Err(ErebusError::Encryption);
        };

        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let Ok(ciphertext) = cipher.encrypt(&nonce, plaintext) else {
            return Err(ErebusError::Encryption);
        };

        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn decrypt(&self, encrypted: &[u8]) -> ErebusResult<Vec<u8>> {
        if encrypted.len() < 28 {
            return Err(ErebusError::Decryption);
        }

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let Ok(cipher) = ChaCha20Poly1305::new_from_slice(&self.0) else {
            return Err(ErebusError::Decryption);
        };

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| ErebusError::Decryption)
    }
}
