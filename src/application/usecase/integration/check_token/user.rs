use std::str::FromStr;

use uuid::Uuid;

use crate::application::error_ext::ServiceErrorExt;
use crate::domain::errors::service::ErrorLevel;
use crate::domain::{errors::service::AppErrorInfo, jwt::service::TokenService};
use crate::domain::jwt::factories::JWTProviderFactory;

use crate::domain::settings::model::Credentials;

use crate::domain::user::models::base::User;
use crate::domain::user::service::QueryUserService;
use crate::domain::user::factories::UserProviderFactory;

use crate::domain::verifies::service::ApiKeyVerifierService;
use crate::domain::verifies::factories::VerifiesProviderFactory;

use super::dto::{CheckTokenRequestDto, CheckTokenResponseDto};
use super::error::CheckTokenError;

const AUTH_TYPE: &str = "apikey";


pub struct CheckTokenUseCase<QUS, TS, AKV>{
    credentials: Credentials,
    query_user_service: QUS,
    token_service: TS,
    api_key_verifier: AKV,
}


impl<QUS, TS, AKV> ServiceErrorExt for CheckTokenUseCase<QUS, TS, AKV> {}


impl <QUS, TS, AKV>CheckTokenUseCase<QUS, TS, AKV>
where
    QUS: QueryUserService,
    TS: TokenService,
    AKV: ApiKeyVerifierService

{
    pub fn new<T, P, U>(
        credentials: Credentials,
        user_provider_factory: &U,
        verifies_provider_factory: &P,
        jwtprovider_factory: &T,
    ) -> Self
    where
        T: JWTProviderFactory<Tokens = TS>,
        P: VerifiesProviderFactory<ApiKeyVerifier = AKV>,
        U: UserProviderFactory<QueryUser = QUS>,
    {
        let query_user_service = user_provider_factory.query_user();
        let token_service = jwtprovider_factory.token_service();
        let api_key_verifier = verifies_provider_factory.api_key_verifier();
        Self { credentials, query_user_service, token_service, api_key_verifier }
    }
    pub async fn execute(&self, dto: CheckTokenRequestDto, api_key: String) -> Result<CheckTokenResponseDto, String> {
        println!("1");
        let identifier = match self.api_key_verifier.extract_identifier(&api_key) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };
        println!("identifier {}", identifier);
        let user = match self
            .query_user_service
            .get_user_by_identifier(&identifier, AUTH_TYPE)
            .await
        {
            Ok(Some(v)) => v,
            Ok(None) => return self.handler_error(CheckTokenError::UserNotFound(api_key)),
            Err(e) => return self.handler_error(e),
        };
        println!("user {:?}", user);

        let Some(api_key_hash) = user.secret() else {
            return self.handler_error(CheckTokenError::AuthMethodNotValid(
                identifier,
            ));
        };

        let is_verified = match self.api_key_verifier.is_verified(&api_key_hash, &api_key) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        if !is_verified {
            return self.handler_error(CheckTokenError::AuthMethodNotValid(
                identifier,
            ));
        }

        let claims = match self.token_service.validate_access(&dto.token) {
            Ok(v) => v,
            Err(e) => {
                println!("error {}", e.log_message());
                return Ok(CheckTokenResponseDto::NotValidToken)
            }
        };


        let user_id_str = claims.hasura_claims.x_hasura_user_id.clone();
        println!("user_id_str {}", user_id_str);

        let user_id = match Uuid::from_str(&user_id_str) {
            Ok(v) => v,
            Err(_) => return Ok(CheckTokenResponseDto::NotValidToken)
        };
        println!("user_id {:?}", user_id);

        let lest_auth_method = match self.query_user_service.get_user_by_id(user_id).await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        println!("lest_auth_method {:?}", lest_auth_method);

        let auth_method = match lest_auth_method.first() {
            Some(v) => v,
            None => return Ok(CheckTokenResponseDto::NotValidToken)
        };

        let user = auth_method.user().as_base();

        Ok(CheckTokenResponseDto::Success { user: user })
    }

    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<CheckTokenResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(CheckTokenResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }
    
}