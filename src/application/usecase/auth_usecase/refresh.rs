use uuid::Uuid;

use crate::application::error_ext::ServiceErrorExt;
use crate::application::usecase::auth_usecase::dto::{JwtResponseDto, RefreshTokenRequestDto};
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::user::service::QueryUserService;
use crate::domain::verifies::service::PasswordVerifierService;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::user::factories::UserProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;



use super::dto::TokenPairDto;
use super::error::AuthenticatorError;

pub struct RefreshTokenUseCase<Q, V, CP, TP> {
    user_provider: Q,
    password_verifier: V,
    claims_provider: CP,
    token_provider: TP,
}


impl<Q, V, CP, TP> ServiceErrorExt for RefreshTokenUseCase<Q, V, CP, TP> {}


impl<Q, V, CP, TP> RefreshTokenUseCase<Q, V, CP, TP>
where
    V: PasswordVerifierService,
    TP: TokenService,
    CP: JwtClaimsService,
    Q: QueryUserService,
{
    pub fn new<T, P, U>(
        user_provider_factory: &U,
        verifies_provider_factory: &P,
        jwtprovider_factory: &T,
    ) -> Self
    where
        T: JWTProviderFactory<Claims = CP, Tokens = TP>,
        P: VerifiesProviderFactory<PasswordVerifier = V>,
        U: UserProviderFactory<QueryUser = Q>,
    {
        let claims_provider = jwtprovider_factory.claims_service();
        let token_provider = jwtprovider_factory.token_service();
        let password_verifier = verifies_provider_factory.password_verifier();
        let user_provider = user_provider_factory.query_user();
        Self {
            user_provider,
            password_verifier,
            claims_provider,
            token_provider,
        }
    }

    pub async fn execute(&self, dto: RefreshTokenRequestDto) -> Result<JwtResponseDto, String> {
        let refresh_claims =  match self.token_provider.validate_refresh(&dto.refresh_token){
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        let user_id = match Uuid::try_parse(&refresh_claims.sub) {
            Ok(v) => v,
            Err(e) => return self.handler_error(AuthenticatorError::NotCorrectRefreshToken)
        };

        let user_data = match self.user_provider.get_user_by_id(user_id.clone()).await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let Some(user) = user_data.first() else {
            return self.handler_error(AuthenticatorError::UserNotFound(user_id.to_string()));
        };

        let claims = match self.claims_provider.access_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let refresh_claims = match self.claims_provider.refresh_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let access_token = match self.token_provider.generate_access(claims) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let refresh_token = match self.token_provider.generate_refresh(refresh_claims) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let token_pair = TokenPairDto {
            access_token,
            refresh_token: Some(refresh_token),
        };

        Ok(JwtResponseDto::Success {
            auth_data: token_pair,
        })
        
    }

    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<JwtResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(JwtResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }
}
