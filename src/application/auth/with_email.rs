use super::dto::{LoginEmailPasRequestDto, LoginEmailPasResponseDto};
use crate::application::auth::dto::TokenPairDto;
use crate::domain::verifies::service::PasswordVerifierService;
use crate::domain::jwt::service::{TokenService, JwtClaimsService};
use crate::domain::user::service::RemoteUserService;

const WRONG_CREDENTIALS: &str = "Incorrect login or password";
const INTERNAL_ERROR_SERVER: &str = "Internal error";


pub struct LoginWithEmailUseCase<U, V, C, T>{
    user_provider: U,
    verifier: V,
    claims_provider: C,
    token_provider: T
}

impl<U, V, C, T> LoginWithEmailUseCase<U, V, C, T>
where
    V: PasswordVerifierService,
    T: TokenService,
    C: JwtClaimsService,
    U: RemoteUserService, {

    pub fn new(user_provider: U, verifier: V, claims_provider: C, token_provider: T) -> Self {
        Self {
            user_provider,
            verifier,
            claims_provider,
            token_provider
        }
    }

    pub async fn login(&self, dto: LoginEmailPasRequestDto) -> Result<LoginEmailPasResponseDto, String> {
        let user = match self.user_provider.get_user_by_email(&dto.email).await {
            Ok(Some(user)) => user,
            Ok(None) => return Ok(LoginEmailPasResponseDto::Error { err_msg: WRONG_CREDENTIALS.to_string() }),
            Err(e) => {
                tracing::error!("Search User Err: {e}");
                return self.internal_error()
            },
        };

        
        let Some(password_hash) =user.password_hash() else {
            return Ok(LoginEmailPasResponseDto::Error { err_msg: WRONG_CREDENTIALS.to_string() });
        };

        match self.verifier.is_verified(&password_hash, &dto.password) {
            Ok(true) => {},
            Ok(false) => return Ok(LoginEmailPasResponseDto::Error { err_msg: WRONG_CREDENTIALS.to_string() }),
            Err(e) => {
                tracing::error!("Verifier Err: {e}");
                return self.internal_error();
            },
        };

        let Ok(claims) = self.claims_provider.access_claims(&user)  else {
            tracing::error!("Err getting claims");
            return self.internal_error();
        };
    
        let Ok(refresh_claims) = self.claims_provider.refresh_claims(&user) else {
            tracing::warn!("Err getting refresh_claims");
            return self.internal_error();
        };

        let Ok(access_token) = self.token_provider.generate_access(claims)  else {
            tracing::error!("Err create access_token");
            return self.internal_error();
        };

        let Ok(refresh_token) = self.token_provider.generate_refresh(refresh_claims) else {
            tracing::error!("Err create refresh_token");
            return self.internal_error();
        };

        let token_pair_dto = TokenPairDto {
            access_token,
            refresh_token,
        };
        Ok(LoginEmailPasResponseDto::Success { auth_data: token_pair_dto })
    }

    fn internal_error(&self) -> Result<LoginEmailPasResponseDto, String> {
        Err(INTERNAL_ERROR_SERVER.to_string())
    }
}
