#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramData {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub photo_url: Option<String>,
    pub auth_date: i64,
    pub hash: String,
}