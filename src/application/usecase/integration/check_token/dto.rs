use crate::domain::user::models::extended::ExtendedUser;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")] // JSON: "success", "error"
pub enum CheckTokenResponseDto {
    Success { user: ExtendedUser },
    NotValidToken,
    Error { err_msg: String },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CheckTokenRequestDto {
    pub token: String,
}