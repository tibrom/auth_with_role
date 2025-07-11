use crate::domain::settings::model::Credentials;
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::infrastructure::network::client_manager::HasuraClientManager;
use crate::infrastructure::network::hasura::interface::HasuraInterface;

use super::errors::UserManagerError;
use super::requests::add_auth_method::{AddAuthMethodDescriptor, AddAuthMethodResponse};
use super::requests::add_roles::{AddRoleRequestDescriptor, AddRoleResponse};
use super::requests::add_user::{AddUserRequestDescriptor, AddUserResponse};
use super::requests::add_user_attribute::{AddAttributesRequestDescriptor, AddAttributesResponse};
use super::requests::check_auth_method::{CheckAuthMethodRequestDescriptor, CheckAuthMethodResponse};

use crate::domain::user::models::base::{
    AuthMethod,
    UserRole,
    User,
    UserAttribute
};

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

    async fn auth_identifier_is_free(&self, identifier: String) -> Result<bool, Self::Error> {
        let mut client = HasuraClientManager::get_hasura_client(&self.credentials)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = CheckAuthMethodRequestDescriptor::new(identifier);

        let result = client
            .execute::<CheckAuthMethodRequestDescriptor, CheckAuthMethodResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.users_auth_method_aggregate.aggregate.count == 0)

    }

    async fn add_auth_method(&self, auth_method: AuthMethod) -> Result<AuthMethod, Self::Error> {
        let mut client = HasuraClientManager::get_hasura_client(&self.credentials)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = AddAuthMethodDescriptor::new(auth_method);

        let result = client
            .execute::<AddAuthMethodDescriptor, AddAuthMethodResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        match result.insert_users_auth_method.returning.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedCreateUser),
        }
    }
    async fn add_role(&self, user_role: UserRole) -> Result<UserRole, Self::Error> {
        let mut client = HasuraClientManager::get_hasura_client(&self.credentials)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = AddRoleRequestDescriptor::new(user_role);

        let result = client
            .execute::<AddRoleRequestDescriptor, AddRoleResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        match result.insert_users_user_role.returning.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedCreateUser),
        }
    }
    async fn add_user(&self) -> Result<User, Self::Error> {
        let mut client = HasuraClientManager::get_hasura_client(&self.credentials)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = AddUserRequestDescriptor;

        let result = client
            .execute::<AddUserRequestDescriptor, AddUserResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        match result.insert_users_user.returning.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedCreateUser),
        }
    }
    async fn add_user_attribute(&self, attributes: Vec<UserAttribute>) -> Result<Vec<UserAttribute>, Self::Error> {
        let mut client = HasuraClientManager::get_hasura_client(&self.credentials)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = AddAttributesRequestDescriptor::new(attributes);

        let result = client
            .execute::<AddAttributesRequestDescriptor, AddAttributesResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.insert_users_user_attribute.returning)
    }
}


use crate::domain::user::models::extended::ExtendedAuthMethod;

use super::requests::get_user_by_identifier::{GetUserByIdentifierRequestDescriptor, GetUserByByIdentifierResponse};
use super::requests::get_user_by_id::{GetUserByUserIdRequestDescriptor, GetUserByByUserIdResponse};


pub struct UserQuery{
    credentials: Credentials,
}

impl UserQuery {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl QueryUserService for UserQuery {
    type Error = UserManagerError;

    
    async fn get_user_by_identifier(&self, identifier: &str) -> Result<Option<ExtendedAuthMethod>, Self::Error> {
        let mut client = HasuraClientManager::get_hasura_client(&self.credentials)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = GetUserByIdentifierRequestDescriptor::new(identifier.to_owned());

        let result = client
            .execute::<GetUserByIdentifierRequestDescriptor, GetUserByByIdentifierResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.users_auth_method.first().and_then(|v| Some(v.clone())))
    }


    async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<Option<ExtendedAuthMethod>, Self::Error> {
        let mut client = HasuraClientManager::get_hasura_client(&self.credentials)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let descriptor = GetUserByUserIdRequestDescriptor::new(id.to_owned());

        let result = client
            .execute::<GetUserByUserIdRequestDescriptor, GetUserByByUserIdResponse>(&descriptor).await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        Ok(result.users_auth_method.first().and_then(|v| Some(v.clone())))
    }
}
