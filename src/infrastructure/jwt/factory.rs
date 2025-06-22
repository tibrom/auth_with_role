use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::settings::model::Credentials;

use super::claims::ClaimsProvider;
use super::token::TokenProvider;


pub struct JWTProvider{
    credentials: Credentials,
}
impl JWTProvider {
    pub fn new(credentials: Credentials) -> Self {
        Self{
            credentials
        }
    }
}

impl JWTProviderFactory for JWTProvider  {
    type Claims = ClaimsProvider;
    type Tokens = TokenProvider;
    fn claims_service(&self) -> Self::Claims {
        ClaimsProvider::new(self.credentials.clone())
    }
    fn token_service(&self) -> Self::Tokens {
        TokenProvider
    }
}