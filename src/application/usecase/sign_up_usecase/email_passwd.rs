use crate::application::error_ext::ServiceErrorExt;
use crate::application::usecase::sign_up_usecase::dto::{SignUpRequestDto, SignUpResponseDto, UserDataDto};
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::settings::model::Credentials;
use crate::domain::user::factories::UserProviderFactory;
use crate::domain::user::models::base::{AuthMethod, UserAttribute, UserRole};
use crate::domain::user::service::CommandUserService;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::verifies::service::PasswordVerifierService;


use super::error::UserAttributeError;

const AUTH_TYPE: &str = "email";
const NAME_ATTRIBUTE: &str = "username";
const EMAIL_ATTRIBUTE: &str = "email";

pub struct SignUpWithEmailUseCase<U, V> {
    credentials: Credentials,
    command_user_service: U,
    password_verifier: V,
}


impl<U, V> ServiceErrorExt for SignUpWithEmailUseCase<U, V> {}


impl<U, V> SignUpWithEmailUseCase<U, V>
where
    U: CommandUserService,
    V: PasswordVerifierService,
{
    pub fn new<VP, UP>(
        credentials: Credentials,
        verifies_provider_factory: &VP,
        user_provider_factory: &UP,
    ) -> Self
    where
        VP: VerifiesProviderFactory<PasswordVerifier = V>,
        UP: UserProviderFactory<CommandUser = U>,
    {
        let password_verifier = verifies_provider_factory.password_verifier();
        let command_user_service = user_provider_factory.command_user();
        Self {
            credentials,
            command_user_service,
            password_verifier,
        }
    }

    pub async fn execute(&self, user: SignUpRequestDto) -> Result<SignUpResponseDto, String> {
        let password_hash = match self.password_verifier.create_hash(&user.password) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        let is_free_email = match self.command_user_service.auth_identifier_is_free(user.email.clone(), AUTH_TYPE).await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        if !is_free_email {
            return self.handler_error(UserAttributeError::EmailIsBusy);
        };

        let new_user = match self.command_user_service.add_user().await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        let user_id = new_user.id();
        let auth_method = AuthMethod::new(
            user_id.clone(),
            AUTH_TYPE.to_string(),
            user.email.clone(),
            Some(password_hash.to_string()),
        );

        if let Err(e) = self.command_user_service.add_auth_method(auth_method).await {
            return self.handler_error(e);
        }

        let user_attribute = vec![
            UserAttribute::new(
                user_id.clone(),
                NAME_ATTRIBUTE.to_string(),
                user.username.clone(),
            ),
            UserAttribute::new(
                user_id.clone(),
                EMAIL_ATTRIBUTE.to_string(),
                user.email.clone(),
            ),
        ];

        if let Err(e) = self.command_user_service.add_user_attribute(user_attribute).await {
            return self.handler_error(e);
        }

        let user_role = UserRole::new(
            true,
            self.credentials.new_user_role().with_email().clone(),
            user_id.clone(),
        );

        if let Err(e) = self.command_user_service.add_role(user_role).await{
            return self.handler_error(e);
        }

        let user_dto = UserDataDto {
            email: user.email.clone(),
            username: user.username.clone(),
        };

        Ok(SignUpResponseDto::Success {
            user: user_dto
        })
        
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

