use crate::application::error_ext::ServiceErrorExt;
use crate::application::usecase::auth_usecase::dto::{LoginApiKeyRequestDto, LoginApiKeyResponseDto};
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::user::service::QueryUserService;
use crate::domain::verifies::service::ApiKeyVerifierService;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::user::factories::UserProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;

use super::dto::TokenPairDto;
use super::error::AuthenticatorError;

const AUTH_TYPE: &str = "apikey";

pub struct LoginWithApiKeyUseCase<Q, A, CP, TP> {
    query_user_service: Q,
    api_key_verifier: A,
    claims_provider: CP,
    token_provider: TP,
}


impl<Q, A, CP, TP> ServiceErrorExt for LoginWithApiKeyUseCase<Q, A, CP, TP> {}


impl<Q, A, CP, TP> LoginWithApiKeyUseCase<Q, A, CP, TP>
where
    Q: QueryUserService,
    A: ApiKeyVerifierService,
    CP: JwtClaimsService,
    TP: TokenService,
{
    pub fn new<T, P, U>(
        user_provider_factory: &U,
        verifies_provider_factory: &P,
        jwtprovider_factory: &T,
    ) -> Self
    where
        T: JWTProviderFactory<Claims = CP, Tokens = TP>,
        P: VerifiesProviderFactory<ApiKeyVerifier = A>,
        U: UserProviderFactory<QueryUser = Q>,
    {
        let claims_provider = jwtprovider_factory.claims_service();
        let token_provider = jwtprovider_factory.token_service();
        let api_key_verifier = verifies_provider_factory.api_key_verifier();
        let query_user_service = user_provider_factory.query_user();
        Self {
            query_user_service,
            api_key_verifier,
            claims_provider,
            token_provider,
        }
    }

    pub async fn execute(&self, dto: LoginApiKeyRequestDto) -> Result<LoginApiKeyResponseDto, String> {
        let identifier = match self.api_key_verifier.extract_identifier(&dto.api_key) {
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
            Ok(None) => return self.handler_error(AuthenticatorError::UserNotFound(dto.api_key)),
            Err(e) => return self.handler_error(e),
        };

        let Some(api_key_hash) = user.secret() else {
            return self.handler_error(AuthenticatorError::ApiKeyAuthenticatorNotAllowed(
                identifier,
            ));
        };

        let is_verified = match self.api_key_verifier.is_verified(&api_key_hash, &dto.api_key) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        if !is_verified {
            return self.handler_error(AuthenticatorError::NotCorrectApiKey);
        }

        let claims = match self.claims_provider.access_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let access_token = match self.token_provider.generate_access(claims) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let token_pair = TokenPairDto {
            access_token,
            refresh_token: None,
        };
        Ok(LoginApiKeyResponseDto::Success {
            auth_data: token_pair,
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
}
