use std::fmt;
use thiserror::Error;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};



#[derive(Debug, Clone)]
pub struct AuthErrorDTO {
    level: ErrorLevel,
    log_message: String,
    client_message: String
}

impl AuthErrorDTO {
    pub fn new(
        level: ErrorLevel,
        log_message: String,
        client_message: String
    ) -> Self {
        Self { level, log_message, client_message }
    }
}

impl From<&dyn AppErrorInfo> for AuthErrorDTO {
    fn from(value: &dyn AppErrorInfo) -> Self {
        Self { 
            level: value.level(),
            log_message: value.log_message(),
            client_message: value.client_message(),
        }
    }
}

impl AppErrorInfo for AuthErrorDTO {
    fn client_message(&self) -> String {
        self.client_message.clone()
    }
    fn log_message(&self) -> String {
        self.log_message.clone()
    }
    fn level(&self) -> ErrorLevel {
        self.level.clone()
    }
}
