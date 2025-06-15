#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HasuraClaims {
    #[serde(rename = "x-hasura-default-role")]
    x_hasura_default_role: String,

    #[serde(rename = "x-hasura-allowed-roles")]
    x_hasura_allowed_roles: Vec<String>,

    #[serde(rename = "x-hasura-user-id")]
    x_hasura_user_id: String,
}
impl HasuraClaims {
    pub fn new(
        x_hasura_default_role: String,
        x_hasura_allowed_roles: Vec<String>,
        x_hasura_user_id: String,
    ) -> Self {
        Self {
            x_hasura_default_role,
            x_hasura_allowed_roles,
            x_hasura_user_id,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    sub: String,
    admin: bool,
    exp: usize,
    #[serde(rename = "https://hasura.io/jwt/claims")]
    hasura_claims: HasuraClaims,
}

impl Claims {
    pub fn new(sub: String, admin: bool, exp: usize, hasura_claims: HasuraClaims) -> Self {
        Self {
            sub,
            admin,
            exp,
            hasura_claims,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RefreshClaims {
    pub sub: String,
    exp: i16,
}

impl RefreshClaims {
    pub fn new(sub: String, exp: i16) -> Self {
        Self { sub, exp }
    }
}
