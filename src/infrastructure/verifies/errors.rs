use std::fmt;
use thiserror::Error;

/// Основная ошибка GraphQL клиента (обёртка)
#[derive(Debug, Error)]
pub enum PasswordVerifierError {
    #[error("Config error (stage = {stage}): {source}")]
    HashPasswordCryptError {
        stage: &'static str,
        #[source]
        source: bcrypt::BcryptError,
    },

}
/// Основная ошибка GraphQL клиента (обёртка)
#[derive(Debug, Error)]
pub enum ApiKeyVerifierError {
    #[error("Config error (stage = {stage}): {source}")]
    HashPasswordCryptError {
        stage: &'static str,
        #[source]
        source: bcrypt::BcryptError,
    },
    #[error("Decryption Error")]
    DecryptionError(String),
    #[error("Encryption Error")]
    EncryptionError(String)

}
