use thiserror::Error;

#[derive(Debug, Error)]
pub enum CredentialsError {
    #[error("Failed to read credentials from cache: {0}")]
    CacheReadError(String),

    #[error("Failed to write credentials to cache: {0}")]
    CacheWriteError(String),

    // Вместо двух отдельных вариантов — один общий, но с полем `stage`
    #[error("Config error (stage = {stage}): {source}")]
    Config {
        stage: &'static str,
        #[source]
        source: config::ConfigError,
    },
}
