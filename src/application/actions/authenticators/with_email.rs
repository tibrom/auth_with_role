use crate::domain::errors::service::AppErrorInfo;
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::user::service::QueryUserService;
use crate::domain::verifies::service::PasswordVerifierService;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::user::factories::UserProviderFactory;

use crate::application::error_dto::ComponentErrorDTO;

use super::error::AuthenticatorError;
use super::dto::TokenPairDto;

pub struct CreateJwtWithEmailPasswdAction<Q, V, CP, TP> {
    user_provider: Q,
    password_verifier: V,
    claims_provider: CP,
    token_provider: TP,
}


impl<Q, V, CP, TP> CreateJwtWithEmailPasswdAction<Q, V, CP, TP>
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
        U: UserProviderFactory<QueryUser = Q>
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

    pub async fn execute(
        &self,
        email: String,
        password: String
    ) -> Result<TokenPairDto, AuthenticatorError> {
        let user = match self.user_provider.get_user_by_email(&email).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(AuthenticatorError::UserNotFound(email)),
            Err(e) => return self.infrastructure_error(&e)
        };

        let Some(password_hash) = user.password_hash() else {
            return Err(AuthenticatorError::EmailPasswdAuthNotAllowed(email))
        };

        match self.password_verifier.is_verified(&password_hash, &password) {
            Ok(true) => {}
            Ok(false) => return Err(AuthenticatorError::NotCorrectPassword),
            Err(e) => return self.infrastructure_error(&e)
        };

        let claims = match self.claims_provider.access_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };

        let refresh_claims = match self.claims_provider.refresh_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };

        let access_token = match self.token_provider.generate_access(claims) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };

        let refresh_token = match self.token_provider.generate_refresh(refresh_claims) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };

        let token_pair = TokenPairDto {
            access_token,
            refresh_token: Some(refresh_token),
        };
        Ok(token_pair)
    }


    fn infrastructure_error(&self, e: &dyn AppErrorInfo) -> Result<TokenPairDto, AuthenticatorError> {
        Err(AuthenticatorError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message())))
    }
}
