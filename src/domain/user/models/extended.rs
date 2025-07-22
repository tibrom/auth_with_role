use chrono::{DateTime, FixedOffset, Utc};
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
    user: ExtendedUser,
}

impl ExtendedAuthMethod {
    pub fn mock() -> Self {
        let mock_user = ExtendedUser::mock();
        Self {
            id: Uuid::new_v4(),
            auth_type: "email".to_string(),
            identifier: "test@test.test".to_string(),
            secret: Some("random".to_string()),
            created_at: Some(Utc::now().into()),
            user_id: mock_user.id.clone(),
            user: mock_user
        }
    }
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
    user_attributes: Vec<UserAttribute>,
}

impl ExtendedUser {
    fn mock() -> Self {
        let user_id = Uuid::new_v4();
        let user_roles = vec![
            UserRole::new(true,"user".to_string(),user_id.clone()),
            UserRole::new(true,"admin".to_string(),user_id.clone()),
            UserRole::new(false,"test".to_string(),user_id.clone())
        ];
        let user_attributes = vec![
            UserAttribute::new(user_id, "name".to_string(), "Mock".to_string()),
            UserAttribute::new(user_id, "surname".to_string(), "Mock".to_string()),
        ];
        Self {
            id: Uuid::new_v4(),
            created_at: Some(Utc::now().into()),
            updated_at: Some(Utc::now().into()),
            user_roles,
            user_attributes
        }
    }
}