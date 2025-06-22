use super::errors::ApiKeyVerifierError;
use crate::domain::settings::model::Credentials;
use crate::domain::verifies::service::ApiKeyVerifierService;

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM symmetric encryption
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, TryRngCore};
use sha2::{Digest, Sha256};
use uuid::Uuid;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const BASE: usize = 62;
const NONCE_LEN: usize = 12;

pub struct ApiKeyVerifier {
    pub credentials: Credentials,
    pub encryption_key: [u8; 32], // 256-bit key
}

impl ApiKeyVerifier {
    pub fn new(credentials: Credentials) -> Self {
        let key = credentials.encryption_api_key().clone();
        
        let hash = Sha256::digest(key.as_bytes());

        // Преобразуем результат в массив [u8; 32]
        let mut encryption_key = [0u8; 32];
        encryption_key.copy_from_slice(&hash[..]);

        Self { credentials, encryption_key }
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
        _ = OsRng.try_fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, uuid.as_bytes().as_ref())
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
            return Err(ApiKeyVerifierError::DecryptionError(
                "Invalid token format".into(),
            ));
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce_bytes);

        let decrypted = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| ApiKeyVerifierError::DecryptionError(e.to_string()))?;

        Uuid::from_slice(&decrypted)
            .map_err(|e| ApiKeyVerifierError::DecryptionError(e.to_string()))
    }
}

impl ApiKeyVerifierService for ApiKeyVerifier {
    type Error = ApiKeyVerifierError;

    fn generate(&self, length: u16, user_id: Uuid) -> String {
        let encrypted_uuid = self.encrypt_uuid(user_id).expect("UUID encryption failed");

        let random_len = length as usize;
        let mut random_part = String::new();

        while random_part.len() < random_len {
            let mut buffer = [0u8; 16];
            OsRng.try_fill_bytes(&mut buffer).unwrap();
            random_part.push_str(&self.bytes_to_base62(buffer.to_vec()));
        }

        random_part.truncate(random_len);

        format!("{}-{}", encrypted_uuid, random_part)
    }

    fn extract_user_id(&self, api_key: &str) -> Result<Uuid, Self::Error> {
        let encrypted_part = api_key.split('-').next().ok_or_else(|| {
            ApiKeyVerifierError::DecryptionError("Token format invalid".to_string())
        })?;

        self.decrypt_uuid(encrypted_part)
    }

    fn is_verified(&self, api_key_hash: &str, api_key: &str) -> Result<bool, Self::Error> {
        bcrypt::verify(api_key, api_key_hash).map_err(|e| {
            ApiKeyVerifierError::HashPasswordCryptError {
                stage: "bcrypt::verify",
                source: e,
            }
        })
    }

    fn create_hash(&self, api_key: &str) -> Result<String, Self::Error> {
        bcrypt::hash(api_key, bcrypt::DEFAULT_COST).map_err(|e| {
            ApiKeyVerifierError::HashPasswordCryptError {
                stage: "bcrypt::hash",
                source: e,
            }
        })
    }
}

