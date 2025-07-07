use crate::application::error_dto::ComponentErrorDTO;
use crate::domain::errors::service::AppErrorInfo;
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::proxy::model::AuthData;
use crate::domain::user::service::QueryUserService;
use crate::domain::verifies::service::ApiKeyVerifierService;
use crate::application::actions::authenticators::with_apikey::CreateJwtWithApiKeyAction;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::user::factories::UserProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;

use super::error::AuthorizeError;

pub struct AuthorizeUseCase<Q, A, CP, TP> {
    create_jwt_sub_case: CreateJwtWithApiKeyAction<Q, A, CP, TP>
}

impl <Q, A, CP, TP>AuthorizeUseCase<Q, A, CP, TP> 
where
    Q: QueryUserService,
    A: ApiKeyVerifierService,
    CP: JwtClaimsService,
    TP: TokenService, {
    
    pub fn new<T, P, U>(
        user_provider_factory: &U,
        verifies_provider_factory: &P,
        jwtprovider_factory: &T,
    ) -> Self
    where
        T: JWTProviderFactory<Claims = CP, Tokens = TP>,
        P: VerifiesProviderFactory<ApiKeyVerifier = A>,
        U: UserProviderFactory<QueryUser = Q> {
        Self { create_jwt_sub_case: CreateJwtWithApiKeyAction::new(user_provider_factory, verifies_provider_factory, jwtprovider_factory) }
    }

    pub async  fn execute (&self, auth_data: AuthData) -> Result<String, AuthorizeError>{
        let token = match auth_data {
            AuthData::ApiKey(v) => self.authorize_by_api_key(&v).await,
            AuthData::None => return Err(AuthorizeError::AuthDataNotFound)
        }?;
        Ok(token)
    }

    async fn authorize_by_api_key(&self, api_key: &str) -> Result<String, AuthorizeError> {

        let token_pair_dto = match self.create_jwt_sub_case.execute(api_key.to_owned()).await {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };
        Ok(token_pair_dto.access_token.clone())
    }


    fn infrastructure_error(&self, e: &dyn AppErrorInfo) -> Result<String, AuthorizeError> {
        Err(AuthorizeError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message())))
    }


}

