use crate::domain::verifies::service::PasswordVerifierService;

use super::errors::PasswordVerifierError;

pub struct PasswordVerifier;

impl PasswordVerifierService for PasswordVerifier  {
    type Error = PasswordVerifierError;
    fn is_verified(&self, password_hash: &str, password: &str) -> Result<bool, PasswordVerifierError> {
        bcrypt::verify(password, password_hash)
            .map_err(|e|PasswordVerifierError::HashPasswordCryptError { stage: "bcrypt::verify", source: e })
    }
    fn create_hash(&self, password: &str) -> Result<String, Self::Error> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e|PasswordVerifierError::HashPasswordCryptError { stage: "bcrypt::verify", source: e })
    }
}