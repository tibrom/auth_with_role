use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use jsonwebtoken::errors::Error as JwtLibraryError;
use std::fmt;
use thiserror::Error;

use super::config::errors::CredentialsError;

#[derive(Debug, Error)]
pub enum JwtError {
    /// Occurs when required credentials for generating the JWT are unavailable or invalid.
    #[error("Failed to retrieve credentials: {0}")]
    CredentialsUnavailable(#[from] CredentialsError),

    /// Indicates that the default role required for the token is missing.
    #[error("Default role is missing in the JWT claims")]
    DefaultRoleMissing,

    /// A JWT-related error occurred during a specific stage (e.g., encoding, decoding).
    #[error("JWT error during '{stage}' stage: {source}")]
    JwtProcessingError {
        /// The stage at which the error occurred (e.g., encoding, decoding).
        stage: StageJwtProcessing,

        /// The original error from the `jsonwebtoken` library.
        #[source]
        source: JwtLibraryError,
    },
}

impl AppErrorInfo for JwtError {
    fn client_message(&self) -> String {
        match self {
            JwtError::CredentialsUnavailable(e) => self.internal_error(),
            JwtError::DefaultRoleMissing => {
                format!("Missing default role")
            }
            JwtError::JwtProcessingError { stage, source } => {
                format!("Token is not correct")
            }
        }
    }
    fn level(&self) -> ErrorLevel {
        match self {
            JwtError::CredentialsUnavailable(_) => ErrorLevel::Error,
            _ => ErrorLevel::Info,
        }
    }
    fn log_message(&self) -> String {
        match self {
            JwtError::CredentialsUnavailable(e) => {
                format!("JwtError::CredentialsUnavailable:: {} ", e)
            }
            JwtError::DefaultRoleMissing => {
                format!("JwtError::DefaultRoleMissing")
            }
            JwtError::JwtProcessingError { stage, source } => {
                format!(
                    "JwtError::JwtProcessingError stage: {} source: {}",
                    stage, source
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageJwtProcessing {
    Encode,
    Decode,
}

impl fmt::Display for StageJwtProcessing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StageJwtProcessing::Encode => write!(f, "encoding"),
            StageJwtProcessing::Decode => write!(f, "decoding"),
        }
    }
}
