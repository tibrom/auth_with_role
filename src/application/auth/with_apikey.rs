use super::dto::{CreateApiKeyRequestDto, CreateApiKeyResponseDto, LoginApiKeyRequestDto, LoginApiKeyResponseDto};
use crate::application::auth::dto::TokenPairDto;
use crate::domain::verifies::service::{PasswordVerifierService, ApiKeyVerifierService};
use crate::domain::jwt::service::{TokenService, JwtClaimsService};
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::domain::settings::service::CredentialsService;
use crate::domain::error::service::ErrorService;

const WRONG_CREDENTIALS: &str = "Incorrect login or password";
const INTERNAL_ERROR_SERVER: &str = "Internal error";


pub struct CreateApiKeyUseCase<U, Q, V, A, C>{
    user_service: U,
    query_user_service: Q,
    password_verifier: V,
    api_key_verifier: A,
    credentials_provider: C
}

impl<U, Q, V, A, C> CreateApiKeyUseCase<U, Q, V, A, C>
where
    U: CommandUserService,
    Q: QueryUserService,
    V: PasswordVerifierService,
    A: ApiKeyVerifierService,
    C: CredentialsService {

    pub fn new(user_service: U, query_user_service: Q, password_verifier: V, api_key_verifier: A, credentials_provider: C) -> Self {
        Self {
            user_service,
            query_user_service,
            password_verifier,
            api_key_verifier,
            credentials_provider
        }
    }

    pub async fn create_api_key(&self, dto: CreateApiKeyRequestDto) -> Result<CreateApiKeyResponseDto, String> {
        let user = match self.query_user_service.get_user_by_email(&dto.email).await {
            Ok(Some(user)) => user,
            Ok(None) => return Ok(CreateApiKeyResponseDto::Error { err_msg: WRONG_CREDENTIALS.to_string() }),
            Err(e) => {
                tracing::error!("Search User Err: {e}");
                return self.internal_error()
            },
        };

        
        let Some(password_hash) = user.password_hash() else {
            return Ok(CreateApiKeyResponseDto::Error { err_msg: WRONG_CREDENTIALS.to_string() });
        };

        match self.password_verifier.is_verified(&password_hash, &dto.password) {
            Ok(true) => {},
            Ok(false) => return Ok(CreateApiKeyResponseDto::Error { err_msg: WRONG_CREDENTIALS.to_string() }),
            Err(e) => {
                tracing::error!("Verifier Err: {e}");
                return self.internal_error();
            },
        };

        let api_key_length = self.credentials_provider.get_credentials()
            .map(|v| v.api_key_length().clone())
            .map_err(|e| format!("Credentials not allowed"))?;
        
        let api_key = self.api_key_verifier.generate(api_key_length, user.id().clone());
        let api_key_hash = self.api_key_verifier.create_hash(&api_key)
            .map_err(|e| "Error create api key hash")?;
        
        _ = self.user_service.add_api_hash(&user.id().to_string(), &api_key_hash).await
            .map_err(|e| "Error save key hash")?;

        Ok(CreateApiKeyResponseDto::Success { api_key })
    }

    fn internal_error(&self) -> Result<CreateApiKeyResponseDto, String> {
        Err(INTERNAL_ERROR_SERVER.to_string())
    }
}



pub struct LoginApiKeyUseCase<Q, A, C, CP, TP, E>{
    query_user_service: Q,
    api_key_verifier: A,
    credentials_provider: C,
    claims_provider: CP,
    token_provider: TP,
    error_service: E
}

impl<Q, A, C, CP, TP, E> LoginApiKeyUseCase<Q, A, C, CP, TP, E>
where
    Q: QueryUserService,
    A: ApiKeyVerifierService,
    C: CredentialsService,
    CP: JwtClaimsService,
    TP: TokenService,
    E: ErrorService {

    pub fn new(
        query_user_service: Q,
        api_key_verifier: A,
        credentials_provider: C,
        claims_provider: CP,
        token_provider: TP,
        error_service: E
    ) -> Self {
        Self {
            query_user_service,
            api_key_verifier,
            credentials_provider,
            claims_provider,
            token_provider,
            error_service
        }
    }
    pub async fn login(&self, dto: LoginApiKeyRequestDto) -> Result<LoginApiKeyResponseDto, String> {
        let user_id = self.api_key_verifier.extract_user_id(&dto.api_key)
            .map_err(
                |e| {
                tracing::error!("Error extract user_id {:?}", e);
                let err = Box::new(e);
                self.error_service.critical_error(&err)
                }
            )?;
        
        let str_user_id = user_id.to_string();
        let Ok(option_user) =  self.query_user_service.get_user_by_id(&str_user_id).await else {
            tracing::error!("Error search user");
            return self.internal_error();
        };
        let Some(user) = option_user else {
            tracing::error!("User not found");
            return self.internal_error();
        };
        let Some(api_key_hash) = user.aip_key_hash() else {
            tracing::error!("User doesn't have api_key_hash");
            return self.internal_error();
        };
        match self.api_key_verifier.is_verified(&api_key_hash, &dto.api_key) {
            Ok(true) => {},
            Ok(false) => return Ok(LoginApiKeyResponseDto::Error { err_msg: WRONG_CREDENTIALS.to_string() }),
            Err(e) => {
                tracing::error!("Verifier Err: {e}");
                return self.internal_error();
            },
        };

        let Ok(claims) = self.claims_provider.access_claims(&user)  else {
            tracing::error!("Err getting claims");
            return self.internal_error();
        };
    

        let Ok(access_token) = self.token_provider.generate_access(claims)  else {
            tracing::error!("Err create access_token");
            return self.internal_error();
        };
        Ok(LoginApiKeyResponseDto::Success { access_token: access_token })

    }

    fn internal_error(&self) -> Result<LoginApiKeyResponseDto, String> {
        Err(INTERNAL_ERROR_SERVER.to_string())
    }
}