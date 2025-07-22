use crate::domain::errors::service::AppErrorInfo;
use std::fmt::Display;

pub trait PasswordVerifierService {
    type Error: Display + AppErrorInfo;
    fn is_verified(&self, password_hash: &str, password: &str) -> Result<bool, Self::Error>;
    fn create_hash(&self, password: &str) -> Result<String, Self::Error>;
}

pub trait ApiKeyVerifierService {
    type Error: AppErrorInfo;
    fn is_verified(&self, api_key_hash: &str, api_key: &str) -> Result<bool, Self::Error>;
    fn generate(&self) -> String;
    fn extract_identifier(&self, api_key: &str) -> Result<String, Self::Error>;
    fn create_hash(&self, api_key: &str) -> Result<String, Self::Error>;
}
