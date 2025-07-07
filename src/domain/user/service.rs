use super::model::{AllowedRoles, UserNameEmailPasswordHash, UserWithRole};
use crate::domain::errors::service::AppErrorInfo;

pub trait QueryUserService {
    type Error: std::fmt::Display + AppErrorInfo;

    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserWithRole>, Self::Error>;
    async fn get_user_by_id(&self, id: &str) -> Result<Option<UserWithRole>, Self::Error>;
    async fn get_user_by_tg_id(&self, tg_id: &str) -> Result<Option<UserWithRole>, Self::Error>;
}

pub trait CommandUserService {
    type Error: std::fmt::Display + AppErrorInfo;

    async fn create_user(
        &self,
        new_user: UserNameEmailPasswordHash,
    ) -> Result<UserWithRole, Self::Error>;
    async fn add_role(
        &self,
        user: UserWithRole,
        allowed_roles: AllowedRoles,
    ) -> Result<UserWithRole, Self::Error>;
    async fn add_api_hash(&self, user: UserWithRole, api_hash: &str) -> Result<UserWithRole, Self::Error>;

    async fn set_default_role(&self, user_with_role: UserWithRole) -> Result<UserWithRole, Self::Error>;
}
