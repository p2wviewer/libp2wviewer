use anyhow::{Result, anyhow};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM 256-bit
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use sha2::{Sha256, Digest};
use log::{info, debug, warn};
use std::fs;
use rand::RngCore as OldRngCore;

pub fn derive_key_from_password(password: &str, salt_opt: Option<[u8; 16]>) -> Result<([u8; 32], [u8; 16])> {
    info!("Generating key from password string using Argon2");
    let salt = match salt_opt {
        Some(s) => s,
        None => {
            let mut s = [0u8; 16];
            rand::rng().fill_bytes(&mut s);
            s
        }
    };
    debug!("Using salt: {:?}", hex::encode(&salt));
    let argon = Argon2::default();
    let salt_str = SaltString::encode_b64(&salt).unwrap();
    let hash = argon.hash_password(password.as_bytes(), &salt_str)
        .map_err(|e| anyhow!("Argon2 hashing failed: {}", e))?;
    let mut key = [0u8; 32];
    let binding = hash.hash.unwrap();
    let hash_bytes = binding.as_bytes();
    let len = hash_bytes.len().min(32);
    key[..len].copy_from_slice(&hash_bytes[..len]);
    debug!("Generated key: {:?}", hex::encode(&key));
    Ok((key, salt))
}

/// Hash the file using SHA256
pub fn derive_key_from_file(path: &str) -> Result<([u8; 32], [u8; 16])> {
    info!("Hash key from file: {}", path);
    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..]);
    debug!("Hash key from file: {:?}", hex::encode(&key));
    Ok((key, [0u8; 16]))
}

/// Generate a rand nonce for AES-GCM
pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    debug!("Random nonce: {:?}", hex::encode(&nonce));
    nonce
}

/// Selfcheck token
pub fn generate_token(key: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let result = hasher.finalize();
    let mut token = [0u8; 32];
    token.copy_from_slice(&result[..]);
    debug!("Selfcheck token: {:?}", hex::encode(&token));
    token
}

/// Encrypt mit AES-GCM
pub fn encrypt(key: &[u8; 32], nonce_bytes: &[u8; 12], plaintext: &[u8]) -> Result<(Vec<u8>, [u8; 16])> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    debug!("Encrypting {} bytes", plaintext.len());
    let ciphertext_with_tag = cipher.encrypt(nonce, plaintext)
        .map_err(|e| {
            warn!("AES-GCM encryption failed: {:?}", e);
            anyhow::anyhow!("AES-GCM encryption failed: {:?}", e)
        })?;
    // Split the ciphertext and the authentication tag
    let (ciphertext, auth_tag) = ciphertext_with_tag.split_at(ciphertext_with_tag.len() - 16);
    let mut auth_tag_array = [0u8; 16];
    auth_tag_array.copy_from_slice(auth_tag);
    
    info!("Encryption successful, length {}", ciphertext.len());
    Ok((ciphertext.to_vec(), auth_tag_array))
}

/// Decrypt mit AES-GCM
pub fn decrypt(key: &[u8; 32], nonce_bytes: &[u8; 12], ciphertext: &[u8], auth_tag: &[u8; 16]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    let mut ciphertext_with_tag = Vec::from(ciphertext);
    ciphertext_with_tag.extend_from_slice(auth_tag);
    debug!("Decrypting {} bytes", ciphertext.len());
    let plaintext = cipher.decrypt(nonce, ciphertext_with_tag.as_ref())
        .map_err(|_| {
            // Provide a user-friendly error instead of the raw crypto error
            anyhow!("Decryption failed. The password may be incorrect or the data is corrupted.")
        })?;
    info!("Decryption successful, length {}", plaintext.len());
    Ok(plaintext)
}