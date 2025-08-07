use super::super::network::hasura::error::HasuraClientError;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use thiserror::Error;

/// Основная ошибка GraphQL клиента (обёртка)
#[derive(Debug, Error)]
pub enum UserManagerError {
    #[error("Failed to parse server response to JSON value: {0}")]
    ResponseJsonParseError(#[from] serde_json::Error),

    #[error("HTTP request failed: {0}")]
    HasuraClientError(#[from] HasuraClientError),

    #[error("User not found")]
    UserNotFound,

    #[error("Failed create user")]
    FailedCreateUser,

    #[error("Failed create allowed roles")]
    FailedCreateAllowedRoles,

    #[error("Failed create allowed roles")]
    FailedUpdateApiKey,
}

impl AppErrorInfo for UserManagerError {
    fn client_message(&self) -> String {
        match self {
            UserManagerError::FailedCreateUser => "Failed create user try again".to_string(),
            UserManagerError::FailedCreateAllowedRoles => {
                "Failed to assign role, contact administrator".to_string()
            }
            UserManagerError::FailedUpdateApiKey => "Failed create api key try again".to_string(),
            UserManagerError::UserNotFound => "User not found".to_string(),
            _ => self.internal_error(),
        }
    }
    fn level(&self) -> crate::domain::errors::service::ErrorLevel {
        match self {
            UserManagerError::FailedCreateUser => ErrorLevel::Error,
            UserManagerError::FailedCreateAllowedRoles => ErrorLevel::Critical,
            UserManagerError::FailedUpdateApiKey => ErrorLevel::Info,
            UserManagerError::UserNotFound => ErrorLevel::Info,
            _ => ErrorLevel::Error,
        }
    }
    fn log_message(&self) -> String {
        match self {
            UserManagerError::ResponseJsonParseError(err) => {
                format!("Failed to parse JSON from response: {err}")
            }
            UserManagerError::HasuraClientError(err) => format!("Hasura request error: {err}"),
            UserManagerError::UserNotFound => "User not found.".to_string(),
            UserManagerError::FailedCreateUser => "Failed to create user.".to_string(),
            UserManagerError::FailedCreateAllowedRoles => {
                "Failed to create allowed roles.".to_string()
            }
            UserManagerError::FailedUpdateApiKey => "Failed to update API key.".to_string(),
        }
    }
}
