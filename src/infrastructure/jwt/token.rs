use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use super::config::credentials_provider::CredentialsProvider;
use crate::domain::jwt::model::{Claims, RefreshClaims};
use crate::domain::jwt::service::TokenService;
use crate::domain::settings::model::Credentials;
use crate::domain::settings::service::CredentialsService as _;

use super::error::{JwtError, StageJwtProcessing};

pub struct TokenProvider{
    credentials: Credentials
}
impl TokenProvider {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl TokenService for TokenProvider {
    type Error = JwtError;
    fn generate_access(&self, claims: Claims) -> Result<String, JwtError> {
        let secret = self.credentials.access_secret().clone();

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
        let secret = self.credentials.refresh_secret().clone();
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
        let secret = self.credentials.access_secret().clone();
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
        let secret = self.credentials.refresh_secret().clone();

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


#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;
    use crate::domain::jwt::model::{Claims, HasuraClaims, RefreshClaims};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn get_timestamp(seconds_from_now: u64) -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize + seconds_from_now as usize
    }

    fn mock_hasura_claims() -> HasuraClaims {
        HasuraClaims::new(
            "TEST".to_string(),
            vec!["TEST".to_string()],
            "user123".to_string()
        )
    }

    fn mock_claims() -> Claims {
        let expiration = Utc::now() + Duration::minutes(10);
        let exp = expiration.timestamp() as usize;
        Claims::new("TEST".to_string(), true, exp, mock_hasura_claims())
    }

    fn mock_refresh_claims() -> RefreshClaims {
        let expiration = Utc::now() + Duration::minutes(10);
        let exp = expiration.timestamp() as usize;
        RefreshClaims::new("TEST".to_string(), exp)
    }

    #[test]
    fn test_generate_and_validate_access_token() {
        let provider = TokenProvider::new(Credentials::mock());

        let claims = mock_claims();
        let token = provider.generate_access(claims.clone()).expect("Token creation failed");
        let validated_claims = provider.validate_access(&token).expect("Validation failed");

        assert_eq!(validated_claims.sub, claims.sub);
    }

    #[test]
    fn test_generate_and_validate_refresh_token() {
        let provider = TokenProvider::new(Credentials::mock());

        let claims = mock_refresh_claims();
        let token = provider.generate_refresh(claims.clone()).expect("Token creation failed");
        let validated_claims = provider.validate_refresh(&token).expect("Validation failed");

        assert_eq!(validated_claims.sub, claims.sub);
    }

    #[test]
    fn test_invalid_token_fails_validation() {
        let provider = TokenProvider::new(Credentials::mock());
        let result = provider.validate_access("not.a.real.token");

        assert!(result.is_err());
    }
}
