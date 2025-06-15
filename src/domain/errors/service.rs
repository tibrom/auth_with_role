
#[derive(Debug, Clone, Copy)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
    Critical,
}

pub trait AppErrorInfo {
    fn internal_error(&self) -> String {
        "Internal Server Error".to_string()
    }
    fn level(&self) -> ErrorLevel;
    fn client_message(&self) -> String;
    fn log_message(&self) -> String;
}

