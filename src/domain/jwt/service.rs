use super::model::{Claims, RefreshClaims};

use super::UserWithRole;

pub trait JwtClaimsService {
    type Error;

    fn access_claims(&self, user: &UserWithRole) -> Result<Claims, Self::Error>;
    fn refresh_claims(&self, user: &UserWithRole) -> Result<RefreshClaims, Self::Error>;
    fn inner_access_claims(&self, ) -> Result<Claims, Self::Error>;
}

pub trait TokenService: Send + Sync {
    type Error;

    fn generate_access(&self, claims: Claims) -> Result<String, Self::Error>;
    fn generate_refresh(&self, claims: RefreshClaims) -> Result<String, Self::Error>;
    fn validate_access(&self, token: &str) -> Result<Claims, Self::Error>;
    fn validate_refresh(&self, token: &str) -> Result<RefreshClaims, Self::Error>;
}