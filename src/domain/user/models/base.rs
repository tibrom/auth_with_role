use chrono::{DateTime, FixedOffset};
use getset::{Getters, Setters};
use uuid::Uuid;

use super::extended::ExtendedUser;

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct User {
    #[get = "pub"]
    id: Uuid,
    #[get = "pub"]
    created_at: DateTime<FixedOffset>,
    #[get = "pub"]
    updated_at: Option<DateTime<FixedOffset>>,
}

impl User {
    pub fn new(id: Uuid, created_at: DateTime<FixedOffset>, updated_at: Option<DateTime<FixedOffset>> ) -> Self {
        Self { id, created_at, updated_at }
    }
}

impl From<ExtendedUser> for User {
    fn from(value: ExtendedUser) -> Self {
        Self { id: value.id().clone(), created_at: value.created_at().clone(), updated_at: value.updated_at().clone() }
    }
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AuthMethod {
    #[get = "pub"]
    id: Option<Uuid>,
    #[get = "pub"]
    created_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    user_id: Uuid,
    #[get = "pub"]
    auth_type: String,
    #[get = "pub"]
    identifier: String,
    #[get = "pub"]
    secret: Option<String>,
}

impl AuthMethod {
    pub fn new(
        user_id: Uuid,
        auth_type: String,
        identifier: String,
        secret: Option<String>,
    ) -> Self {
        Self {
            id: None,
            created_at: None,
            user_id,
            auth_type,
            identifier,
            secret,
        }
    }
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserRole {
    id: Option<Uuid>,
    #[get = "pub"]
    created_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    is_default: bool,
    #[get = "pub"]
    role: String,
    #[get = "pub"]
    user_id: Uuid,
}

impl UserRole {
    pub fn new(is_default: bool, role: String, user_id: Uuid) -> Self {
        Self {
            id: None,
            created_at: None,
            is_default,
            role,
            user_id,
        }
    }
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserAttribute {
    #[get = "pub"]
    id: Option<Uuid>,
    #[get = "pub"]
    created_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    updated_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    user_id: Uuid,
    #[get = "pub"]
    attribute: String,
    #[get = "pub"]
    value: String,
}

impl UserAttribute {
    pub fn new(user_id: Uuid, attribute: String, value: String) -> Self {
        Self {
            id: None,
            created_at: None,
            updated_at: None,
            user_id,
            attribute,
            value,
        }
    }
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Attribute {
    #[get = "pub"]
    is_required: bool,
    #[get = "pub"]
    is_unique: bool,
    #[get = "pub"]
    value: String,
}
