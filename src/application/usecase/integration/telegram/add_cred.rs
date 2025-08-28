use crate::application::usecase::integration::telegram::dto::TelegramDataDTO; 
use crate::domain::{
    jwt::factories::JWTProviderFactory,
    settings::model::Credentials,
    user::{
        factories::UserProviderFactory,
        models::{
            base::{AuthMethod, UserAttribute, UserRole, User},
            extended::{
                ExtendedUser,
                ExtendedAuthMethod
            }
        },
    service::CommandUserService}};

use super::errors::{AddCredError, TelegramIntError};
use super::constants::{AUTH_TYPE, TELEGRAM_USERNAME, TELEGRAM_LAST_NAME, TELEGRAM_FIRST_NAME};

pub struct AddTelegramCredUseCase<CUS> {
    credentials: Credentials,
    command_user_service: CUS,
}
impl <CUS>AddTelegramCredUseCase <CUS>
where
    CUS: CommandUserService
{
    pub fn new<U>(credentials: Credentials, user_provider_factory: &U,) -> Self 
    where
        U: UserProviderFactory<CommandUser = CUS>,
    {
        let command_user_service = user_provider_factory.command_user();
        Self {credentials, command_user_service}
    }

    pub async fn execute(&self, user: User, dto: TelegramDataDTO) -> Result<ExtendedAuthMethod, TelegramIntError> {
        let user_id = user.id();

        let auth_method_by_id = AuthMethod::new(
            user_id.clone(),
            AUTH_TYPE.to_string(),
            dto.id.to_string(),
            None
        );

        let auth_method_by_username = AuthMethod::new(
            user_id.clone(),
            AUTH_TYPE.to_string(),
            dto.username.clone(),
            None
        );

        let Ok(auth_method) = self.command_user_service.add_auth_method(auth_method_by_id.clone()).await else {
            return Err(TelegramIntError::from(AddCredError::FailedAddingAuthMethod(auth_method_by_id.identifier().clone())));
        };

        if let Err(e) = self.command_user_service.add_auth_method(auth_method_by_username).await {
            return Err(TelegramIntError::from(AddCredError::FailedAddingAuthMethod(e.to_string())));
        }

        let mut user_attributes = vec![
            UserAttribute::new(
                user_id.clone(),
                TELEGRAM_USERNAME.to_string(),
                dto.username.clone(),
            )
        ];

        if let Some(first_name) = dto.first_name {
            user_attributes.push(
                UserAttribute::new(
                    user_id.clone(),
                    TELEGRAM_FIRST_NAME.to_string(),
                    first_name
                )
            );
        }

        if let Some(last_name) = dto.last_name {
            user_attributes.push(
                UserAttribute::new(
                    user_id.clone(),
                    TELEGRAM_LAST_NAME.to_string(),
                    last_name
                )
            );
        };

        if let Err(e) = self.command_user_service.add_user_attribute(user_attributes).await {
            return Err(TelegramIntError::from(AddCredError::FailedAddingUserAttribute(e.to_string())));
        };

        let user_role = UserRole::new(
            true,
            self.credentials.new_user_role().with_email().clone(),
            user_id.clone(),
        );

        if let Err(e) = self.command_user_service.add_role(user_role.clone()).await{
            return Err(TelegramIntError::from(AddCredError::FailedAddingUserRole(e.to_string())));
        }
        
        let mut extended = ExtendedUser::new(user.id().clone(), user.created_at().clone(), user.updated_at().clone());

        extended.add_role(user_role);

        Ok(ExtendedAuthMethod::new(auth_method, extended))
    }
}