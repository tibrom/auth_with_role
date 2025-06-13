use chrono::{DateTime, FixedOffset};
use getset::{Getters, Setters};
use uuid::Uuid;
use crate::errors::{DomainError, ValidationError, BusinessRuleError};

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

impl User {
    pub fn new(
        username: String,
        email: Option<String>,
        telegram_id: Option<String>,
        password_hash: Option<String>,
    ) -> Result<Self, DomainError> {
        // Валидация username
        if username.is_empty() {
            return Err(DomainError::Validation(ValidationError::EmptyValue("username".to_string())));
        }
        if username.len() < 3 || username.len() > 50 {
            return Err(DomainError::Validation(ValidationError::InvalidLength("username".to_string())));
        }

        // Валидация email если он предоставлен
        if let Some(email) = &email {
            if !is_valid_email(email) {
                return Err(DomainError::Validation(ValidationError::InvalidEmail));
            }
        }

        Ok(Self {
            id: Uuid::new_v4(),
            created_at: Some(chrono::Utc::now().with_timezone(&FixedOffset::east(0))),
            updated_at: None,
            username,
            email,
            telegram_id,
            password_hash,
            api_key_hash: None,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn set_email(&mut self, email: String) -> Result<(), DomainError> {
        if !is_valid_email(&email) {
            return Err(DomainError::Validation(ValidationError::InvalidEmail));
        }
        self.email = Some(email);
        self.updated_at = Some(chrono::Utc::now().with_timezone(&FixedOffset::east(0)));
        Ok(())
    }

    pub fn set_password_hash(&mut self, password_hash: String) -> Result<(), DomainError> {
        if password_hash.is_empty() {
            return Err(DomainError::Validation(ValidationError::EmptyValue("password_hash".to_string())));
        }
        self.password_hash = Some(password_hash);
        self.updated_at = Some(chrono::Utc::now().with_timezone(&FixedOffset::east(0)));
        Ok(())
    }
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
    pub fn new(username: &str, email: &str, password_hash: &str) -> Result<Self, DomainError> {
        // Валидация username
        if username.is_empty() {
            return Err(DomainError::Validation(ValidationError::EmptyValue("username".to_string())));
        }
        if username.len() < 3 || username.len() > 50 {
            return Err(DomainError::Validation(ValidationError::InvalidLength("username".to_string())));
        }

        // Валидация email
        if !is_valid_email(email) {
            return Err(DomainError::Validation(ValidationError::InvalidEmail));
        }

        // Валидация password_hash
        if password_hash.is_empty() {
            return Err(DomainError::Validation(ValidationError::EmptyValue("password_hash".to_string())));
        }

        Ok(Self {
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
        })
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

impl UserWithRole {
    pub fn add_role(&mut self, role: AllowedRole) -> Result<(), DomainError> {
        // Проверка на дубликат роли
        if self.allowed_roles.iter().any(|r| r.role() == role.role()) {
            return Err(DomainError::BusinessRule(BusinessRuleError::EntityAlreadyExists(
                format!("Role '{}' already exists for user", role.role())
            )));
        }
        self.allowed_roles.push(role);
        self.updated_at = Some(chrono::Utc::now().with_timezone(&FixedOffset::east(0)));
        Ok(())
    }

    pub fn remove_role(&mut self, role_name: &str) -> Result<(), DomainError> {
        let initial_len = self.allowed_roles.len();
        self.allowed_roles.retain(|r| r.role() != role_name);
        
        if self.allowed_roles.len() == initial_len {
            return Err(DomainError::BusinessRule(BusinessRuleError::EntityNotFound(
                format!("Role '{}' not found for user", role_name)
            )));
        }
        
        self.updated_at = Some(chrono::Utc::now().with_timezone(&FixedOffset::east(0)));
        Ok(())
    }
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
    pub fn new_default(role: &str, user_id: &Uuid) -> Result<Self, DomainError> {
        if role.is_empty() {
            return Err(DomainError::Validation(ValidationError::EmptyValue("role".to_string())));
        }
        
        Ok(Self {
            id: None,
            role: role.to_string(),
            is_default: true,
            created_at: Some(chrono::Utc::now().with_timezone(&FixedOffset::east(0))),
            user_id: user_id.clone(),
        })
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Role {
    value: String,
}

impl Role {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        if value.is_empty() {
            return Err(DomainError::Validation(ValidationError::EmptyValue("role".to_string())));
        }
        Ok(Self { value: value.to_string() })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

// Вспомогательная функция для валидации email
fn is_valid_email(email: &str) -> bool {
    // Простая валидация email
    email.contains('@') && email.contains('.')
}