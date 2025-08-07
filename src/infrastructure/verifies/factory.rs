use super::password_verifier::PasswordVerifier;
use crate::domain::settings::model::Credentials;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::infrastructure::verifies::api_key_verifier::ApiKeyVerifier;
use super::telegram_verifier::TelegramVerifier;

pub struct VerifiesProvider {
    credentials: Credentials,
}

impl VerifiesProvider {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl VerifiesProviderFactory for VerifiesProvider {
    type ApiKeyVerifier = ApiKeyVerifier;
    type PasswordVerifier = PasswordVerifier;
    type TelegramVerifierService = TelegramVerifier;
    fn api_key_verifier(&self) -> Self::ApiKeyVerifier {
        ApiKeyVerifier::new(self.credentials.clone())
    }
    fn password_verifier(&self) -> Self::PasswordVerifier {
        PasswordVerifier
    }
    fn telegram_verifier(&self) -> Self::TelegramVerifierService {
        TelegramVerifier::new(self.credentials.clone())
    }
}
