use chrono::{DateTime, FixedOffset};
use getset::{Getters, Setters};
use uuid::Uuid;

use super::base::{UserAttribute, UserRole};


#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ExtendedAuthMethod {
    #[get = "pub"]
    id: Uuid,
    #[get = "pub"]
    auth_type: String,
    #[get = "pub"]
    identifier: String,
    #[get = "pub"]
    secret: Option<String>,
    #[get = "pub"]
    created_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    user_id: Uuid,
    #[get = "pub"]
    user: ExtendedUser
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ExtendedUser {
    id: Uuid,
    #[get = "pub"]
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    user_roles: Vec<UserRole>,
    #[get = "pub"]
    user_attributes: Vec<UserAttribute>
}