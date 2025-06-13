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
