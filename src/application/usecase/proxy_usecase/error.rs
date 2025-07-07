use thiserror::Error;
use crate::application::error_dto::ComponentErrorDTO;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};

#[derive(Debug, Error)]
pub enum AuthorizeError {
    #[error("Infrastructure Error")]
    InfrastructureError(ComponentErrorDTO),
    #[error("Not found allowed auth method")]
    AuthDataNotFound,


}

impl AuthorizeError {
    fn error_level(&self) -> ErrorLevel {
        ErrorLevel::Info
    }
    fn msg_not_correct_credentials(&self) -> String {
        format!("Not correct credentials")
    }
}

impl AppErrorInfo for AuthorizeError {
    fn client_message(&self) -> String {
        match self {
            AuthorizeError::InfrastructureError(e) => {
                e.client_message()
            }
            _ => self.msg_not_correct_credentials()
        }
    }

    fn level(&self) -> ErrorLevel {
        match self {
            AuthorizeError::InfrastructureError(e) => {
                e.level()
            }
            AuthorizeError::AuthDataNotFound => {
                ErrorLevel::Info
            }
        }
    }
    fn log_message(&self) -> String {
        match self {
            AuthorizeError::InfrastructureError(e) => {
                format!("AuthorizeError {}", e.log_message())
            }
            AuthorizeError::AuthDataNotFound => {
                format!("AuthorizeError::AuthDataNotFound")
            }
        }
    }
}


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