use crate::application::usecase::{
    auth_usecase::{
        refresh::RefreshTokenUseCase,
        email_passwd::LoginWithEmailPasswdUseCase,
        api_key::LoginWithApiKeyUseCase
    },
    sign_up_usecase::{
        api_key::CreateApiKeyUseCase,
        email_passwd::SignUpWithEmailUseCase
    },
    integration::telegram::{
        link_account::LinkTelegramAccountUseCase,
        auth::AuthTelegramUseCase
    }

};

use crate::infrastructure::user::user_manager::{UserQuery, UserCommand};
use crate::infrastructure::verifies::password_verifier::PasswordVerifier;
use crate::infrastructure::verifies::api_key_verifier::ApiKeyVerifier;
use crate::infrastructure::verifies::telegram_verifier::TelegramVerifier;
use crate::infrastructure::jwt::claims::ClaimsProvider;
use crate::infrastructure::jwt::token::TokenProvider;

use crate::infrastructure::network::http::client::HttpClient;



use std::sync::Arc;

type LoginWithEmailPasswdUseCaseConcrete = LoginWithEmailPasswdUseCase<
    UserQuery<HttpClient>, PasswordVerifier, ClaimsProvider, TokenProvider
>;

type RefreshTokenUseCaseConcrete = RefreshTokenUseCase<
    UserQuery<HttpClient>, PasswordVerifier, ClaimsProvider, TokenProvider
>;

type LoginWithApiKeyUseCaseConcrete = LoginWithApiKeyUseCase<
    UserQuery<HttpClient>, ApiKeyVerifier, ClaimsProvider, TokenProvider
>;

type CreateApiKeyUseCaseConcrete = CreateApiKeyUseCase<
    UserCommand<HttpClient>, UserQuery<HttpClient>, PasswordVerifier, ApiKeyVerifier
>;

type SignUpWithEmailUseCaseConcrete = SignUpWithEmailUseCase<UserCommand<HttpClient>, PasswordVerifier>;

type LinkTelegramAccountUseCaseConcrete = LinkTelegramAccountUseCase<UserCommand<HttpClient>, UserQuery<HttpClient>, TelegramVerifier, ClaimsProvider, TokenProvider>;

type AuthTelegramUseCaseConcrete = AuthTelegramUseCase<UserCommand<HttpClient>, UserQuery<HttpClient>, TelegramVerifier, ClaimsProvider, TokenProvider>;


#[derive(Clone)]
pub struct AppState {
    pub login_with_email_passwd_use_case: Arc<LoginWithEmailPasswdUseCaseConcrete>,
    pub refresh_token_use_case: Arc<RefreshTokenUseCaseConcrete>,
    pub login_with_api_key_use_case: Arc<LoginWithApiKeyUseCaseConcrete>,
    pub create_api_key_use_case: Arc<CreateApiKeyUseCaseConcrete>,
    pub sign_up_with_email_use_case: Arc<SignUpWithEmailUseCaseConcrete>,
    pub link_telegram_account_use_case: Arc<LinkTelegramAccountUseCaseConcrete>,
    pub auth_telegram_use_case: Arc<AuthTelegramUseCaseConcrete>
}

