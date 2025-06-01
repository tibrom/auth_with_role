use serde_json::{json, Value};
use super::user::ModuleUserWithRole;
use super::errors::DatabaseError;

use crate::http::hasura::{HasuraClient, HasuraClientInterface};
use crate::http::hasura::{GET_USER_BY_EMAIL, GET_USER_BY_ID, GET_USER_BY_TG_ID};


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct HasurasModuleUser {
    pub user: Vec<ModuleUserWithRole>,
}


#[derive(Clone, Debug)]
pub struct UserApi {
    gql_client: HasuraClient,
}

impl UserApi {
    pub fn new() -> Self {
        Self {
            gql_client: HasuraClientInterface::get_hasura_client()
        }
    }
}

impl UserApi {
    fn parse_module_user_with_role(&self, value: Value) -> Result<ModuleUserWithRole, DatabaseError> {
        let result: HasurasModuleUser = serde_json::from_value(value)
            .map_err(|e| DatabaseError::ParseJsonError(e.to_string()))?;

        let Some(user) = result.user.first() else {
            return Err(DatabaseError::UserNotFound);
        };
        Ok(user.clone())
    }
    pub async fn get_user_by_email(&self, email: &str) -> Result<ModuleUserWithRole, DatabaseError> {
        let variables = json!({ "email": email });
        let value = self.gql_client
            .execute(GET_USER_BY_EMAIL, variables)
            .await
            .map_err(|e| DatabaseError::HasuraError(e))?;
        self.parse_module_user_with_role(value)
    }

    pub async fn get_user_by_telegram_id(&self, telegram_id: i64) -> Result<ModuleUserWithRole, DatabaseError> {
        let variables = json!({ "telegram_id": telegram_id });
        let value = self.gql_client
            .execute(GET_USER_BY_TG_ID, variables)
            .await
            .map_err(|e| DatabaseError::HasuraError(e))?;
        self.parse_module_user_with_role(value)
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<ModuleUserWithRole, DatabaseError> {
        let variables = json!({ "id": id });
        let value = self.gql_client
            .execute(GET_USER_BY_ID, variables)
            .await
            .map_err(|e| DatabaseError::HasuraError(e))?;
        self.parse_module_user_with_role(value)
    }
    
}