use crate::application::auth_usecase::with_apikey::{CreateApiKeyUseCase, LoginApiKeyUseCase};
use crate::application::auth_usecase::with_email::LoginWithEmailUseCase;
use crate::application::sign_up_usecase::with_email::SignUpWithEmailUseCase;
use crate::infrastructure::config::credentials_provider::CredentialsProvider;
use crate::infrastructure::jwt::claims::ClaimsProvider;
use crate::infrastructure::jwt::token::TokenProvider;
use crate::infrastructure::user::user_manager::{UserCommand, UserQuery};
use crate::infrastructure::verifies::api_key_verifier::ApiKeyVerifier;
use crate::infrastructure::verifies::password_verifier::PasswordVerifier;

use crate::infrastructure::user::factory::UserProvider;
use crate::infrastructure::verifies::factory::VerifiesProvider;
use crate::infrastructure::jwt::factory::JWTProvider;

use std::sync::Arc;

type LoginUseCaseConcrete = LoginWithEmailUseCase<
    JWTProvider,
    VerifiesProvider,
    UserProvider
>;

type SignUpUseCaseConcrete =
    SignUpWithEmailUseCase<VerifiesProvider, UserProvider>;

type LoginApiKeyUseCaseConcrete = LoginApiKeyUseCase<
    JWTProvider,
    VerifiesProvider,
    UserProvider,
>;

type CreateApiKeyUseCaseConcrete = CreateApiKeyUseCase<
    VerifiesProvider,
    UserProvider,
>;

#[derive(Clone)]
pub struct AppState {
    pub login_use_case: Arc<LoginUseCaseConcrete>,
    pub sign_up_use_case: Arc<SignUpUseCaseConcrete>,
    pub create_apikey_use_case: Arc<CreateApiKeyUseCaseConcrete>,
    pub login_api_key_use_case: Arc<LoginApiKeyUseCaseConcrete>,
}

