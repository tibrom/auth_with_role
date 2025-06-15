use crate::domain::verifies::service::ApiKeyVerifierService;
use super::errors::ApiKeyVerifierError;

use rand::{rngs::OsRng, RngCore, TryRngCore};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM symmetric encryption
use aes_gcm::aead::{Aead, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use std::fmt::Display;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const BASE: usize = 62;
const NONCE_LEN: usize = 12;
const ENCRYPTED_UUID_LEN_ESTIMATE: usize = 60;
const API_KEY_MIN_LEN: usize = 64;

pub struct ApiKeyVerifier {
    pub encryption_key: [u8; 32], // 256-bit key
}

impl ApiKeyVerifier {
    pub fn new(key: &str) -> Self {
        
        let hash = Sha256::digest(key.as_bytes());

        
        let mut encryption_key = [0u8; 32];
        encryption_key.copy_from_slice(&hash[..]);

        Self { encryption_key }
    }
    
    fn bytes_to_base62(&self, mut bytes: Vec<u8>) -> String {
        let mut num = 0u128;
        for byte in bytes.drain(..) {
            num = (num << 8) | (byte as u128);
        }

        let mut result = String::new();
        while num > 0 {
            let index = (num % BASE as u128) as usize;
            result.insert(0, CHARSET[index] as char);
            num /= BASE as u128;
        }

        result
    }

    fn encrypt_uuid(&self, uuid: Uuid) -> Result<String, ApiKeyVerifierError> {
        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);

        let mut nonce_bytes = [0u8; NONCE_LEN];
        OsRng.try_fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher.encrypt(nonce, uuid.as_bytes().as_ref())
            .map_err(|e| ApiKeyVerifierError::EncryptionError(e.to_string()))?;

        let mut data = nonce_bytes.to_vec();
        data.extend(ciphertext);

        Ok(general_purpose::STANDARD_NO_PAD.encode(data))
    }

    fn decrypt_uuid(&self, token: &str) -> Result<Uuid, ApiKeyVerifierError> {
        let data = general_purpose::STANDARD_NO_PAD
            .decode(token)
            .map_err(|e| ApiKeyVerifierError::DecryptionError(e.to_string()))?;

        if data.len() < NONCE_LEN {
            return Err(ApiKeyVerifierError::DecryptionError("Invalid token format".into()));
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce_bytes);

        let decrypted = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| ApiKeyVerifierError::DecryptionError(e.to_string()))?;

        Uuid::from_slice(&decrypted)
            .map_err(|e| ApiKeyVerifierError::DecryptionError(e.to_string()))
    }
}

impl ApiKeyVerifierService for ApiKeyVerifier {
    type Error = ApiKeyVerifierError;

    fn generate(&self, length: u16, user_id: Uuid) -> String {
        let encrypted_uuid = self.encrypt_uuid(user_id).expect("UUID encryption failed");

        let mut key = encrypted_uuid;
        let min_len = length.max(API_KEY_MIN_LEN as u16) as usize;

        while key.len() < min_len {
            let mut buffer = [0u8; 16];
            OsRng.try_fill_bytes(&mut buffer).unwrap();
            key.push_str(&self.bytes_to_base62(buffer.to_vec()));
        }

        key.truncate(min_len);
        key
    }

    fn extract_user_id(&self, api_key: &str) -> Result<Uuid, Self::Error> {
        let candidate = &api_key[..ENCRYPTED_UUID_LEN_ESTIMATE.min(api_key.len())];
        self.decrypt_uuid(candidate)
    }

    fn is_verified(&self, api_key_hash: &str, api_key: &str) -> Result<bool, Self::Error> {
        bcrypt::verify(api_key, api_key_hash)
            .map_err(|e| ApiKeyVerifierError::HashPasswordCryptError {
                stage: "bcrypt::verify",
                source: e,
            })
    }

    fn create_hash(&self, api_key: &str) -> Result<String, Self::Error> {
        bcrypt::hash(api_key, bcrypt::DEFAULT_COST)
            .map_err(|e| ApiKeyVerifierError::HashPasswordCryptError {
                stage: "bcrypt::hash",
                source: e,
            })
    }
}
