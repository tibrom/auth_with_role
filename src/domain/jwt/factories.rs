use super::service::{TokenService, JwtClaimsService};

pub trait JWTProviderFactory {
    type Claims: JwtClaimsService;
    type Tokens: TokenService;

    fn claims_service(&self) -> Self::Claims;
    fn token_service(&self) -> Self::Tokens;
}
