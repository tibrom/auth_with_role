use super::service::{TokenService, JwtClaimsService};

pub trait JWTProviderFactory {
    type Claims: JwtClaimsService + Send;
    type Tokens: TokenService + Send;

    fn claims_service(&self) -> Self::Claims;
    fn token_service(&self) -> Self::Tokens;
}
