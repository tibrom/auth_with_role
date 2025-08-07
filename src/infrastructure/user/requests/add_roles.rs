use super::super::network::hasura::interface::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::models::base::UserRole;

pub struct AddRoleRequestDescriptor {
    user_role: UserRole,
}
impl AddRoleRequestDescriptor {
    pub fn new(user_role: UserRole) -> Self {
        Self { user_role }
    }
}

impl ObjectGQLDescriptor for AddRoleRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!(
            {
                "user_id": self.user_role.user_id(),
                "role": self.user_role.role(),
                "is_default": self.user_role.is_default()
            }
        )
    }
}

impl StaticGQLDescriptor for AddRoleRequestDescriptor {
    fn filename(&self) -> &'static str {
        "insert_user_role.graphql"
    }
    fn operation_name(&self) -> &'static str {
        "InsertUsersRole"
    }
    fn path(&self) -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AddRoleResponse {
    pub insert_users_user_role: Returning,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Returning {
    pub returning: Vec<UserRole>,
}
