use thiserror::Error;

use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};

#[derive(Debug, Error)]
pub enum CredentialsError {
    #[error("Failed to read credentials from cache: {0}")]
    CacheReadError(String),

    #[error("Failed to write credentials to cache: {0}")]
    CacheWriteError(String),

    // Вместо двух отдельных вариантов — один общий, но с полем `stage`
    #[error("Config error (stage = {stage}): {source}")]
    Config {
        stage: &'static str,
        #[source]
        source: config::ConfigError,
    },
}


impl AppErrorInfo for CredentialsError  {
    fn client_message(&self) -> String {
        self.internal_error()
    }
    fn level(&self) -> ErrorLevel {
        ErrorLevel::Critical
    }
    fn log_message(&self) -> String {
        match self {
            CredentialsError::CacheReadError(e) => {
                format!("CredentialsError::CacheReadError::{}", e)
            }
            CredentialsError::CacheWriteError(e) => {
                format!("CredentialsError::CacheWriteError::{}", e)
            }
            CredentialsError::Config { stage, source } => {
                format!("CredentialsError::Config stage: {} source: {}", stage, source)
            }
        }
    }
}