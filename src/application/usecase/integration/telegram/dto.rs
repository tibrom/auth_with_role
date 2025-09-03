use crate::domain::integration::telegram::model::TelegramData;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TelegramDataDTO {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub photo_url: Option<String>,
    pub auth_date: i64,
    pub hash: String,
}


impl Into<TelegramData> for TelegramDataDTO {
    fn into(self) -> TelegramData {
        TelegramData {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            username: self.username,
            photo_url: self.photo_url,
            auth_date: self.auth_date,
            hash: self.hash,
        }
    }
}

pub struct TelegramCredentials{
    pub id: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl TelegramCredentials {
    pub fn new( id: String, username: String, first_name: Option<String>, last_name: Option<String>,) -> Self {
        Self { id, username, first_name, last_name}
    }
}

impl From<TelegramDataDTO> for TelegramCredentials {
    fn from(value: TelegramDataDTO) -> Self {
        Self { id: value.id.to_string(), username: value.username, first_name: value.first_name, last_name: value.last_name }
    }
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InitDataDTO {
    pub init_data: String,
}
