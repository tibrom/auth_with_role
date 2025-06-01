use std::fmt;
use thiserror::Error;

use crate::http::errors::HasuraClientError;

/// Основная ошибка GraphQL клиента (обёртка)
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Hasura Client Error {0}")]
    HasuraError(#[from] HasuraClientError),
    #[error("Parse Json Error")]
    ParseJsonError(String),
    #[error("User not found ")]
    UserNotFound,

}