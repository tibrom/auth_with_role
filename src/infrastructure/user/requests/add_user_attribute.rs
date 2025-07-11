use uuid::Uuid;

use super::super::network::hasura::interface::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::models::base::UserAttribute;

pub struct AddAttributesRequestDescriptor{
    attributes: Vec<UserAttribute>
}
impl AddAttributesRequestDescriptor {
    pub fn new(attributes: Vec<UserAttribute>) -> Self {
        Self { attributes }
    }
}

impl ObjectGQLDescriptor for AddAttributesRequestDescriptor {
    fn variables(&self) -> serde_json::Value {
        let objects = self.attributes
            .iter()
        .map(|v| 
            serde_json::json!({
                "user_id": v.user_id(),
                "attribute": v.attribute(),
                "value": v.value()
            })
        )
        .collect::<Vec<_>>();
        serde_json::json!(
            {"objects": objects}
        )
    }
}

impl StaticGQLDescriptor for AddAttributesRequestDescriptor {
    fn filename(&self) -> &'static str {
        "insert_user_attributes.graphql"
    }
    fn operation_name(&self) -> &'static str {
        "InsertMultipleAttributes"
    }
    fn path(&self) -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AddAttributesResponse {
    pub insert_users_user_attribute: Returning
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Returning {
    pub returning: Vec<UserAttribute>,
}