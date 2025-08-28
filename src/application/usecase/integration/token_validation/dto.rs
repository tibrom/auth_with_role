use crate::domain::user::models::extended::ExtendedUser;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckJwtTokenRequestDTO{
    token: String
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedUserDTO {
    user: ExtendedUser
}
