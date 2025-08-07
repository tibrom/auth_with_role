use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::settings::model::Credentials;
use crate::infrastructure::jwt::factory::JWTProvider;

use super::hasura::client::HasuraClient;
use super::hasura::error::HasuraClientError;
use super::http::client::HttpClient;

lazy_static! {
    static ref HASURA_CLIENT_CACHE: RwLock<Option<HasuraClient<HttpClient>>> = RwLock::new(None);
}

pub struct HasuraClientManager;

impl HasuraClientManager {
    fn create_http_client(credentials: &Credentials) -> Result<HttpClient, HasuraClientError> {
        let host = credentials.hasura_url();
        let token =
            Self::jwt_token(credentials).map_err(|e| HasuraClientError::CredentialsError)?;

        let client = HttpClient::new(host.clone())
            .add_header(("Authorization".to_string(), format!("Bearer {token}")))
            .add_header(("content-type".to_string(), "application/json".to_string()));
        Ok(client)
    }

    fn create_hasura_client(
        credentials: &Credentials,
    ) -> Result<HasuraClient<HttpClient>, HasuraClientError> {
        let http_client = Self::create_http_client(&credentials)?;
        let gql_client = HasuraClient::new(Box::new(http_client));
        Ok(gql_client)
    }

    pub async fn get_hasura_client(
        credentials: &Credentials,
    ) -> Result<HasuraClient<HttpClient>, HasuraClientError> {
        if let Some(cached) = Self::try_get_cached_hasura_client().await {
            return Ok(cached);
        }

        let client = Self::create_and_cache_hasura_client(credentials).await?;
        Ok(client)
    }

    async fn try_get_cached_hasura_client() -> Option<HasuraClient<HttpClient>> {
        let cache_lock = HASURA_CLIENT_CACHE.read().await;
        cache_lock.as_ref().cloned()
    }

    async fn create_and_cache_hasura_client(
        credentials: &Credentials,
    ) -> Result<HasuraClient<HttpClient>, HasuraClientError> {
        let hasura_client = Self::create_hasura_client(credentials)
            .map_err(|_| HasuraClientError::ErrorInitHasuraClient)?;

        let mut cache_lock = HASURA_CLIENT_CACHE.write().await;
        *cache_lock = Some(hasura_client.clone());

        Ok(hasura_client)
    }


    fn jwt_token(credentials: &Credentials) -> Result<String, HasuraClientError> {
        let factory = JWTProvider::new(credentials.clone());
        let claims = factory
            .claims_service()
            .inner_access_claims()
            .map_err(|_| HasuraClientError::CredentialsError)?;
        factory
            .token_service()
            .generate_access(claims)
            .map_err(|_| HasuraClientError::CredentialsError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_hasura_client_manager() {
        let credentials = Credentials::mock();
        let result = HasuraClientManager::create_hasura_client(&credentials);

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn get_hasura_client() {
        let credentials = Credentials::mock();
        let cashed_client = HasuraClientManager::try_get_cached_hasura_client().await;
        assert!(cashed_client.is_none());
        let result = HasuraClientManager::get_hasura_client(&credentials).await;
        assert!(result.is_ok());
        let cashed_client = HasuraClientManager::try_get_cached_hasura_client().await;
        assert!(cashed_client.is_some());
    }


}