use super::dto::{SignUpRequestDto, SignUpResponseDto};
use super::ServiceErrorExt;
use super::dto::UserDataDto;
use crate::application::actions::user_attribute::dto::SignUpUserDto;
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::settings::service::CredentialsService;
use crate::domain::user::factories::UserProviderFactory;
use crate::domain::user::model::{AllowedRoles, UserNameEmailPasswordHash};
use crate::domain::user::service::CommandUserService;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::verifies::service::PasswordVerifierService;

use crate::application::actions::user_attribute::user::CreateUserWithEmailPasswdAction;

pub struct SignUpWithEmailUseCase<V, U> {
    verifies_provider_factory: V,
    user_provider_factory: U,
}

impl<V, U> ServiceErrorExt for SignUpWithEmailUseCase<V, U> {}

impl<V, U> SignUpWithEmailUseCase<V, U>
where
    V: VerifiesProviderFactory,
    U: UserProviderFactory
{
    pub fn new(verifies_provider_factory: V, user_provider_factory: U) -> Self {
        Self {
            verifies_provider_factory,
            user_provider_factory,
        }
    }

    pub async fn execute(&self, user: SignUpRequestDto) -> Result<SignUpResponseDto, String> {
        let create_user_sub_case = CreateUserWithEmailPasswdAction::new(
            &self.verifies_provider_factory,
            &self.user_provider_factory
        );
        let user = match create_user_sub_case.sign_up(SignUpUserDto::from(user)).await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        
        Ok(SignUpResponseDto::Success { user: user.to_user_data_dto() })
        
    }

    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<SignUpResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(SignUpResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }
}
