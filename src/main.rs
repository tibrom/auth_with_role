mod domain;
mod infrastructure;
mod application;
mod interface;

use crate::application::auth::with_email::LoginWithEmailUseCase;
use crate::application::sign_up::with_email::SignUpWithEmailUseCase;
use crate::application::auth::with_apikey::{CreateApiKeyUseCase, LoginApiKeyUseCase};
use crate::domain::settings::service::CredentialsService as _;
use crate::infrastructure::jwt::claims::ClaimsProvider;
use crate::infrastructure::user::user_manager::{UserQuery, UserCommand};
use crate::infrastructure::jwt::token::TokenProvider;
use crate::infrastructure::verifies::password_verifier::PasswordVerifier;
use crate::infrastructure::verifies::api_key_verifier::ApiKeyVerifier;
use crate::infrastructure::config::credentials_provider::CredentialsProvider;

use actix_web::{App, HttpServer, web};
use interface::web::routes::auth::{login, loginapikey};
use interface::web::routes::empty::empty;
use interface::web::routes::sign_up::signup;
use interface::web::routes::auth::createapikey;
use interface::web::state::AppState;
use std::sync::Arc;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let credentials = CredentialsProvider.get_credentials().expect("CredentialsManager not allowed");

    let login_use_case = LoginWithEmailUseCase::new(
        UserQuery,
        PasswordVerifier,
        ClaimsProvider,
        TokenProvider,
    );

    let sing_up_use_case = SignUpWithEmailUseCase::new(
        UserCommand,
        PasswordVerifier,
        CredentialsProvider
    );
    
    let create_api_key_use_case = CreateApiKeyUseCase::new(
        UserCommand,
        UserQuery,
        PasswordVerifier,
        ApiKeyVerifier::new(credentials.encryption_api_key()),
        CredentialsProvider
    );

    let login_api_key_use_case = LoginApiKeyUseCase::new(
        UserQuery,
        ApiKeyVerifier::new(credentials.encryption_api_key()),
        CredentialsProvider,
        ClaimsProvider,
        TokenProvider
    );

    let app_state = AppState {
        login_use_case: Arc::new(login_use_case),
        sign_up_use_case: Arc::new(sing_up_use_case),
        create_apikey_use_case: Arc::new(create_api_key_use_case),
        login_api_key_use_case: Arc::new(login_api_key_use_case)
    };

    
    let host: String = credentials.host().clone();
    let port = credentials.port().clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(
                web::scope("/auth")
                    .service(login)
                    .service(loginapikey)
            )
            .service(signup)
            .service(createapikey)
    })
    .bind((host.clone(), port))?
    .run()
    .await
}
