use super::super::hasura::gql_descriptor::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::model::UserWithRole;

pub struct GetUserByIdRequestDescriptor{
    id: String
}
impl GetUserByIdRequestDescriptor {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl ObjectGQLDescriptor for GetUserByIdRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!({ "id": self.id })
    }
}

impl StaticGQLDescriptor for GetUserByIdRequestDescriptor {
    fn filename() -> &'static str {
        "get_user_by_id.graphql"
    }
    fn operation_name() -> &'static str {
        "GetUserById"
    }
    fn path() -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GetUserByIdResponse {
    pub users: Vec<UserWithRole>
}
