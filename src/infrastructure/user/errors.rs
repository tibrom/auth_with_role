use std::fmt;
use thiserror::Error;

/// Основная ошибка GraphQL клиента (обёртка)
#[derive(Debug, Error)]
pub enum UserManagerError {
    #[error("Failed to parse server response to JSON value: {0}")]
    ResponseJsonParseError(#[from] serde_json::Error),

    #[error("HTTP request failed: {0}")]
    HasuraClientError(#[from] super::hasura::errors::HasuraClientError),

    #[error("User not found")]
    UserNotFound,

    #[error("Failed create user")]
    FailedCreateUser,

    #[error("Failed create allowed roles")]
    FailedCreateAllowedRoles,

    #[error("Failed create allowed roles")]
    FailedUpdateApiKey
}
