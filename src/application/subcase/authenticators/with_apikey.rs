use crate::domain::errors::service::AppErrorInfo;
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::user::service::QueryUserService;
use crate::domain::verifies::service::ApiKeyVerifierService;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::user::factories::UserProviderFactory;

use super::error::AuthenticatorError;
use super::error_dto::ComponentErrorDTO;
use super::dto::TokenPairDto;


pub struct CreateJwtWithApiKeySubCase<Q, A, CP, TP> {
    query_user_service: Q,
    api_key_verifier: A,
    claims_provider: CP,
    token_provider: TP,
}


impl<Q, A, CP, TP> CreateJwtWithApiKeySubCase<Q, A, CP, TP>
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
        U: UserProviderFactory<QueryUser = Q>
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

    pub async fn create_access(
        &self,
        api_key: String
    ) -> Result<TokenPairDto, AuthenticatorError> {
        
        let user_id = match self.api_key_verifier.extract_user_id(&api_key) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };
            
        let user = match self.query_user_service.get_user_by_id(&user_id.to_string()).await {
            Ok(Some(v)) => v,
            Ok(None) => return Err(AuthenticatorError::UserNotFound(api_key)),
            Err(e) => return self.infrastructure_error(&e)
        };

        let Some(api_key_hash) = user.aip_key_hash() else {
            return Err(AuthenticatorError::ApiKeyAuthenticatorNotAllowed(user_id.to_string()))
        };

        let is_verified = match self
            .api_key_verifier
            .is_verified(&api_key_hash, &api_key)
        {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };

        if !is_verified {
            return Err(AuthenticatorError::NotCorrectApiKey)
        }

        let claims = match self.claims_provider.access_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };

        let access_token = match self.token_provider.generate_access(claims) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };

        let token_pair = TokenPairDto {
            access_token,
            refresh_token: None,
        };
        Ok(token_pair)
    }


    fn infrastructure_error(&self, e: &dyn AppErrorInfo) -> Result<TokenPairDto, AuthenticatorError> {
        Err(AuthenticatorError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message())))
    }
}
