use super::dto::{SignUpRequestDto, SignUpResponseDto};
use crate::application::auth::dto::TokenPairDto;
use crate::application::sign_up::dto::UserDataPairDto;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::user::model::{AllowedRoles, UserNameEmailPasswordHash};
use crate::domain::verifies::service::PasswordVerifierService;
use crate::domain::settings::service::CredentialsService;
use crate::domain::user::service::{CommandUserService, QueryUserService};
use super::ServiceErrorExt;


pub struct SignUpWithEmailUseCase<U, V, C>{
    command_user_service: U,
    verifier: V,
    credentials_provider: C

}

impl <U, V, C>ServiceErrorExt for SignUpWithEmailUseCase<U, V, C> {}

impl<U, V, C> SignUpWithEmailUseCase<U, V, C>
where
    V: PasswordVerifierService,
    U: CommandUserService,
    C: CredentialsService {

    pub fn new(command_user_service: U, verifier: V, credentials_provider: C) -> Self {
        Self {
            command_user_service,
            verifier,
            credentials_provider
        }
    }

    pub async fn sign_up(&self, user: SignUpRequestDto) -> Result<SignUpResponseDto, String> {
        let password_hash = match self.verifier.create_hash(&user.password) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        let new_user_data= UserNameEmailPasswordHash::new(&user.username, &user.email, &password_hash);
        let new_user = match self.command_user_service.create_user(new_user_data).await {
            Ok(u) => u,
            Err(e) => return self.handler_error(e)
        };
        let default_role = match self.credentials_provider.get_credentials(){
            Ok(v) => v.new_user_role().with_email().clone(),
            Err(e) => return self.handler_error(e)
        };

        let allowed_roles = AllowedRoles::new_default(&default_role, new_user.id());

        match self.command_user_service.add_role(new_user, allowed_roles).await {
            Ok(u) => {
                let email = u.email().clone().unwrap_or_else(||"".to_string());
                let user = UserDataPairDto {
                    username: u.username().clone(),
                    email
                };
                Ok(SignUpResponseDto::Success { user })
            },
            Err(e) => self.handler_error(e)
        }
    }
    

    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<SignUpResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => {
                Ok(SignUpResponseDto::Error { err_msg: self.map_service_error(e) })
            }
            _ => Err(self.map_service_error(e))
        }
    }
}
