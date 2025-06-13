use super::model::{UserWithRole, UserNameEmailPasswordHash, AllowedRoles};


pub trait RemoteUserService {
    type Error: std::fmt::Display;

    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserWithRole>, Self::Error>;
    async fn get_user_by_id(&self, id: &str) -> Result<Option<UserWithRole>, Self::Error>;
    async fn get_user_by_tg_id(&self, tg_id: &str) -> Result<Option<UserWithRole>, Self::Error>;
    async fn create_user(&self, new_user: UserNameEmailPasswordHash) -> Result<UserWithRole, Self::Error>;
    async fn add_role(&self, allowed_roles: AllowedRoles) -> Result<UserWithRole, Self::Error>;
}



