use std::fmt::format;

use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckTokenError {
    #[error("User not found")]
    UserNotFound(String),
    #[error("Auth method not valid")]
    AuthMethodNotValid(String)

    
}


impl AppErrorInfo for CheckTokenError{
    fn client_message(&self) -> String {
        match self {
            CheckTokenError::UserNotFound(_) => "Something went wrong".to_string(),
            CheckTokenError::AuthMethodNotValid(_) => "Something went wrong".to_string()
        }
    }

    fn level(&self) -> ErrorLevel {
        match self {
            CheckTokenError::UserNotFound(_) => ErrorLevel::Warning,
            CheckTokenError::AuthMethodNotValid(_) => ErrorLevel::Warning
        }
    }

    fn log_message(&self) -> String {
        match self {
            CheckTokenError::UserNotFound(id) => format!("User not found: {}", id),
            CheckTokenError::AuthMethodNotValid(id) => format!("Auth method not valid {}", id)
        }
    }
    
}