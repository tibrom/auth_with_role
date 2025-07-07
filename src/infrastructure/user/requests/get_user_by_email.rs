use super::super::hasura::gql_descriptor::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::model::UserWithRole;

pub struct GetUserByEmailRequestDescriptor{
    email: String
}
impl GetUserByEmailRequestDescriptor {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

impl ObjectGQLDescriptor for GetUserByEmailRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!({ "email": self.email })
    }
}

impl StaticGQLDescriptor for GetUserByEmailRequestDescriptor {
    fn filename() -> &'static str {
        "get_user_by_email.graphql"
    }
    fn operation_name() -> &'static str {
        "GetUserByEmail"
    }
    fn path() -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GetUserByEmailResponse {
    pub users: Vec<UserWithRole>
}
