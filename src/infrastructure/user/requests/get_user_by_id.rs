use uuid::Uuid;

use super::super::network::hasura::interface::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::models::extended::ExtendedAuthMethod;

pub struct GetUserByUserIdRequestDescriptor {
    user_id: Uuid,
}
impl GetUserByUserIdRequestDescriptor {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

impl ObjectGQLDescriptor for GetUserByUserIdRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!({ "user_id": self.user_id})
    }
}

impl StaticGQLDescriptor for GetUserByUserIdRequestDescriptor {
    fn filename(&self) -> &'static str {
        "query_auth_method_by_user_id.graphql"
    }
    fn operation_name(&self) -> &'static str {
        "GetAuthMethodByUserId"
    }
    fn path(&self) -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GetUserByByUserIdResponse {
    pub users_auth_method: Vec<ExtendedAuthMethod>,
}
