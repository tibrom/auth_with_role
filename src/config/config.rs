use std::collections::HashMap;

use config::Config;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Settings{
    pub host: String,
    pub port: String,
    pub role: String,
    pub jwt_secret: String,
    pub jwt_refresh_secret: String,
    pub hasura_url: String
}


