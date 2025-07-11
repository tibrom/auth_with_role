use crate::domain::verifies::service::PasswordVerifierService;

use super::errors::PasswordVerifierError;

pub struct PasswordVerifier;

impl PasswordVerifierService for PasswordVerifier {
    type Error = PasswordVerifierError;
    fn is_verified(
        &self,
        password_hash: &str,
        password: &str,
    ) -> Result<bool, PasswordVerifierError> {
        bcrypt::verify(password, password_hash).map_err(|e| {
            PasswordVerifierError::HashPasswordCryptError {
                stage: "bcrypt::verify",
                source: e,
            }
        })
    }
    fn create_hash(&self, password: &str) -> Result<String, Self::Error> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
            PasswordVerifierError::HashPasswordCryptError {
                stage: "bcrypt::verify",
                source: e,
            }
        })
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::verifies::service::PasswordVerifierService;

    #[test]
    fn test_valid_password() {
        let verifier = PasswordVerifier;
        let password = "Password123";
        let hash = verifier.create_hash(password).unwrap();

        let is_valid = verifier.is_verified(&hash, password).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_invalid_password() {
        let verifier = PasswordVerifier;
        let password = "Password123";
        let invalid_password = "InvalidPassword123";
        let hash = verifier.create_hash(password).unwrap();

        let is_valid = verifier.is_verified(&hash, invalid_password).unwrap();
        assert!(!is_valid);
    }
}

