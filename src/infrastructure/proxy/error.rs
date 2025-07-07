use thiserror::Error;
use uuid::Uuid;

use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};

#[derive(Debug, Error)]
pub enum ConnectionCommandError {
    #[error("Failed save access token")]
    FailedSaveAccessToken,
    #[error("Connection address not available: {0}")]
    ConnectionAddressNotAvailable(Uuid)


}

impl ConnectionCommandError {
    fn error_level(&self) -> ErrorLevel {
        ErrorLevel::Info
    }
    fn msg_inner_server_error(&self) -> String {
        format!("Inner server error")
    }
}

impl AppErrorInfo for ConnectionCommandError {
    fn client_message(&self) -> String {
        self.msg_inner_server_error()
    }

    fn level(&self) -> ErrorLevel {
        match self {
            ConnectionCommandError::FailedSaveAccessToken => ErrorLevel::Error,
            ConnectionCommandError::ConnectionAddressNotAvailable(_) => ErrorLevel::Error
        }
    }
    fn log_message(&self) -> String {
        match self {
            ConnectionCommandError::FailedSaveAccessToken => "Access token was obtained, but failed to send it to the proxy connection process.".to_string(),
            ConnectionCommandError::ConnectionAddressNotAvailable(id) => format!("Could not retrieve the address of the connection subprocess. Connection id {id}")
        }
    }
}
