use super::service::{ApiKeyVerifierService, PasswordVerifierService};

pub trait VerifiesProviderFactory {
    type PasswordVerifier: PasswordVerifierService;
    type ApiKeyVerifier: ApiKeyVerifierService;

    fn password_verifier(&self) -> Self::PasswordVerifier;
    fn api_key_verifier(&self) -> Self::ApiKeyVerifier;
}
