use super::super::network::hasura::interface::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

pub struct CheckAuthMethodRequestDescriptor {
    identifier: String,
}
impl CheckAuthMethodRequestDescriptor {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}

impl ObjectGQLDescriptor for CheckAuthMethodRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        serde_json::json!(
            {"identifier": self.identifier}
        )
    }
}

impl StaticGQLDescriptor for CheckAuthMethodRequestDescriptor {
    fn filename(&self) -> &'static str {
        "users_auth_method_aggregate.graphql"
    }
    fn operation_name(&self) -> &'static str {
        "CheckAuthMethodExists"
    }
    fn path(&self) -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CheckAuthMethodResponse {
    pub users_auth_method_aggregate: Aggregate,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Aggregate {
    pub aggregate: Count,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Count {
    pub count: u64,
}
