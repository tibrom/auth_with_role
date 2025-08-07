use super::super::network::hasura::interface::{ObjectGQLDescriptor, StaticGQLDescriptor};
use super::gql_dir::GQL_DIR;

use crate::domain::user::models::base::AuthMethod;

pub struct AddAuthMethodDescriptor {
    auth_method: AuthMethod,
}
impl AddAuthMethodDescriptor {
    pub fn new(auth_method: AuthMethod) -> Self {
        Self { auth_method }
    }
}
//$auth_type: users_auth_type_enum = email, $secret: String = "", $user_id: uuid = "", $identifier: String = "") {

impl ObjectGQLDescriptor for AddAuthMethodDescriptor {
    fn variables(&self) -> serde_json::Value {
        let r = serde_json::json!(
            {
                "auth_type": self.auth_method.auth_type(),
                "secret": self.auth_method.secret(),
                "user_id": self.auth_method.user_id(),
                "identifier": self.auth_method.identifier()

            }
        );
        println!("r {}", r);
        r
    }
}

impl StaticGQLDescriptor for AddAuthMethodDescriptor {
    fn filename(&self) -> &'static str {
        "insert_user_auth_method.graphql"
    }
    fn operation_name(&self) -> &'static str {
        "InsertUsersAuthMethod"
    }
    fn path(&self) -> include_dir::Dir<'static> {
        GQL_DIR.clone()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AddAuthMethodResponse {
    pub insert_users_auth_method: Returning,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Returning {
    pub returning: Vec<AuthMethod>,
}
