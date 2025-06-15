
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use super::config::credentials_provider::CredentialsProvider;
use super::errors::HasuraClientError;
use super::query_loader::GraphQLDLoader;
use crate::domain::settings::service::CredentialsService as _;
use crate::infrastructure::hasura::client::HasuraClient;


pub const GET_USER_BY_EMAIL: &str = "get_user_by_email";
pub const GET_USER_BY_ID: &str = "get_user_by_id";
pub const GET_USER_BY_TG_ID: &str = "get_user_by_tg_id";
pub const CREATE_USER: &str = "create_user";
pub const CREATE_ALLOWED_ROLES: &str = "create_allowed_roles";
pub const UPDATE_API_KEY_USER: &str = "update_api_key_user";

static GQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/gql");

const GQL_FILES: [&str; 6] = [
    GET_USER_BY_EMAIL,
    GET_USER_BY_ID,
    GET_USER_BY_TG_ID,
    CREATE_USER,
    CREATE_ALLOWED_ROLES,
    UPDATE_API_KEY_USER,
];

lazy_static! {
    static ref HASURA_CLIENT_CACHE: RwLock<Option<HasuraClient>> = RwLock::new(None);
}

pub struct HasuraClientManager;

impl HasuraClientManager {
    fn create_hasura_client() -> Result<HasuraClient, HasuraClientError> {
        let host = CredentialsProvider
            .get_credentials()
            .unwrap()
            .hasura_url()
            .clone();
        let mut gql_client =
            HasuraClient::new(host).map_err(|_| HasuraClientError::ErrorInitHasuraClient)?;

        let louder = GraphQLDLoader::new(&GQL_DIR);
        for filename in GQL_FILES {
            let content = louder
                .read_query(&format!("{filename}.graphql"))
                .map_err(|_| HasuraClientError::ErrorInitHasuraClient)?;
            gql_client.add_query(filename, content);
        }
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
}
