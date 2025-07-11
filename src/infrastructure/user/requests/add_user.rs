use super::super::network::hasura::interface::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::models::base::{User};

pub struct AddUserRequestDescriptor;

impl ObjectGQLDescriptor for AddUserRequestDescriptor{}

impl StaticGQLDescriptor for AddUserRequestDescriptor {
    fn filename(&self) -> &'static str {
        "insert_user.graphql"
    }
    fn operation_name(&self) -> &'static str {
        "InsertUser"
    }
    fn path(&self) -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AddUserResponse {
    pub insert_users_user: Returning
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Returning {
    pub returning: Vec<User>
}
