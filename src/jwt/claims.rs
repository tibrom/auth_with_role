use std::default;

use serde::{Serialize, Deserialize};

use crate::conf::credentials::CredentialsManager;
use crate::domain::module::user::UserWithRole;

use super::error::JwtError;

#[derive(Debug, Serialize, Deserialize)]
struct HasuraClaims {
    #[serde(rename = "x-hasura-default-role")]
    x_hasura_default_role: String,

    #[serde(rename = "x-hasura-allowed-roles")]
    x_hasura_allowed_roles: Vec<String>,

    #[serde(rename = "x-hasura-user-id")]
    x_hasura_user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    admin: bool,
    exp: usize,
    #[serde(rename = "https://hasura.io/jwt/claims")]
    hasura_claims: HasuraClaims,
}


impl TryFrom<UserWithRole> for Claims {
    type Error = JwtError;

    fn try_from(value: UserWithRole) -> Result<Self, Self::Error> {
        let x_hasura_default_role = value.allowed_roles()
            .iter()
            .find(|v| *v.is_default())
            .map(|v| Ok(v.role().clone()))
            .unwrap_or_else(|| Err(JwtError::DefaultRoleNotFound))?;

        let x_hasura_allowed_roles = value.allowed_roles()
            .iter()
            .map(|v|v.role().clone())
            .collect::<Vec<_>>();

        let x_hasura_user_id = value.id().to_string();

        let hasura_claims = HasuraClaims{
            x_hasura_default_role,
            x_hasura_allowed_roles,
            x_hasura_user_id: x_hasura_user_id.clone()
        };

        let exp = CredentialsManager::get_credentials()
            .map(|v| v.expiration_access_hours().clone())
            .map_err(|e| JwtError::CredentialsNotFound(e))?;

        Ok(Self { 
            sub: x_hasura_user_id,
            admin: false,
            exp,
            hasura_claims
        })
    }
}
