use uuid::Uuid;

use super::super::hasura::gql_descriptor::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::model::AllowedRoles;

pub struct AddRoleRequestDescriptor{
    role: String,
    user_id: Uuid,
    is_default: bool,
}
impl AddRoleRequestDescriptor {
    pub fn new(role: String, user_id: Uuid, is_default: bool,) -> Self {
        Self { role, user_id, is_default }
    }
}

impl ObjectGQLDescriptor for AddRoleRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!(
            {
                "role": self.role,
                "user_id": self.user_id,
                "is_default": self.is_default
            }
        )
    }
}

impl StaticGQLDescriptor for AddRoleRequestDescriptor {
    fn filename() -> &'static str {
        "create_allowed_roles.graphql"
    }
    fn operation_name() -> &'static str {
        "CreateAllowedRoles"
    }
    fn path() -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetRoleResponse {
    pub new: UserRoles
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserRoles {
    pub roles: Vec<AllowedRoles>,
}