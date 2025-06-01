use chrono::{DateTime, FixedOffset};
use uuid::Uuid;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ModuleUser {
    id: Uuid,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    username: String,
    last_name: Option<String>,
    first_name: Option<String>,
    email: Option<String>,
    telegram_id: Option<String>,
    password_hash: Option<String>,
    aip_key_hash: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ModuleUserWithRole {
    id: Uuid,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    username: String,
    last_name: Option<String>,
    first_name: Option<String>,
    email: Option<String>,
    telegram_id: Option<String>,
    password_hash: Option<String>,
    aip_key_hash: Option<String>,
    allowed_roles: Vec<AllowedRoles>
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AllowedRoles{
    id: Uuid,
    role: String,
    is_default: bool,
    created_at: DateTime<FixedOffset>,
    user_id: Uuid
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Role{
    value: String,
}