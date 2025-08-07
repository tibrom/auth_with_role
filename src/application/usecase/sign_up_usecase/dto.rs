#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SignUpRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")] // JSON: "success", "error"
pub enum SignUpResponseDto {
    Success { user: UserDataDto },
    Error { err_msg: String },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserDataDto {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ApiKeyDto {
    pub api_key: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CreateApiKeyRequestDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")] // JSON: "success", "error"
pub enum CreateApiKeyResponseDto {
    Success { auth_data: ApiKeyDto },
    Error { err_msg: String },
}