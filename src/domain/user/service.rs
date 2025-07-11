use uuid::Uuid;

use super::models::extended::ExtendedAuthMethod;
use super::models::base::{User, UserAttribute, UserRole, AuthMethod};
use crate::domain::errors::service::AppErrorInfo;

pub trait QueryUserService {
    type Error: std::fmt::Display + AppErrorInfo;

    async fn get_user_by_identifier(&self, identifier: &str) -> Result<Option<ExtendedAuthMethod>, Self::Error>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<ExtendedAuthMethod>, Self::Error>;
}

pub trait CommandUserService {
    type Error: std::fmt::Display + AppErrorInfo;

    async fn auth_identifier_is_free(&self, identifier: String) -> Result<bool, Self::Error>;
    async fn add_user(&self) -> Result<User, Self::Error>;
    async fn add_role(&self, user_role: UserRole) -> Result<UserRole, Self::Error>;
    async fn add_user_attribute(&self, user_attribute: Vec<UserAttribute>) -> Result<Vec<UserAttribute>, Self::Error>;
    async fn add_auth_method(&self, auth_method: AuthMethod) -> Result<AuthMethod, Self::Error>;
}
