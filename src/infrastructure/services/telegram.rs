use crate::domain::integration::telegram::service::{
    ParserInitDataService,
    FactoryTelegramInitDataParser
};


use serde_json::Value;
use url::form_urlencoded;
use urlencoding::decode;



pub struct ParsedInitData {
    pub data: Value,
}

impl ParsedInitData {
    pub fn from_init_data(init_data: &str) -> Self {
        let pairs: Vec<(String, String)> =
            form_urlencoded::parse(init_data.as_bytes()).into_owned().collect();

        let mut map = serde_json::Map::new();

        for (k, v) in pairs {
            if k == "user" {
                let decoded = decode(&v).unwrap();
                if let Ok(user_json) = serde_json::from_str::<Value>(&decoded) {
                    map.insert("user".to_string(), user_json);
                }
            } else {
                map.insert(k, Value::String(v));
            }
        }

        ParsedInitData {
            data: Value::Object(map),
        }
    }
}

impl ParserInitDataService for ParsedInitData {
    fn get_tg_id(&self) -> Option<i64> {
        self.data.get("user")?
            .get("id")?
            .as_i64()
    }

    fn get_tg_username(&self) -> Option<String> {
        self.data.get("user")?
            .get("username")?
            .as_str()
            .map(|s| s.to_string())
    }

    fn first_name(&self) -> Option<String> {
        self.data.get("user")?
            .get("first_name")?
            .as_str()
            .map(|s| s.to_string())
    }

    fn last_name(&self) -> Option<String> {
        self.data.get("user")?
            .get("last_name")?
            .as_str()
            .map(|s| s.to_string())
    }
}


pub struct FactoryParsedInitDataParser;

impl FactoryTelegramInitDataParser for FactoryParsedInitDataParser {
    type Service = ParsedInitData;
    fn create(&self, init_data: String) -> Self::Service {
        ParsedInitData::from_init_data(&init_data)
    }
}