use std::fmt::format;

use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TelegramIntError {
    #[error("Telegram data is not verified")]
    LinkAccount(#[from] LinkAccountError),
    #[error("Telegram credentials are not verified")]
    AddCred(#[from] AddCredError),
}


impl AppErrorInfo for TelegramIntError{
    fn client_message(&self) -> String {
        match self {
            TelegramIntError::LinkAccount(e) => e.client_message(),
            TelegramIntError::AddCred(e) => e.client_message(),
        }
    }

    fn level(&self) -> ErrorLevel {
        match self {
            TelegramIntError::LinkAccount(e) => e.level(),
            TelegramIntError::AddCred(e) => e.level(),
        }
    }

    fn log_message(&self) -> String {
        match self {
            TelegramIntError::LinkAccount(e) => e.log_message(),
            TelegramIntError::AddCred(e) => e.log_message(),
        }
    }
    
}




#[derive(Debug, Error)]
pub enum LinkAccountError {
    #[error("Telegram data is not verified")]
    NotVerified,
    #[error("User not found by {0}")]
    UserNotFound(String),
    #[error("User doesn't have telegram credentials")]
    NoTelegramCreds,
}


impl AppErrorInfo for LinkAccountError {
    fn client_message(&self) -> String {
        match self {
            LinkAccountError::NotVerified => "Telegram data is not verified".to_string(),
            LinkAccountError::UserNotFound(v) => format!("User not found by: {}", v),
            LinkAccountError::NoTelegramCreds => "User doesn't have telegram credentials".to_string(),
        }
    }

    fn level(&self) -> ErrorLevel {
        ErrorLevel::Info
    }

    fn log_message(&self) -> String {
        self.client_message()
    }
}


#[derive(Debug, Error, Clone)]
pub enum AddCredError {
    #[error("Failed to add telegram AUTH method")]
    FailedAddingAuthMethod(String),
    #[error("Failed to add telegram User Attribute")]
    FailedAddingUserAttribute(String),
    #[error("Failed to add UserRole")]
    FailedAddingUserRole(String)
}

impl AppErrorInfo for AddCredError {
    fn client_message(&self) -> String {
        match self {
            AddCredError::FailedAddingAuthMethod(msg) => format!("Failed to add telegram AUTH method: {}", msg),
            AddCredError::FailedAddingUserAttribute(msg) => format!("Failed to add telegram User Attribute: {}", msg),
            AddCredError::FailedAddingUserRole(msg) => format!("Failed to add UserRole: {}", msg)
        }
    }

    fn level(&self) -> ErrorLevel {
        ErrorLevel::Warning
    }

    fn log_message(&self) -> String {
        self.client_message()
    }
    
}

