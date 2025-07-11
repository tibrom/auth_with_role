use crate::application::error_dto::ComponentErrorDTO;
use crate::domain::errors::service::AppErrorInfo;
use crate::domain::settings::model::Credentials;
use crate::domain::user::models::base::{AuthMethod, UserAttribute, UserRole};
use crate::domain::user::service::CommandUserService;
use crate::domain::verifies::service::PasswordVerifierService;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::user::factories::UserProviderFactory;

use super::dto::{UserDTO, SignUpUserDto};
use super::error::UserAttributeError;

const AUTH_TYPE: &str = "email";
const NAME_ATTRIBUTE: &str = "username";
const EMAIL_ATTRIBUTE: &str = "email";



pub struct CreateUserWithEmailPasswdAction<U, V> {
    credentials: Credentials,
    command_user_service: U,
    password_verifier: V,
}


impl<U, V> CreateUserWithEmailPasswdAction<U, V>
where
    V: PasswordVerifierService,
    U: CommandUserService,
{
    

    pub fn new<VP, UP>(
        credentials: Credentials,
        verifies_provider_factory: &VP,
        user_provider_factory: &UP,
    ) -> Self
    where
        VP: VerifiesProviderFactory<PasswordVerifier = V>,
        UP: UserProviderFactory<CommandUser = U>
    {

        let password_verifier = verifies_provider_factory.password_verifier();
        let command_user_service = user_provider_factory.command_user();
        Self {
            credentials,
            command_user_service,
            password_verifier,
        }
    }

    pub async fn sign_up(&self, user: SignUpUserDto) -> Result<UserDTO, UserAttributeError> {
        
        let password_hash = self.password_verifier.create_hash(&user.password)
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        let is_free_email =  self.command_user_service.auth_identifier_is_free(user.email.clone()).await
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        if !is_free_email {
            return Err(UserAttributeError::EmailIsBusy);
        };

        let new_user = self.command_user_service.add_user().await
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        let user_id = new_user.id();
        let auth_method = AuthMethod::new(
            user_id.clone(),
            AUTH_TYPE.clone().to_string(),
            user.email.clone(),
            Some(password_hash.to_string())
        );

        self.command_user_service.add_auth_method(auth_method).await
            .map_err(|e|Self::map_infrastructure_error(&e))?;

        let user_attribute =vec![
            UserAttribute::new(user_id.clone(), NAME_ATTRIBUTE.to_string(), user.username.clone()),
            UserAttribute::new(user_id.clone(), EMAIL_ATTRIBUTE.to_string(), user.email.clone()),
        ];

        self.command_user_service.add_user_attribute(user_attribute).await
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        let user_role = UserRole::new(
            true,
            self.credentials.new_user_role().with_email().clone(),
            user_id.clone()
        );

        self.command_user_service.add_role(user_role).await
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        Ok(UserDTO {email: user.email.clone(), username: user.username.clone()})
    }

    
    fn infrastructure_error(&self, e: &dyn AppErrorInfo) -> Result<UserDTO, UserAttributeError> {
        Err(UserAttributeError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message())))
    }

    fn map_infrastructure_error(e: &dyn AppErrorInfo) -> UserAttributeError {
        UserAttributeError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message()))
    }
}
