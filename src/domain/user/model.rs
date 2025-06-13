use chrono::{DateTime, FixedOffset};
use getset::{Getters, Setters};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct User {
    id: Uuid,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    username: String,
    email: Option<String>,
    telegram_id: Option<String>,
    password_hash: Option<String>,
    api_key_hash: Option<String>,
}


#[derive(Getters, Setters, Debug, Clone, PartialEq)]
pub struct UserNameEmailPasswordHash {
    #[get = "pub"]
    username: String,
    #[get = "pub"]
    email: String,
    #[get = "pub"]
    password_hash: String,
}


impl UserNameEmailPasswordHash {
    pub fn new(username: &str, email: &str, password_hash: &str) -> Self {
        Self { username: username.to_string(), email: email.to_string(), password_hash: password_hash.to_string() }
    }
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserWithRole {
    #[get = "pub"]
    id: Uuid,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    username: String,
    #[get = "pub"]
    email: Option<String>,
    telegram_id: Option<String>,
    #[get = "pub"]
    password_hash: Option<String>,
    #[get = "pub"]
    allowed_roles: Vec<AllowedRole>,
}

#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AllowedRole {
    #[get = "pub"]
    id: Option<Uuid>,
    #[get = "pub"]
    role: String,
    #[get = "pub"]
    is_default: bool,
    created_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    user_id: Uuid,
}

impl AllowedRole {
    pub fn new_default(role: &str, user_id: &Uuid) -> Self {
        Self { 
            id: None,
            role: role.to_string(),
            is_default: true,
            created_at: None,
            user_id: user_id.clone() 
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Role {
    value: String,
}