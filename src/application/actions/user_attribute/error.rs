use thiserror::Error;
use crate::application::error_dto::ComponentErrorDTO;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};

#[derive(Debug, Error)]
pub enum UserAttributeError {
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

impl UserAttributeError {
    fn error_level(&self) -> ErrorLevel {
        ErrorLevel::Info
    }
    fn msg_not_correct_credentials(&self) -> String {
        format!("Not correct credentials")
    }
}

impl AppErrorInfo for UserAttributeError {
    fn client_message(&self) -> String {
        match self {
            UserAttributeError::InfrastructureError(e) => {
                e.client_message()
            }
            _ => self.msg_not_correct_credentials()
        }
    }

    fn level(&self) -> ErrorLevel {
        match self {
            UserAttributeError::InfrastructureError(e) => {
                e.level()
            }
            _ => {
                self.error_level()
            }
        }
    }
    fn log_message(&self) -> String {
        match self {
            UserAttributeError::UserNotFound(v) => {
                format!("User not found by: {}", v)
            }
            UserAttributeError::ApiKeyAuthenticatorNotAllowed(user_id) => {
                format!("User {} doesn't have api key", user_id)
            }
            UserAttributeError::NotCorrectApiKey => {
                format!("Try create JWT with not correct api key")
            }
            UserAttributeError::NotCorrectPassword => {
                format!("Try create JWT with not correct password")
            }
            UserAttributeError::EmailPasswdAuthNotAllowed(v) => {
                format!("User doesn't have email or password. Authentications not allowed for this user {}", v)
            }
            UserAttributeError::InfrastructureError(e) => {
                e.log_message()
            }
        }
    }
}