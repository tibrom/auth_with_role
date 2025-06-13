mod domain;
mod infrastructure;
mod application;
mod interface;

use crate::application::auth::with_email::LoginWithEmailUseCase;
use crate::application::sign_up::with_email::SignUpWithEmailUseCase;
use crate::domain::settings::service::CredentialsService as _;
use crate::infrastructure::jwt::claims::ClaimsProvider;
use crate::infrastructure::user::user_manager::UserManager;
use crate::infrastructure::jwt::token::TokenProvider;
use crate::infrastructure::verifies::password_verifier::PasswordVerifier;
use crate::infrastructure::config::credentials_provider::CredentialsProvider;

use actix_web::{App, HttpServer, web};
use interface::web::routes::auth::login;
use interface::web::routes::empty::empty;
use interface::web::routes::sign_up::signup;
use interface::web::state::AppState;
use std::sync::Arc;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    // инфраструктура
    let user_provider = UserManager;
    let verifier = PasswordVerifier;
    let claims_provider = ClaimsProvider;
    let token_provider = TokenProvider;

    let login_use_case = LoginWithEmailUseCase::new(
        user_provider,
        verifier,
        claims_provider,
        token_provider,
    );

    let sing_up_use_case = SignUpWithEmailUseCase::new(
        UserManager, PasswordVerifier, CredentialsProvider);


    let app_state = AppState {
        login_use_case: Arc::new(login_use_case),
        sign_up_use_case:Arc::new(sing_up_use_case)
    };

    let credentials = CredentialsProvider.get_credentials().expect("CredentialsManager not allowed");
    let host: String = credentials.host().clone();
    let port = credentials.port().clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(
                web::scope("/auth")
                    .service(login)
            )
            .service(signup)
    })
    .bind((host.clone(), port))?
    .run()
    .await
}
