use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::settings::model::Credentials;

use super::claims::ClaimsProvider;
use super::token::TokenProvider;

pub struct JWTProvider {
    credentials: Credentials,
}
impl JWTProvider {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl JWTProviderFactory for JWTProvider {
    type Claims = ClaimsProvider;
    type Tokens = TokenProvider;
    fn claims_service(&self) -> Self::Claims {
        ClaimsProvider::new(self.credentials.clone())
    }
    fn token_service(&self) -> Self::Tokens {
        TokenProvider::new(self.credentials.clone())
    }
}


#[cfg(test)]
mod test {
    use crate::infrastructure::jwt::claims;

    use super::*;

    #[test]
    fn create_provider(){
        let credentials = Credentials::mock();
        let provider_factory = JWTProvider::new(credentials.clone());
        let claims_service = provider_factory.claims_service();
        let accept_claims_service = ClaimsProvider::new(credentials.clone());
        
        let token_service = provider_factory.token_service();
        let accept_token_service = TokenProvider::new(credentials.clone());

        
    }
}