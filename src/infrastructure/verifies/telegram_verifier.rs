use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use url::form_urlencoded;

use crate::domain::settings::model::Credentials;
use crate::domain::integration::telegram::model::TelegramData;
use crate::domain::integration::telegram::verifier::TelegramVerifierService;
use super::errors::TelegramVerifierError;

type HmacSha256 = Hmac<Sha256>;

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
        // 1) распарсить query-строку с URL-декодом
        let mut pairs: Vec<(String, String)> =
            url::form_urlencoded::parse(init_data.as_bytes()).into_owned().collect();

        // достать hash
        let received_hash = pairs
            .iter()
            .find(|(k, _)| k == "hash")
            .map(|(_, v)| v.clone())
            .ok_or_else(|| TelegramVerifierError::Message("hash is missing in init_data".to_string()))?;

        // 2) убрать hash; (signature можно оставлять/убирать — она не нужна для проверки)
        pairs.retain(|(k, _)| k != "hash");

        // 3) отсортировать по ключу
        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        // 4) собрать data_check_string
        let data_check_string = pairs
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("\n");

        // 5) секрет = HMAC("WebAppData", bot_token)
        let mut mac1 = HmacSha256::new_from_slice(b"WebAppData")
            .map_err(|_| TelegramVerifierError::Message("HMAC init failed".to_string()))?;
        mac1.update(bot_token.as_bytes());
        let secret = mac1.finalize().into_bytes(); // bytes, не hex!

        // 6) подпись = HMAC(secret, data_check_string)
        let mut mac2 = HmacSha256::new_from_slice(&secret)
            .map_err(|_| TelegramVerifierError::Message("HMAC init failed".to_string()))?;
        mac2.update(data_check_string.as_bytes());
        let calc = mac2.finalize().into_bytes();
        let calc_hex = hex::encode(calc); // hex в нижнем регистре

        Ok(calc_hex.eq_ignore_ascii_case(&received_hash))
    }
}
