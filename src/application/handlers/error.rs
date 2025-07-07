use thiserror::Error;
use crate::application::error_dto::ComponentErrorDTO;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};

#[derive(Debug, Error)]
pub enum UpdateStateError {
    #[error("Infrastructure Error")]
    InfrastructureError(ComponentErrorDTO),
}

impl UpdateStateError {
    fn error_level(&self) -> ErrorLevel {
        ErrorLevel::Info
    }
    fn msg_inner_server_error(&self) -> String {
        format!("Inner server error")
    }
}

impl AppErrorInfo for UpdateStateError {
    fn client_message(&self) -> String {
        match self {
            UpdateStateError::InfrastructureError(e) => {
                e.client_message()
            }
            _ => self.msg_inner_server_error()
        }
    }

    fn level(&self) -> ErrorLevel {
        match self {
            UpdateStateError::InfrastructureError(e) => {
                e.level()
            }
        }
    }
    fn log_message(&self) -> String {
        match self {
            UpdateStateError::InfrastructureError(e) => {
                format!("UpdateStateError {}", e.log_message())
            }
        }
    }
}