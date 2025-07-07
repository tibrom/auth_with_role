use super::super::hasura::gql_descriptor::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::model::UserWithRole;

pub struct GetUserByTgIdRequestDescriptor{
    tg_id: String
}
impl GetUserByTgIdRequestDescriptor {
    pub fn new(tg_id: String) -> Self {
        Self { tg_id }
    }
}

impl ObjectGQLDescriptor for GetUserByTgIdRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!({ "tg_id": self.tg_id })
    }
}

impl StaticGQLDescriptor for GetUserByTgIdRequestDescriptor {
    fn filename() -> &'static str {
        "get_user_by_tg_id.graphql"
    }
    fn operation_name() -> &'static str {
        "UserByTgIdQuery"
    }
    fn path() -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GetUserByTgIdResponse {
    pub users: Vec<UserWithRole>
}
