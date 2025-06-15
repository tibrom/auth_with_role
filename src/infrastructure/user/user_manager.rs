use std::clone;

use crate::domain::user::model::{AllowedRoles, UserNameEmailPasswordHash, UserWithRole};
use crate::domain::user::service::{CommandUserService, QueryUserService};

use super::errors::UserManagerError;
use super::hasura::client::HasuraClient;
use super::hasura::client_manager::{
    HasuraClientManager, CREATE_ALLOWED_ROLES, CREATE_USER, GET_USER_BY_EMAIL, GET_USER_BY_ID,
    GET_USER_BY_TG_ID, UPDATE_API_KEY_USER,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct HasuraAnswerUser {
    pub user: Vec<UserWithRole>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct HasuraCreatedUser {
    pub new: HasuraAnswerUser,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct HasuraCreatedRoles {
    pub new: HasuraAllowedRoles,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct HasuraAllowedRoles {
    pub roles: Vec<AllowedRoles>,
}

pub struct UserCommand;

impl CommandUserService for UserCommand {
    type Error = UserManagerError;

    async fn create_user(
        &self,
        new_user: UserNameEmailPasswordHash,
    ) -> Result<UserWithRole, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;
        let variables = serde_json::json!(
            {
                "password_hash": new_user.password_hash(),
                "username": new_user.username(),
                "email": new_user.email()
            }
        );
        let value = client
            .execute(CREATE_USER, variables)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let parsed_result: HasuraCreatedUser = serde_json::from_value(value)
            .map_err(|e| UserManagerError::ResponseJsonParseError(e))?;

        match parsed_result.new.user.first() {
            Some(user) => Ok(user.clone()),
            None => Err(UserManagerError::FailedCreateUser),
        }
    }

    async fn add_role(
        &self,
        mut user: UserWithRole,
        allowed_roles: AllowedRoles,
    ) -> Result<UserWithRole, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;
        let variables = serde_json::json!(
            {
                "role": allowed_roles.role(),
                "user_id": allowed_roles.user_id(),
                "is_default": allowed_roles.is_default()
            }
        );
        let value = client
            .execute(CREATE_ALLOWED_ROLES, variables)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let parsed_result: HasuraCreatedRoles = serde_json::from_value(value)
            .map_err(|e| UserManagerError::ResponseJsonParseError(e))?;

        let Some(new_role) = parsed_result.new.roles.first() else {
            return Err(UserManagerError::FailedCreateAllowedRoles);
        };

        user.add_role(new_role);

        Ok(user)
    }

    async fn add_api_hash(&self, id: &str, api_hash: &str) -> Result<UserWithRole, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;
        let variables = serde_json::json!(
            {
                "user_id": id,
                "aip_key_hash": api_hash
            }
        );

        let value = client
            .execute(UPDATE_API_KEY_USER, variables)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let parsed_result: HasuraCreatedUser = serde_json::from_value(value)
            .map_err(|e| UserManagerError::ResponseJsonParseError(e))?;

        match parsed_result.new.user.first() {
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
        let variables = serde_json::json!({ "email": email });
        let value = client
            .execute(GET_USER_BY_EMAIL, variables)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let parsed_result: HasuraAnswerUser = serde_json::from_value(value)
            .map_err(|e| UserManagerError::ResponseJsonParseError(e))?;
        Ok(parsed_result.user.first().and_then(|v| Some(v.clone())))
    }

    async fn get_user_by_id(&self, id: &str) -> Result<Option<UserWithRole>, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let variables = serde_json::json!({ "id": id });
        let value = client
            .execute(GET_USER_BY_ID, variables)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let parsed_result: HasuraAnswerUser = serde_json::from_value(value)
            .map_err(|e| UserManagerError::ResponseJsonParseError(e))?;

        Ok(parsed_result.user.first().and_then(|v| Some(v.clone())))
    }

    async fn get_user_by_tg_id(&self, tg_id: &str) -> Result<Option<UserWithRole>, Self::Error> {
        let client = HasuraClientManager::get_hasura_client()
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;
        let variables = serde_json::json!({ "tg_id": tg_id });
        let value = client
            .execute(GET_USER_BY_TG_ID, variables)
            .await
            .map_err(|e| UserManagerError::HasuraClientError(e))?;

        let parsed_result: HasuraAnswerUser = serde_json::from_value(value)
            .map_err(|e| UserManagerError::ResponseJsonParseError(e))?;

        Ok(parsed_result.user.first().and_then(|v| Some(v.clone())))
    }
}
