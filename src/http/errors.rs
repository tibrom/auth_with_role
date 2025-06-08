use std::fmt;
use thiserror::Error;

/// Основная ошибка GraphQL клиента (обёртка)
#[derive(Debug, Error)]
pub enum HasuraClientError {
    #[error("Failed to parse server response to JSON value: {0}")]
    ResponseJsonParseError(#[from] serde_json::Error),

    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("{0}")]
    HasuraResponseError(#[from] HasuraErrorResponse),

    #[error("Failed to parse hasura response errors: {0}")]
    UnknownHasuraResponseError(String),

    #[error("GqlBuilder not found by name: {0}")]
    GqlBuilderNotFound(String),
}

/// Ошибка Hasura (десериализуется из тела ответа)
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HasuraErrorResponse {
    pub message: String,
    pub extensions: HasuraExtension,
}

impl fmt::Display for HasuraErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hasura error: {}", self.message)
    }
}

impl std::error::Error for HasuraErrorResponse {}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HasuraExtension {
    pub code: String,
    pub path: String,
    pub internal: Option<HasuraInternal>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HasuraInternal {
    pub error: HasuraInternalError,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HasuraInternalError {
    pub description: Option<String>,
    pub exec_status: String,
    pub hint: Option<String>,
    pub message: String,
    pub status_code: String,
}
