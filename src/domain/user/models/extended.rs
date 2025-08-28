use chrono::{DateTime, FixedOffset, Utc};
use getset::{Getters, Setters};
use uuid::Uuid;

use crate::domain::user::models::base::AuthMethod;

use super::base::{UserAttribute, UserRole, User};


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
    pub fn new(
        auth_method: AuthMethod,
        extended_user: ExtendedUser
    ) -> Self {
        let id = match auth_method.id() {
            Some(v) => v.clone(),
            None => Uuid::new_v4() 
        };
        Self { 
            id,
            auth_type: auth_method.auth_type().clone(),
            identifier: auth_method.identifier().clone(),
            secret: auth_method.secret().clone(),
            created_at: auth_method.created_at().clone(),
            user_id: auth_method.user_id().clone(),
            user: extended_user
        }

    }
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
    #[get = "pub"]
    id: Uuid,
    #[get = "pub"]
    created_at: DateTime<FixedOffset>,
    #[get = "pub"]
    updated_at: Option<DateTime<FixedOffset>>,
    #[get = "pub"]
    user_roles: Vec<UserRole>,
    #[get = "pub"]
    user_attributes: Vec<UserAttribute>,
}


impl ExtendedUser {
    pub fn new(id: Uuid, created_at: DateTime<FixedOffset>, updated_at: Option<DateTime<FixedOffset>>) -> Self {
        Self { id, created_at, updated_at, user_roles: Vec::new(), user_attributes: Vec::new() }
    }

    pub fn as_base(&self) -> User {
        User::new(self.id.clone(), self.created_at.clone(), self.updated_at.clone())
    }

    pub fn add_role(&mut self, role: UserRole) {
        self.user_roles.push(role);
    }

    pub fn add_attribute(&mut self, attribute: UserAttribute) {
        self.user_attributes.push(attribute);
    }

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
            created_at: Utc::now().into(),
            updated_at: Some(Utc::now().into()),
            user_roles,
            user_attributes
        }
    }
}