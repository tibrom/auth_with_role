use crate::domain::errors::service::AppErrorInfo;
use crate::domain::errors::service::ErrorLevel;
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

impl AppErrorInfo for PasswordVerifierError {
    fn client_message(&self) -> String {
        "Internal Server Error".to_string()
    }
    fn level(&self) -> ErrorLevel {
        ErrorLevel::Error
    }
    fn log_message(&self) -> String {
        match self {
            PasswordVerifierError::HashPasswordCryptError { stage, source } => {
                format!(
                    "PasswordVerifierError::HashPasswordCryptError at stage '{}': {}",
                    stage, source
                )
            }
        }
    }
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
    EncryptionError(String),
}

impl AppErrorInfo for ApiKeyVerifierError {
    fn client_message(&self) -> String {
        "Internal Server Error".to_string()
    }
    fn level(&self) -> ErrorLevel {
        ErrorLevel::Error
    }
    fn log_message(&self) -> String {
        match self {
            ApiKeyVerifierError::HashPasswordCryptError { stage, source } => {
                format!(
                    "ApiKeyVerifierError::HashPasswordCryptError at stage '{}': {}",
                    stage, source
                )
            }
            ApiKeyVerifierError::DecryptionError(msg) => {
                format!("ApiKeyVerifierError::DecryptionError: {}", msg)
            }
            ApiKeyVerifierError::EncryptionError(msg) => {
                format!("ApiKeyVerifierError::EncryptionError: {}", msg)
            }
        }
    }
}



#[derive(Debug, Error)]
pub enum TelegramVerifierError {
}

impl AppErrorInfo for TelegramVerifierError {
    fn client_message(&self) -> String {
        "Internal Server Error".to_string()
    }
    fn level(&self) -> ErrorLevel {
        ErrorLevel::Error
    }
    fn log_message(&self) -> String {
        "Internal Server Error".to_string()
    }
}