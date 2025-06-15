use super::dto::{
    CreateApiKeyRequestDto, CreateApiKeyResponseDto, LoginApiKeyRequestDto, LoginApiKeyResponseDto,
};
use super::ServiceErrorExt;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::settings::service::CredentialsService;
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::domain::verifies::service::{ApiKeyVerifierService, PasswordVerifierService};

const WRONG_CREDENTIALS: &str = "Incorrect login or password";
const INTERNAL_ERROR_SERVER: &str = "Internal error";

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

    fn internal_error(&self) -> Result<CreateApiKeyResponseDto, String> {
        Err(INTERNAL_ERROR_SERVER.to_string())
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

pub struct LoginApiKeyUseCase<Q, A, C, CP, TP> {
    query_user_service: Q,
    api_key_verifier: A,
    credentials_provider: C,
    claims_provider: CP,
    token_provider: TP,
}

impl<Q, A, C, CP, TP> ServiceErrorExt for LoginApiKeyUseCase<Q, A, C, CP, TP> {}

impl<Q, A, C, CP, TP> LoginApiKeyUseCase<Q, A, C, CP, TP>
where
    Q: QueryUserService,
    A: ApiKeyVerifierService,
    C: CredentialsService,
    CP: JwtClaimsService,
    TP: TokenService,
{
    pub fn new(
        query_user_service: Q,
        api_key_verifier: A,
        credentials_provider: C,
        claims_provider: CP,
        token_provider: TP,
    ) -> Self {
        Self {
            query_user_service,
            api_key_verifier,
            credentials_provider,
            claims_provider,
            token_provider,
        }
    }
    pub async fn login(
        &self,
        dto: LoginApiKeyRequestDto,
    ) -> Result<LoginApiKeyResponseDto, String> {
        let user_id = match self.api_key_verifier.extract_user_id(&dto.api_key) {
            Ok(id) => id,
            Err(e) => {
                return self.handler_error(e);
            }
        };

        let user = match self
            .query_user_service
            .get_user_by_id(&user_id.to_string())
            .await
        {
            Ok(Some(v)) => v,
            Ok(None) => return self.map_none("User not found"),
            Err(e) => return self.handler_error(e),
        };

        let Some(api_key_hash) = user.aip_key_hash() else {
            return self.map_none("Api key not allowed");
        };

        let is_verified = match self
            .api_key_verifier
            .is_verified(&api_key_hash, &dto.api_key)
        {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        if !is_verified {
            return Ok(LoginApiKeyResponseDto::Error {
                err_msg: WRONG_CREDENTIALS.to_string(),
            });
        }
        match self
            .api_key_verifier
            .is_verified(&api_key_hash, &dto.api_key)
        {
            Ok(true) => {}
            Ok(false) => {
                return Ok(LoginApiKeyResponseDto::Error {
                    err_msg: WRONG_CREDENTIALS.to_string(),
                })
            }
            Err(e) => return self.handler_error(e),
        };

        let claims = match self.claims_provider.access_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let access_token = match self.token_provider.generate_access(claims) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        Ok(LoginApiKeyResponseDto::Success {
            access_token: access_token,
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
