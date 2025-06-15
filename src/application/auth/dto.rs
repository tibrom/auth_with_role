#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct LoginEmailPasRequestDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")] // JSON: "success", "error"
pub enum LoginEmailPasResponseDto {
    Success { auth_data: TokenPairDto },
    Error { err_msg: String },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct TokenPairDto {
    pub access_token: String,
    pub refresh_token: String,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CreateApiKeyRequestDto {
    pub email: String,
    pub password: String,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")] // JSON: "success", "error"
pub enum CreateApiKeyResponseDto {
    Success { api_key: String },
    Error { err_msg: String },
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct LoginApiKeyRequestDto {
    pub api_key: String,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")] // JSON: "success", "error"
pub enum LoginApiKeyResponseDto {
    Success { access_token: String },
    Error { err_msg: String },
}

