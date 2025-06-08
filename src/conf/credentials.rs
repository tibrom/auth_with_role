use config::Config;
use getset::{Getters, Setters};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::RwLock};

use super::errors::CredentialsError;



#[derive(Getters, Setters, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Default)]
pub struct Credentials {
    #[get = "pub"]
    #[set = "pub"]
    host: String,
    #[get = "pub"]
    #[set = "pub"]
    port: String,
    #[get = "pub"]
    #[set = "pub"]
    role: String,
    #[get = "pub"]
    #[set = "pub"]
    expiration_access_hours: usize,
    #[get = "pub"]
    #[set = "pub"]
    expiration_refresh_days: usize,
    #[get = "pub"]
    #[set = "pub"]
    access_secret: String,
    #[get = "pub"]
    #[set = "pub"]
    refresh_secret: String,
    #[get = "pub"]
    #[set = "pub"]
    hasura_url: String,
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
pub struct CredentialsManager;

impl CredentialsManager {
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
