use super::dto::{
    CreateApiKeyRequestDto, CreateApiKeyResponseDto, LoginApiKeyRequestDto, LoginApiKeyResponseDto,
};
use super::ServiceErrorExt;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::settings::service::CredentialsService;
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::domain::verifies::service::{ApiKeyVerifierService, PasswordVerifierService};

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::user::factories::UserProviderFactory;

use super::subcase::authenticators::with_apikey::CreateJwtWithApiKeySubCase;
use super::subcase::user_attribute::api_key::CreateApiKeyByEmailPasswdSubCase;


const WRONG_CREDENTIALS: &str = "Incorrect login or password";

pub struct CreateApiKeyUseCase<V, U> {
    verifies_provider_factory: V,
    user_provider_factory: U,
}

impl<V, U> ServiceErrorExt for CreateApiKeyUseCase<V, U> {}

impl<V, U> CreateApiKeyUseCase<V, U>
where
    V: VerifiesProviderFactory,
    U: UserProviderFactory
{
    pub fn new(
        verifies_provider_factory: V,
        user_provider_factory: U,
    ) -> Self {
        Self {
            verifies_provider_factory,
            user_provider_factory
        }
    }

    pub async fn create_api_key(
        &self,
        dto: CreateApiKeyRequestDto,
    ) -> Result<CreateApiKeyResponseDto, String> {
        let create_api_key_sub_case = CreateApiKeyByEmailPasswdSubCase::new(
            &self.user_provider_factory,
            &self.verifies_provider_factory
        );

        let api_key = match create_api_key_sub_case.execute(dto.email.clone(), dto.password).await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };


        Ok(CreateApiKeyResponseDto::Success { auth_data: api_key })
    }


    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<CreateApiKeyResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(CreateApiKeyResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }
}

pub struct LoginApiKeyUseCase<J, V, U> {
    jwtprovider_factory: J,
    verifies_provider_factory: V,
    user_provider_factory: U,
}

impl<J, V, U> ServiceErrorExt for LoginApiKeyUseCase<J, V, U> {}

impl<J, V, U> LoginApiKeyUseCase<J, V, U>
where
    J: JWTProviderFactory,
    V: VerifiesProviderFactory,
    U: UserProviderFactory
{
    pub fn new(
        jwtprovider_factory: J,
        verifies_provider_factory: V,
        user_provider_factory: U,
    ) -> Self {
        Self {
            jwtprovider_factory,
            verifies_provider_factory,
            user_provider_factory,
        }
    }
    pub async fn login(
        &self,
        dto: LoginApiKeyRequestDto,
    ) -> Result<LoginApiKeyResponseDto, String> {

        let create_jwt_use_case = CreateJwtWithApiKeySubCase::new(
            &self.user_provider_factory,
            &self.verifies_provider_factory,
            &self.jwtprovider_factory
        );

        let token_pair_dto = match create_jwt_use_case.create_access(dto.api_key).await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };


        Ok(LoginApiKeyResponseDto::Success {
            auth_data: token_pair_dto
        })
    }

    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<LoginApiKeyResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(LoginApiKeyResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }

    fn map_none(&self, msg: &str) -> Result<LoginApiKeyResponseDto, String> {
        Ok(LoginApiKeyResponseDto::Error {
            err_msg: msg.to_string(),
        })
    }
}
