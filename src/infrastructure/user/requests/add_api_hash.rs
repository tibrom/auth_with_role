use uuid::Uuid;

use super::super::hasura::gql_descriptor::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::model::UserWithRole;

pub struct AddApiHAshRequestDescriptor{
    user_id: Uuid,
    api_key_hash: String

}
impl AddApiHAshRequestDescriptor {
    pub fn new( user_id: Uuid, api_key_hash: String) -> Self {
        Self { user_id, api_key_hash }
    }
}

impl ObjectGQLDescriptor for AddApiHAshRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!(
            {
                "user_id": self.user_id,
                "api_key_hash": self.api_key_hash
            }
        )
    }
}

impl StaticGQLDescriptor for AddApiHAshRequestDescriptor {
    fn filename() -> &'static str {
        "update_api_key_user.graphql"
    }
    fn operation_name() -> &'static str {
        "UpdateApiKeyUser"
    }
    fn path() -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AddApiHashResponse {
    pub update_users_user: Users
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Users {
    pub user: Vec<UserWithRole>
}