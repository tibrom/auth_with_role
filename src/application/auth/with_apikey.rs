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

use super::authenticators::with_apikey::CreateJwtWithApiKeyUseCase;


const WRONG_CREDENTIALS: &str = "Incorrect login or password";

pub struct CreateApiKeyUseCase<U, Q, V, A, C> {
    user_service: U,
    query_user_service: Q,
    password_verifier: V,
    api_key_verifier: A,
    credentials_provider: C,
}

impl<U, Q, V, A, C> ServiceErrorExt for CreateApiKeyUseCase<U, Q, V, A, C> {}

impl<U, Q, V, A, C> CreateApiKeyUseCase<U, Q, V, A, C>
where
    U: CommandUserService,
    Q: QueryUserService,
    V: PasswordVerifierService,
    A: ApiKeyVerifierService,
    C: CredentialsService,
{
    pub fn new(
        user_service: U,
        query_user_service: Q,
        password_verifier: V,
        api_key_verifier: A,
        credentials_provider: C,
    ) -> Self {
        Self {
            user_service,
            query_user_service,
            password_verifier,
            api_key_verifier,
            credentials_provider,
        }
    }

    pub async fn create_api_key(
        &self,
        dto: CreateApiKeyRequestDto,
    ) -> Result<CreateApiKeyResponseDto, String> {
        let user = match self.query_user_service.get_user_by_email(&dto.email).await {
            Ok(Some(user)) => user,
            Ok(None) => return self.map_none(WRONG_CREDENTIALS),
            Err(e) => return self.handler_error(e),
        };

        let Some(password_hash) = user.password_hash() else {
            return self.map_none(WRONG_CREDENTIALS);
        };

        match self
            .password_verifier
            .is_verified(&password_hash, &dto.password)
        {
            Ok(true) => {}
            Ok(false) => return self.map_none(WRONG_CREDENTIALS),
            Err(e) => return self.handler_error(e),
        };

        let api_key_length = match self.credentials_provider.get_credentials() {
            Ok(v) => v.api_key_length().clone(),
            Err(e) => return self.handler_error(e),
        };

        let api_key = self
            .api_key_verifier
            .generate(api_key_length, user.id().clone());

        let api_key_hash = match self.api_key_verifier.create_hash(&api_key) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        if let Err(e) = self
            .user_service
            .add_api_hash(&user.id().to_string(), &api_key_hash)
            .await
        {
            return self.handler_error(e);
        };

        Ok(CreateApiKeyResponseDto::Success { api_key })
    }


    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<CreateApiKeyResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(CreateApiKeyResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }

    fn map_none(&self, msg: &str) -> Result<CreateApiKeyResponseDto, String> {
        Ok(CreateApiKeyResponseDto::Error {
            err_msg: msg.to_string(),
        })
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

        let create_jwt_use_case = CreateJwtWithApiKeyUseCase::new(
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
