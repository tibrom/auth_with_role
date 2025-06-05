use std::{collections::HashMap, sync::RwLock};
use config::Config;
use lazy_static::lazy_static;

use super::errors::CredentialsError;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default)]
pub struct Credentials {
    pub host: String,
    pub port: String,
    pub role: String,
    pub jwt_secret: String,
    pub jwt_refresh_secret: String,
    pub hasura_url: String,
}

lazy_static! {
    static ref CREDENTIALS_CACHE: RwLock<Option<Credentials>> = RwLock::new(None);
}

/// A utility for loading and caching application credentials.
///
/// `CredentialsManager` loads credentials from a configuration file
/// (`credentials.toml`) and/or environment variables with the `AUTH_` prefix.
/// 
/// The first successful load is cached in memory using a thread-safe
/// `RwLock`, reducing the need to re-read configuration on every access.
///
/// # Errors
///
/// Returns a [`CredentialsError`] if:
/// - The configuration file is missing or invalid.
/// - Environment variables are malformed.
/// - Reading or writing to the credentials cache fails.
pub struct CredentialsGetter;

impl CredentialsGetter {
    /// Returns credentials either from the in-memory cache or by loading from config sources.
    ///
    /// Loads from:
    /// - A TOML file named `credentials.toml`
    /// - Environment variables with the `AUTH_` prefix
    ///
    /// Caches the result after the first successful load.
    pub fn get_credentials() -> Result<Credentials, CredentialsError> {
        
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
            .map_err(|e| CredentialsError::Config { stage: "build", source: e })?;

        let credentials: Credentials = config
            .try_deserialize()
            .map_err(|e| CredentialsError::Config { stage: "deserialize", source: e })?;

        let mut cache_lock = CREDENTIALS_CACHE
            .write()
            .map_err(|e| CredentialsError::CacheWriteError(e.to_string()))?;
        *cache_lock = Some(credentials.clone());

        Ok(credentials)
    }
}
