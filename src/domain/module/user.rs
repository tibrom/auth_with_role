use chrono::{DateTime, FixedOffset};
use uuid::Uuid;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ModuleUser {
    id: Uuid,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    name: String,
    email: String,
    tg_id: String,
    password_hash: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserWithRole {
    id: Uuid,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    name: String,
    email: String,
    tg_id: String,
    password_hash: String,
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