use std::fmt::Display;


pub trait PasswordVerifierService {
    type Error: Display;
    fn is_verified(&self, password_hash: &str, password: &str) -> Result<bool, Self::Error>;
    fn create_hash(&self, password: &str) -> Result<String, Self::Error>;
}
