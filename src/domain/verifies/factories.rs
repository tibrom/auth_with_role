use super::service::{ApiKeyVerifierService, PasswordVerifierService};

pub trait VerifiesProviderFactory {
    type PasswordVerifier: PasswordVerifierService + Send;
    type ApiKeyVerifier: ApiKeyVerifierService + Send;

    fn password_verifier(&self) -> Self::PasswordVerifier;
    fn api_key_verifier(&self) -> Self::ApiKeyVerifier;
}
