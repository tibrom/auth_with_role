use tracing::Instrument;

use crate::application::error_dto::ComponentErrorDTO;
use crate::domain::errors::service::AppErrorInfo;
use crate::domain::user::model::UserNameEmailPasswordHash;
use crate::domain::user::service::CommandUserService;
use crate::domain::verifies::service::PasswordVerifierService;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::user::factories::UserProviderFactory;

use super::dto::{UserDTO, SignUpUserDto};
use super::error::UserAttributeError;



pub struct CreateUserWithEmailPasswdAction<U, V> {
    command_user_service: U,
    password_verifier: V,
}


impl<U, V> CreateUserWithEmailPasswdAction<U, V>
where
    V: PasswordVerifierService,
    U: CommandUserService,
{

    pub fn new<VP, UP>(
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
            command_user_service,
            password_verifier,
        }
    }

    pub async fn sign_up(&self, user: SignUpUserDto) -> Result<UserDTO, UserAttributeError> {
        let password_hash = match self.password_verifier.create_hash(&user.password) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e),
        };

        let new_user_data =
            UserNameEmailPasswordHash::new(&user.username, &user.email, &password_hash);
        let new_user = match self.command_user_service.create_user(new_user_data).await {
            Ok(u) => u,
            Err(e) => return self.infrastructure_error(&e),
        };
        let user =match self.command_user_service.set_default_role(new_user).await {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e)
        };

        let Some(email) = user.email() else {
            return Err(UserAttributeError::UserNotFound(user.id().to_string()));
        };

        Ok(UserDTO {email: email.clone(), username: user.username().clone()})
    }

    
    fn infrastructure_error(&self, e: &dyn AppErrorInfo) -> Result<UserDTO, UserAttributeError> {
        Err(UserAttributeError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message())))
    }
}
