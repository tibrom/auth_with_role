#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SignUpRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")] // JSON: "success", "error"
pub enum SignUpResponseDto {
    Success { user: UserDataPairDto },
    Error { err_msg: String },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct UserDataPairDto {
    pub username: String,
    pub email: String,
}
