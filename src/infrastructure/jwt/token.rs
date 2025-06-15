use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use super::config::credentials_provider::CredentialsProvider;
use crate::domain::jwt::model::{Claims, HasuraClaims, RefreshClaims};
use crate::domain::jwt::service::TokenService;
use crate::domain::settings::service::CredentialsService as _;

use super::error::{JwtError, StageJwtProcessing};

pub struct TokenProvider;

impl TokenService for TokenProvider {
    type Error = JwtError;
    fn generate_access(&self, claims: Claims) -> Result<String, JwtError> {
        let secret = CredentialsProvider
            .get_credentials()
            .map(|v| v.access_secret().clone())
            .map_err(|e| JwtError::CredentialsUnavailable(e))?;

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| JwtError::JwtProcessingError {
            stage: StageJwtProcessing::Encode,
            source: e,
        })
    }

    fn generate_refresh(&self, claims: RefreshClaims) -> Result<String, JwtError> {
        let secret = CredentialsProvider
            .get_credentials()
            .map(|v| v.access_secret().clone())
            .map_err(|e| JwtError::CredentialsUnavailable(e))?;

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| JwtError::JwtProcessingError {
            stage: StageJwtProcessing::Encode,
            source: e,
        })
    }

    fn validate_access(&self, token: &str) -> Result<Claims, JwtError> {
        let secret = CredentialsProvider
            .get_credentials()
            .map(|v| v.access_secret().clone())
            .map_err(|e| JwtError::CredentialsUnavailable(e))?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| JwtError::JwtProcessingError {
            stage: StageJwtProcessing::Decode,
            source: e,
        })?;
        Ok(token_data.claims)
    }

    fn validate_refresh(&self, token: &str) -> Result<RefreshClaims, JwtError> {
        let secret = CredentialsProvider
            .get_credentials()
            .map(|v| v.access_secret().clone())
            .map_err(|e| JwtError::CredentialsUnavailable(e))?;

        let token_data = decode::<RefreshClaims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| JwtError::JwtProcessingError {
            stage: StageJwtProcessing::Decode,
            source: e,
        })?;
        Ok(token_data.claims)
    }
}
