use super::super::hasura::gql_descriptor::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::model::{UserNameEmailPasswordHash, UserWithRole};

pub struct CrateUserRequestDescriptor{
    user: UserNameEmailPasswordHash,
}
impl CrateUserRequestDescriptor {
    pub fn new(new_user: UserNameEmailPasswordHash) -> Self {
        Self { user: new_user }
    }
}

impl ObjectGQLDescriptor for CrateUserRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!(
            {
                "password_hash": self.user.password_hash(),
                "username": self.user.username(),
                "email": self.user.email()
            }
        )
    }
}

impl StaticGQLDescriptor for CrateUserRequestDescriptor {
    fn filename() -> &'static str {
        "create_user.graphql"
    }
    fn operation_name() -> &'static str {
        "CreateUser"
    }
    fn path() -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CrateUserResponse {
    pub insert_users_user: Users
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Users {
    pub user: Vec<UserWithRole>
}
