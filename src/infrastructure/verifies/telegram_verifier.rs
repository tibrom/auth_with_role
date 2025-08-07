use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use crate::domain::settings::model::Credentials;
use crate::domain::verifies::service::TelegramVerifierService;
use crate::domain::verifies::model::TelegramData;
use super::errors::TelegramVerifierError;

pub struct TelegramVerifier{
    credentials: Credentials
}
impl TelegramVerifier {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}


impl TelegramVerifierService for TelegramVerifier {
    type Error = TelegramVerifierError;
    fn is_verified(&self, telegram_data: TelegramData) -> Result<bool, Self::Error> {
        let bot_token = self.credentials.bot_token().clone();
        let mut data_map = BTreeMap::new();

        data_map.insert("auth_date", telegram_data.auth_date.to_string());
        data_map.insert("id", telegram_data.id.to_string());
        data_map.insert("username", telegram_data.username.clone());

        if let Some(first_name) = &telegram_data.first_name {
            data_map.insert("first_name", first_name.clone());
        }
        if let Some(last_name) = &telegram_data.last_name {
            data_map.insert("last_name", last_name.clone());
        }
        if let Some(photo_url) = &telegram_data.photo_url {
            data_map.insert("photo_url", photo_url.clone());
        }

        let data_check_string = data_map
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("\n");

        // Хэшируем bot_token
        let secret_key = Sha256::digest(bot_token.as_bytes());

        // Подпись
        let mut mac = Hmac::<Sha256>::new_from_slice(&secret_key).unwrap();
        mac.update(data_check_string.as_bytes());

        let result = mac.finalize();
        let calculated_hash = hex::encode(result.into_bytes());

        Ok(calculated_hash == telegram_data.hash)
    }
}
