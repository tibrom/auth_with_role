use crate::domain::settings::model::Credentials;
use crate::domain::user::model::{AllowedRoles, UserNameEmailPasswordHash, UserWithRole};
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::infrastructure::hasura::client_manager::HasuraClientManager;
use crate::infrastructure::user::requests::create_user::{CrateUserRequestDescriptor, CrateUserResponse};
use crate::infrastructure::user::requests::add_roles::{AddRoleRequestDescriptor, SetRoleResponse};
use crate::infrastructure::user::requests::add_api_hash::{AddApiHAshRequestDescriptor, AddApiHashResponse};
use crate::infrastructure::user::requests::get_user_by_email::{GetUserByEmailRequestDescriptor, GetUserByEmailResponse};
use crate::infrastructure::user::requests::get_user_by_id::{GetUserByIdRequestDescriptor, GetUserByIdResponse};
use crate::infrastructure::user::requests::get_user_by_tg_id::{GetUserByTgIdRequestDescriptor, GetUserByTgIdResponse};

use super::errors::UserManagerError;


pub struct UserCommand{
    credentials: Credentials,
}

impl UserCommand {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl CommandUserService for UserCommand {
    type Error = UserManagerError;

    async fn create_user(
        &self,
        new_user: UserNameEmailPasswordHash,
    ) -> Result<UserWithRole, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;
        
        let descriptor = CrateUserRequestDescriptor::new(new_user);

        let result = client
            .execute::<CrateUserRequestDescriptor, CrateUserResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;
        

        match result.insert_users_user.user.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedCreateUser),
        }
    }


    async fn set_default_role(
        &self,
        mut user: UserWithRole,
    ) -> Result<UserWithRole, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let default_role = self.credentials.new_user_role().with_email().clone();

        let descriptor = AddRoleRequestDescriptor::new(default_role, user.id().clone(), true);

        let result = client
            .execute::<AddRoleRequestDescriptor, SetRoleResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let Some(new_role) = result.new.roles.first() else {
            return Err(UserManagerError::FailedCreateAllowedRoles);
        };

        user.add_role(new_role);

        Ok(user)
    }

    async fn add_role(
        &self,
        mut user: UserWithRole,
        allowed_role: AllowedRoles,
    ) -> Result<UserWithRole, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = AddRoleRequestDescriptor::new(allowed_role.role().to_string(), user.id().clone(), true);
        
        let result = client
            .execute::<AddRoleRequestDescriptor, SetRoleResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;


        let Some(new_role) = result.new.roles.first() else {
            return Err(UserManagerError::FailedCreateAllowedRoles);
        };

        user.add_role(new_role);

        Ok(user)
    }

    async fn add_api_hash(&self, user: UserWithRole, api_hash: &str) -> Result<UserWithRole, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = AddApiHAshRequestDescriptor::new(user.id().clone(), api_hash.clone().to_owned());

        let result = client
            .execute::<AddApiHAshRequestDescriptor, AddApiHashResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        match result.update_users_user.user.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedUpdateApiKey),
        }
    }
}

pub struct UserQuery;

impl QueryUserService for UserQuery {
    type Error = UserManagerError;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserWithRole>, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = GetUserByEmailRequestDescriptor::new(email.to_owned());

        let result = client
            .execute::<GetUserByEmailRequestDescriptor, GetUserByEmailResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.users.first().and_then(|v| Some(v.clone())))
    }

    async fn get_user_by_id(&self, id: &str) -> Result<Option<UserWithRole>, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = GetUserByIdRequestDescriptor::new(id.to_owned());

        let result = client
            .execute::<GetUserByIdRequestDescriptor, GetUserByIdResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.users.first().and_then(|v| Some(v.clone())))
    }

    async fn get_user_by_tg_id(&self, tg_id: &str) -> Result<Option<UserWithRole>, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = GetUserByTgIdRequestDescriptor::new(tg_id.to_owned());

        let result = client
            .execute::<GetUserByTgIdRequestDescriptor, GetUserByTgIdResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.users.first().and_then(|v| Some(v.clone())))
    }
}
