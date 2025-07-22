use super::super::network::hasura::interface::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::models::extended::ExtendedAuthMethod;

pub struct GetUserByIdentifierRequestDescriptor {
    identifier: String,
}
impl GetUserByIdentifierRequestDescriptor {
    pub fn new(email: String) -> Self {
        Self { identifier: email }
    }
}

impl ObjectGQLDescriptor for GetUserByIdentifierRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!({ "identifier": self.identifier})
    }
}

impl StaticGQLDescriptor for GetUserByIdentifierRequestDescriptor {
    fn filename(&self) -> &'static str {
        "query_auth_methods_by_identifier.graphql"
    }
    fn operation_name(&self) -> &'static str {
        "GetAuthMethodByIdentifier"
    }
    fn path(&self) -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GetUserByByIdentifierResponse {
    pub users_auth_method: Vec<ExtendedAuthMethod>,
}
