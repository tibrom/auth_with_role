use thiserror::Error;
use super::error_dto::ComponentErrorDTO;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};

#[derive(Debug, Error)]
pub enum AuthenticatorError {
    #[error("Infrastructure Error")]
    InfrastructureError(ComponentErrorDTO),
    #[error("User not found by {0}")]
    UserNotFound(String),
    #[error("User not found by {0}")]
    ApiKeyAuthenticatorNotAllowed(String),
    #[error("Email password authentications not allowed for this user {0}")]
    EmailPasswdAuthNotAllowed(String),
    #[error("Api Key is not verified")]
    NotCorrectApiKey,
    #[error("Password Hash is not verified")]
    NotCorrectPassword


}

impl AuthenticatorError {
    fn error_level(&self) -> ErrorLevel {
        ErrorLevel::Info
    }
    fn msg_not_correct_credentials(&self) -> String {
        format!("Not correct credentials")
    }
}

impl AppErrorInfo for AuthenticatorError {
    fn client_message(&self) -> String {
        match self {
            AuthenticatorError::InfrastructureError(e) => {
                e.client_message()
            }
            _ => self.msg_not_correct_credentials()
        }
    }

    fn level(&self) -> ErrorLevel {
        match self {
            AuthenticatorError::InfrastructureError(e) => {
                e.level()
            }
            _ => {
                self.error_level()
            }
        }
    }
    fn log_message(&self) -> String {
        match self {
            AuthenticatorError::UserNotFound(v) => {
                format!("User not found by: {}", v)
            }
            AuthenticatorError::ApiKeyAuthenticatorNotAllowed(user_id) => {
                format!("User {} doesn't have api key", user_id)
            }
            AuthenticatorError::NotCorrectApiKey => {
                format!("Try create JWT with not correct api key")
            }
            AuthenticatorError::NotCorrectPassword => {
                format!("Try create JWT with not correct password")
            }
            AuthenticatorError::EmailPasswdAuthNotAllowed(v) => {
                format!("User doesn't have email or password. Authentications not allowed for this user {}", v)
            }
            AuthenticatorError::InfrastructureError(e) => {
                e.log_message()
            }
        }
    }
}