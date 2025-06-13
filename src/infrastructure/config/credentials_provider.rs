use config::Config;
use lazy_static::lazy_static;
use std::sync::RwLock;

use super::errors::CredentialsError;
use crate::domain::settings::model::Credentials;
use crate::domain::settings::service::CredentialsService;


lazy_static! {
    static ref CREDENTIALS_CACHE: RwLock<Option<Credentials>> = RwLock::new(None);
}


pub struct CredentialsProvider;

impl CredentialsService for CredentialsProvider {
    type Error = CredentialsError;
    fn get_credentials(&self) -> Result<Credentials, Self::Error> {
        {
            let cache_lock = CREDENTIALS_CACHE
                .read()
                .map_err(|e| CredentialsError::CacheReadError(e.to_string()))?;
            if let Some(cached) = &*cache_lock {
                return Ok(cached.clone());
            }
        }

        let config = Config::builder()
            .add_source(config::File::with_name("credentials"))
            .add_source(config::Environment::with_prefix("AUTH"))
            .build()
            .map_err(|e| CredentialsError::Config {
                stage: "build",
                source: e,
            })?;

        let credentials: Credentials =
            config
                .try_deserialize()
                .map_err(|e| CredentialsError::Config {
                    stage: "deserialize",
                    source: e,
                })?;

        let mut cache_lock = CREDENTIALS_CACHE
            .write()
            .map_err(|e| CredentialsError::CacheWriteError(e.to_string()))?;
        *cache_lock = Some(credentials.clone());

        Ok(credentials)
    }
}
