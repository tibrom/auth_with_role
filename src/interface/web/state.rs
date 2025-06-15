use std::sync::Arc;
use crate::application::auth::with_email::LoginWithEmailUseCase;
use crate::application::sign_up::with_email::SignUpWithEmailUseCase;
use crate::application::auth::with_apikey::{CreateApiKeyUseCase, LoginApiKeyUseCase};
use crate::infrastructure::jwt::claims::ClaimsProvider;
use crate::infrastructure::user::user_manager::{UserQuery, UserCommand};
use crate::infrastructure::jwt::token::TokenProvider;
use crate::infrastructure::config::credentials_provider::CredentialsProvider;
use crate::infrastructure::verifies::password_verifier::PasswordVerifier;
use crate::infrastructure::verifies::api_key_verifier::ApiKeyVerifier;


type LoginUseCaseConcrete = LoginWithEmailUseCase<
    UserQuery,
    PasswordVerifier,
    ClaimsProvider,
    TokenProvider,
>;

type SignUpUseCaseConcrete = SignUpWithEmailUseCase<
    UserCommand,
    PasswordVerifier,
    CredentialsProvider
>;

type LoginApiKeyUseCaseConcrete = LoginApiKeyUseCase<
    UserQuery,
    ApiKeyVerifier,
    CredentialsProvider,
    ClaimsProvider,
    TokenProvider
>;

type CreateApiKeyUseCaseConcrete = CreateApiKeyUseCase<
    UserCommand,
    UserQuery,
    PasswordVerifier,
    ApiKeyVerifier,
    CredentialsProvider
>;

#[derive(Clone)]
pub struct AppState {
    pub login_use_case: Arc<LoginUseCaseConcrete>,
    pub sign_up_use_case: Arc<SignUpUseCaseConcrete>,
    pub create_apikey_use_case: Arc<CreateApiKeyUseCaseConcrete>,
    pub login_api_key_use_case: Arc<LoginApiKeyUseCaseConcrete>
}

