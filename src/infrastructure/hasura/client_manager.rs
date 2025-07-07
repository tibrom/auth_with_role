use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::settings::model::Credentials;
use crate::domain::settings::service::CredentialsService;
use crate::infrastructure::config::credentials_provider::CredentialsProvider;
use crate::infrastructure::jwt::factory::JWTProvider;


use super::client::HasuraClient;
use super::error::HasuraClientError;

lazy_static! {
    static ref HASURA_CLIENT_CACHE: RwLock<Option<HasuraClient>> = RwLock::new(None);
}

pub struct HasuraClientManager;

impl HasuraClientManager {
    fn create_hasura_client() -> Result<HasuraClient, HasuraClientError> {
        let credentials = CredentialsProvider.get_credentials()
            .map_err(|e| HasuraClientError::CredentialsError)?;
        let host = credentials.hasura_url();
        let token = Self::jwt_token(&credentials)
            .map_err(|e| HasuraClientError::CredentialsError)?;
        let mut gql_client =
            HasuraClient::new(host.clone(), Some(token));
        Ok(gql_client)
    }

    pub async fn get_hasura_client() -> Result<HasuraClient, HasuraClientError> {
        {
            let cache_lock = HASURA_CLIENT_CACHE.read().await;
            if let Some(cached) = &*cache_lock {
                return Ok(cached.clone());
            }
        }

        let hasura_client =
            Self::create_hasura_client().map_err(|_| HasuraClientError::ErrorInitHasuraClient)?;

        let mut cache_lock = HASURA_CLIENT_CACHE.write().await;
        *cache_lock = Some(hasura_client.clone());

        Ok(hasura_client)
    }



    fn jwt_token(credentials: &Credentials) -> Result<String, HasuraClientError> {
        let factory = JWTProvider::new(credentials.clone());
        let claims = factory.claims_service().inner_access_claims()
            .map_err(|_| HasuraClientError::CredentialsError)?;
        factory.token_service().generate_access(claims)
            .map_err(|_| HasuraClientError::CredentialsError)
    }
}