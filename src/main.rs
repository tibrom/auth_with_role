mod application;
mod domain;
mod infrastructure;
mod interface;
mod mock;

use crate::application::usecase::{
    auth_usecase::{
        refresh::RefreshTokenUseCase,
        email_passwd::LoginWithEmailPasswdUseCase,
        api_key::LoginWithApiKeyUseCase
    },
    sign_up_usecase::{
        api_key::CreateApiKeyUseCase,
        email_passwd::SignUpWithEmailUseCase
    }
};

use crate::domain::settings::service::CredentialsService as _;
use crate::infrastructure::config::credentials_provider::CredentialsProvider;

use crate::infrastructure::jwt::factory::JWTProvider;
use crate::infrastructure::user::factory::UserProvider;
use crate::infrastructure::verifies::factory::VerifiesProvider;
use crate::infrastructure::network::client_manager::HasuraClientManager;

use actix_web::{web, App, HttpServer};
use interface::web::routes::auth::createapikey;
use interface::web::routes::auth::{login, loginapikey, refresh};
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


    let hasura_client= HasuraClientManager::get_hasura_client(&credentials)
        .await
        .expect("Hasura client not allowed");

    let jwtprovider_factory = JWTProvider::new(credentials.clone());
    let verifies_provider_factory = VerifiesProvider::new(credentials.clone());
    let user_provider_factory = UserProvider::new(credentials.clone(), hasura_client.clone());

    let login_with_email_passwd_use_case = LoginWithEmailPasswdUseCase::new(
        &user_provider_factory,
        &verifies_provider_factory,
        &jwtprovider_factory
    );

    let refresh_token_use_case = RefreshTokenUseCase::new(
        &user_provider_factory,
        &verifies_provider_factory,
        &jwtprovider_factory
    );

    let login_with_api_key_use_case = LoginWithApiKeyUseCase::new(
        &user_provider_factory,
        &verifies_provider_factory,
        &jwtprovider_factory
    );

    let create_api_key_use_case = CreateApiKeyUseCase::new(
        &user_provider_factory,
        &verifies_provider_factory
    );

    let sign_up_with_email_use_case = SignUpWithEmailUseCase::new(
        credentials.clone(),
        &verifies_provider_factory,
        &user_provider_factory
    );


    let app_state = AppState{
        login_with_email_passwd_use_case: Arc::new(login_with_email_passwd_use_case),
        refresh_token_use_case: Arc::new(refresh_token_use_case),
        login_with_api_key_use_case: Arc::new(login_with_api_key_use_case),
        create_api_key_use_case: Arc::new(create_api_key_use_case),
        sign_up_with_email_use_case: Arc::new(sign_up_with_email_use_case)
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
                    .service(refresh),
            )
            .service(signup)
            .service(createapikey)
    })
    .bind((host.clone(), port))?
    .run()
    .await
}
