#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct TokenPairDto {
    pub access_token: String,
    pub refresh_token: Option<String>,
}
