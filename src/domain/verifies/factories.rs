use super::service::{ApiKeyVerifierService, PasswordVerifierService};
use super::super::integration::telegram::verifier::TelegramVerifierService;

pub trait VerifiesProviderFactory {
    type PasswordVerifier: PasswordVerifierService + Send;
    type ApiKeyVerifier: ApiKeyVerifierService + Send;
    type TelegramVerifierService: TelegramVerifierService + Send;

    fn password_verifier(&self) -> Self::PasswordVerifier;
    fn api_key_verifier(&self) -> Self::ApiKeyVerifier;
    fn telegram_verifier(&self) -> Self::TelegramVerifierService;
}
