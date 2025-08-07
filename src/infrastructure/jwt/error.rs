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
            JwtError::CredentialsUnavailable(_) => self.internal_error(),
            JwtError::DefaultRoleMissing => {
                format!("Missing default role")
            }
            JwtError::JwtProcessingError { .. } => {
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


#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::errors::ErrorKind;
    use std::io;

    fn mock_jwt_error(stage: StageJwtProcessing) -> JwtError {
        let dummy_jwt_error = JwtLibraryError::from(ErrorKind::InvalidToken);
        JwtError::JwtProcessingError {
            stage,
            source: dummy_jwt_error,
        }
    }

    #[test]
    fn test_credentials_unavailable() {
        let credentials_error = CredentialsError::CacheReadError("test".to_string());
        let str_credentials_error = credentials_error.to_string();
        let jwt_error = JwtError::CredentialsUnavailable(credentials_error);

        assert_eq!(jwt_error.client_message(), jwt_error.internal_error());
        assert_eq!(jwt_error.level(), ErrorLevel::Error);
        assert!(jwt_error.log_message().contains("JwtError::CredentialsUnavailable"));
        assert!(jwt_error.log_message().contains(&str_credentials_error));
    }

    #[test]
    fn test_default_role_missing() {
        let jwt_error = JwtError::DefaultRoleMissing;

        assert_eq!(jwt_error.client_message(), "Missing default role");
        assert_eq!(jwt_error.level(), ErrorLevel::Info);
        assert_eq!(jwt_error.log_message(), "JwtError::DefaultRoleMissing");
    }

    #[test]
    fn test_jwt_processing_error_encode() {
        let jwt_error = mock_jwt_error(StageJwtProcessing::Encode);

        assert_eq!(jwt_error.client_message(), "Token is not correct");
        assert_eq!(jwt_error.level(), ErrorLevel::Info);
        let log_msg = jwt_error.log_message();
        assert!(log_msg.contains("JwtError::JwtProcessingError"));
        assert!(log_msg.contains("encoding"));
        assert!(log_msg.contains("InvalidToken"));
    }

    #[test]
    fn test_jwt_processing_error_decode() {
        let jwt_error = mock_jwt_error(StageJwtProcessing::Decode);

        assert_eq!(jwt_error.client_message(), "Token is not correct");
        assert_eq!(jwt_error.level(), ErrorLevel::Info);
        let log_msg = jwt_error.log_message();
        assert!(log_msg.contains("JwtError::JwtProcessingError"));
        assert!(log_msg.contains("decoding"));
        assert!(log_msg.contains("InvalidToken"));
    }

    #[test]
    fn test_stage_jwt_processing_display() {
        assert_eq!(StageJwtProcessing::Encode.to_string(), "encoding");
        assert_eq!(StageJwtProcessing::Decode.to_string(), "decoding");
    }
}
