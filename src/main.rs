mod application;
mod domain;
mod infrastructure;
mod interface;

use crate::application::auth::with_apikey::{CreateApiKeyUseCase, LoginApiKeyUseCase};
use crate::application::auth::with_email::LoginWithEmailUseCase;
use crate::application::sign_up::with_email::SignUpWithEmailUseCase;
use crate::domain::settings::service::CredentialsService as _;
use crate::infrastructure::config::credentials_provider::CredentialsProvider;
use crate::infrastructure::jwt::claims::ClaimsProvider;
use crate::infrastructure::jwt::token::TokenProvider;
use crate::infrastructure::user::user_manager::{UserCommand, UserQuery};
use crate::infrastructure::verifies::api_key_verifier::ApiKeyVerifier;
use crate::infrastructure::verifies::password_verifier::PasswordVerifier;

use crate::infrastructure::verifies::factory::VerifiesProvider;
use crate::infrastructure::jwt::factory::JWTProvider;
use crate::infrastructure::user::factory::UserProvider;

use actix_web::{web, App, HttpServer};
use interface::web::routes::auth::createapikey;
use interface::web::routes::auth::{login, loginapikey};
use interface::web::routes::sign_up::signup;
use interface::web::state::AppState;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let credentials = CredentialsProvider
        .get_credentials()
        .expect("CredentialsManager not allowed");

    let login_use_case = LoginWithEmailUseCase::new(
        JWTProvider::new(credentials.clone()),
        VerifiesProvider::new(credentials.clone()),
        UserProvider
    );

    let sing_up_use_case =
        SignUpWithEmailUseCase::new(UserCommand, PasswordVerifier, CredentialsProvider);

    let create_api_key_use_case = CreateApiKeyUseCase::new(
        UserCommand,
        UserQuery,
        PasswordVerifier,
        ApiKeyVerifier::new(credentials.clone()),
        CredentialsProvider,
    );

    let login_api_key_use_case = LoginApiKeyUseCase::new(
        JWTProvider::new(credentials.clone()),
        VerifiesProvider::new(credentials.clone()),
        UserProvider
    );

    let app_state = AppState {
        login_use_case: Arc::new(login_use_case),
        sign_up_use_case: Arc::new(sing_up_use_case),
        create_apikey_use_case: Arc::new(create_api_key_use_case),
        login_api_key_use_case: Arc::new(login_api_key_use_case),
    };

    let host: String = credentials.host().clone();
    let port = credentials.port().clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(web::scope("/auth").service(login).service(loginapikey))
            .service(signup)
            .service(createapikey)
    })
    .bind((host.clone(), port))?
    .run()
    .await
}
