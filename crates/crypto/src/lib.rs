//! Cryptographic utilities for the cli-secrets vault.
//!
//! Provides zero-knowledge key derivation and symmetric encryption algorithms.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;

/// Error type for cryptographic operations.
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Key derivation error: {0}")]
    KeyDerivation(String),
}

/// Derives a 32-byte symmetric key from a master password and salt using PBKDF2-HMAC-SHA256.
///
/// # Arguments
/// * `password` - The user's master password.
/// * `salt` - A cryptographic salt.
/// * `rounds` - The number of hashing iterations (e.g., 600_000).
pub fn derive_key_pbkdf2(password: &[u8], salt: &[u8], rounds: u32) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2::pbkdf2_hmac::<Sha256>(password, salt, rounds, &mut key);
    key
}

/// Derives a 32-byte symmetric key from a master password and salt using Argon2id.
///
/// # Arguments
/// * `password` - The user's master password.
/// * `salt` - A cryptographic salt (must be at least 8 bytes, recommended 16 bytes).
pub fn derive_key_argon2(password: &[u8], salt: &[u8]) -> Result<[u8; 32], CryptoError> {
    use argon2::{Algorithm, Argon2, Params, Version};

    let params = Params::default();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password, salt, &mut key)
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;
    Ok(key)
}

/// Encrypts plaintext bytes using AES-256-GCM.
///
/// Returns a vector of bytes containing the 12-byte random nonce prepended to the ciphertext and tag.
///
/// # Arguments
/// * `key` - The 32-byte AES key.
/// * `plaintext` - The raw data to encrypt.
pub fn encrypt_bytes(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    // Generate a 12-byte random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;

    // Prepend the nonce to the ciphertext
    let mut payload = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
    payload.extend_from_slice(&nonce_bytes);
    payload.extend_from_slice(&ciphertext);

    Ok(payload)
}

/// Decrypts ciphertext bytes encrypted with `encrypt_bytes` using AES-256-GCM.
///
/// # Arguments
/// * `key` - The 32-byte AES key.
/// * `payload` - The encrypted data containing the prepended 12-byte nonce.
pub fn decrypt_bytes(key: &[u8; 32], payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if payload.len() < 12 {
        return Err(CryptoError::Decryption("Ciphertext too short".to_string()));
    }

    let (nonce_bytes, ciphertext) = payload.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::Decryption(e.to_string()))?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::Decryption(e.to_string()))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbkdf2_derivation() {
        let password = b"supersecretpassword";
        let salt = b"saltsaltsalt";
        let key1 = derive_key_pbkdf2(password, salt, 1000);
        let key2 = derive_key_pbkdf2(password, salt, 1000);
        assert_eq!(key1, key2);

        let key3 = derive_key_pbkdf2(password, b"differentsalt", 1000);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_argon2_derivation() {
        let password = b"supersecretpassword";
        let salt = b"saltsaltsaltsalt"; // 16 bytes salt
        let key1 = derive_key_argon2(password, salt).unwrap();
        let key2 = derive_key_argon2(password, salt).unwrap();
        assert_eq!(key1, key2);

        let key3 = derive_key_argon2(password, b"differentsalt123").unwrap();
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_aes_gcm_roundtrip() {
        let key = [7u8; 32];
        let plaintext = b"Hello, Zero-Knowledge Vault World!";

        let encrypted = encrypt_bytes(&key, plaintext).unwrap();
        assert_ne!(plaintext.as_slice(), encrypted.as_slice());
        assert_eq!(encrypted.len(), 12 + plaintext.len() + 16); // 12-byte nonce + cipher + 16-byte auth tag

        let decrypted = decrypt_bytes(&key, &encrypted).unwrap();
        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }
}
