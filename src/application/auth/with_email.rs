use super::dto::{LoginEmailPasRequestDto, LoginEmailPasResponseDto};
use crate::application::auth::dto::TokenPairDto;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::verifies::service::PasswordVerifierService;
use crate::domain::jwt::service::{TokenService, JwtClaimsService};
use crate::domain::user::service::QueryUserService;
use super::ServiceErrorExt;

const WRONG_CREDENTIALS: &str = "Incorrect login or password";
const INTERNAL_ERROR_SERVER: &str = "Internal error";


pub struct LoginWithEmailUseCase<U, V, C, T>{
    user_provider: U,
    verifier: V,
    claims_provider: C,
    token_provider: T
}

impl <U, V, C, T>ServiceErrorExt for  LoginWithEmailUseCase <U, V, C, T>{}

impl<U, V, C, T> LoginWithEmailUseCase<U, V, C, T>
where
    V: PasswordVerifierService,
    T: TokenService,
    C: JwtClaimsService,
    U: QueryUserService, {

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
            Ok(None) => return self.map_none(WRONG_CREDENTIALS),
            Err(e) => return self.handler_error(e)
        };

        
        let Some(password_hash) =user.password_hash() else {
            return self.map_none(WRONG_CREDENTIALS);
        };

        match self.verifier.is_verified(&password_hash, &dto.password) {
            Ok(true) => {},
            Ok(false) => return self.map_none(WRONG_CREDENTIALS),
            Err(e) => return self.handler_error(e)
        };

        let claims = match self.claims_provider.access_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };
    
        let refresh_claims = match self.claims_provider.refresh_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        let access_token = match self.token_provider.generate_access(claims)  {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        let refresh_token = match self.token_provider.generate_refresh(refresh_claims) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        let token_pair_dto = TokenPairDto {
            access_token,
            refresh_token,
        };
        Ok(LoginEmailPasResponseDto::Success { auth_data: token_pair_dto })
    }


    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<LoginEmailPasResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => {
                Ok(LoginEmailPasResponseDto::Error { err_msg: self.map_service_error(e) })
            }
            _ => Err(self.map_service_error(e))
        }
    }

    fn map_none(&self, msg: &str) -> Result<LoginEmailPasResponseDto, String> {
        Ok(LoginEmailPasResponseDto::Error { err_msg: msg.to_string() })
    }
}
