use std::sync::Arc;
use crate::application::auth::with_email::LoginWithEmailUseCase;
use crate::application::sign_up::with_email::SignUpWithEmailUseCase;
use crate::infrastructure::jwt::claims::ClaimsProvider;
use crate::infrastructure::user::user_manager::UserManager;
use crate::infrastructure::jwt::token::TokenProvider;
use crate::infrastructure::config::credentials_provider::CredentialsProvider;
use crate::infrastructure::verifies::password_verifier::PasswordVerifier;

type LoginUseCaseConcrete = LoginWithEmailUseCase<
    UserManager,
    PasswordVerifier,
    ClaimsProvider,
    TokenProvider,
>;

type SignUpUseCaseConcrete = SignUpWithEmailUseCase<
    UserManager,
    PasswordVerifier,
    CredentialsProvider
>;

#[derive(Clone)]
pub struct AppState {
    pub login_use_case: Arc<LoginUseCaseConcrete>,
    pub sign_up_use_case: Arc<SignUpUseCaseConcrete>
}

