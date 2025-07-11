use crate::domain::settings::model::Credentials;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::infrastructure::verifies::api_key_verifier::ApiKeyVerifier;
use super::password_verifier::PasswordVerifier;




pub struct VerifiesProvider{
    credentials: Credentials,
}

impl VerifiesProvider {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl VerifiesProviderFactory for VerifiesProvider  {
    type ApiKeyVerifier = ApiKeyVerifier;
    type PasswordVerifier = PasswordVerifier;
    fn api_key_verifier(&self) -> Self::ApiKeyVerifier {
        ApiKeyVerifier::new(self.credentials.clone())
    }
    fn password_verifier(&self) -> Self::PasswordVerifier {
        PasswordVerifier
    }
}