use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use url::form_urlencoded;

use crate::domain::settings::model::Credentials;
use crate::domain::integration::telegram::model::TelegramData;
use crate::domain::integration::telegram::verifier::TelegramVerifierService;
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
    fn is_verified_telegram_data(&self, telegram_data: TelegramData) -> Result<bool, Self::Error> {
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

    fn is_verified_mini_app_data(&self, init_data: &str) -> Result<bool, Self::Error> {
        let bot_token = self.credentials.bot_token().clone();
        let pairs: Vec<(String, String)> =
            form_urlencoded::parse(init_data.as_bytes()).into_owned().collect();

        let mut data_map = BTreeMap::new();
        let mut received_hash = String::new();

        for (k, v) in pairs {
            if k == "hash" {
                received_hash = v;
            } else {
                data_map.insert(k, v);
            }
        }

        let data_check_string = data_map
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("\n");

        tracing::info!("Data check string: {}", data_check_string);
        // Хэшируем bot_token
        let secret_key = Sha256::digest(bot_token.as_bytes());

        // Подпись
        let mut mac = Hmac::<Sha256>::new_from_slice(&secret_key).unwrap();
        mac.update(data_check_string.as_bytes());

        let result = mac.finalize();
        let calculated_hash = hex::encode(result.into_bytes());

        Ok(calculated_hash == received_hash)
    }
}
