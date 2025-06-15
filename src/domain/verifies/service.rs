use std::fmt::Display;

use uuid::Uuid;


pub trait PasswordVerifierService {
    type Error: Display;
    fn is_verified(&self, password_hash: &str, password: &str) -> Result<bool, Self::Error>;
    fn create_hash(&self, password: &str) -> Result<String, Self::Error>;
}


pub trait ApiKeyVerifierService {
    type Error: std::error::Error + Send + Sync + 'static;
    fn is_verified(&self, api_key_hash: &str, api_key: &str) -> Result<bool, Self::Error>;
    fn generate(&self, length: u16, user_id: Uuid) -> String;
    fn extract_user_id(&self, api_key: &str) -> Result<Uuid, Self::Error>;
    fn create_hash(&self, api_key: &str) -> Result<String, Self::Error>;
}

