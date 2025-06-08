use chrono::{DateTime, FixedOffset};
use getset::{Getters, Setters};
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

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserWithRole {
    #[get = "pub"]
    id: Uuid,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    name: String,
    email: String,
    tg_id: String,
    #[get = "pub"]
    allowed_roles: Vec<AllowedRoles>,
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AllowedRoles {
    #[get = "pub"]
    id: Uuid,
    #[get = "pub"]
    role: String,
    #[get = "pub"]
    is_default: bool,
    created_at: DateTime<FixedOffset>,
    user_id: Uuid,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Role {
    value: String,
}
