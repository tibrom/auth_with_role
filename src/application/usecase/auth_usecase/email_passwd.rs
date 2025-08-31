use crate::application::error_ext::ServiceErrorExt;
use crate::application::usecase::auth_usecase::dto::{LoginEmailPasRequestDto, JwtResponseDto};
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::user::service::QueryUserService;
use crate::domain::verifies::service::PasswordVerifierService;

use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::user::factories::UserProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;


use super::dto::TokenPairDto;
use super::error::AuthenticatorError;
use super::constants::AUTH_TYPE;



impl<Q, V, CP, TP> ServiceErrorExt for LoginWithEmailPasswdUseCase<Q, V, CP, TP> {}

pub struct LoginWithEmailPasswdUseCase<Q, V, CP, TP> {
    user_provider: Q,
    password_verifier: V,
    claims_provider: CP,
    token_provider: TP,
}

impl<Q, V, CP, TP> LoginWithEmailPasswdUseCase<Q, V, CP, TP>
where
    Q: QueryUserService,
    V: PasswordVerifierService,
    CP: JwtClaimsService,
    TP: TokenService,
    
{
    pub fn new<T, P, U>(
        user_provider_factory: &U,
        verifies_provider_factory: &P,
        jwtprovider_factory: &T,
    ) -> Self
    where
        T: JWTProviderFactory<Claims = CP, Tokens = TP>,
        P: VerifiesProviderFactory<PasswordVerifier = V>,
        U: UserProviderFactory<QueryUser = Q>,
    {
        let claims_provider = jwtprovider_factory.claims_service();
        let token_provider = jwtprovider_factory.token_service();
        let password_verifier = verifies_provider_factory.password_verifier();
        let user_provider = user_provider_factory.query_user();
        Self {
            user_provider,
            password_verifier,
            claims_provider,
            token_provider,
        }
    }

    pub async fn execute(
        &self,
        dto: LoginEmailPasRequestDto,
    ) -> Result<JwtResponseDto, String> {
        let user = match self.user_provider.get_user_by_identifier(&dto.email, AUTH_TYPE).await {
            Ok(Some(user)) => user,
            Ok(None) => return self.handler_error(AuthenticatorError::UserNotFound(dto.email)),
            Err(e) => return self.handler_error(e)
        };

        let Some(password_hash) = user.secret() else {
            return self.handler_error(AuthenticatorError::EmailPasswdAuthNotAllowed(dto.email));
        };

        match self
            .password_verifier
            .is_verified(&password_hash, &dto.password)
        {
            Ok(true) => {}
            Ok(false) => return self.handler_error(AuthenticatorError::NotCorrectPassword),
            Err(e) => return self.handler_error(e),
        };

        let claims = match self.claims_provider.access_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let refresh_claims = match self.claims_provider.refresh_claims(&user) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let access_token = match self.token_provider.generate_access(claims) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let refresh_token = match self.token_provider.generate_refresh(refresh_claims) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let token_pair = TokenPairDto {
            access_token,
            refresh_token: Some(refresh_token),
        };


        Ok(JwtResponseDto::Success {
            auth_data: token_pair,
        })
    }

    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<JwtResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(JwtResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::settings::model::Credentials;
    
    use crate::infrastructure::verifies::factory::VerifiesProvider;
    use crate::infrastructure::jwt::factory::JWTProvider;


    use crate::mock::hasura_client::MockHasuraClientBuilder;
    use crate::mock::user_provider::MockUserProvider;
    use crate::mock::user::MockUser;

    fn login_email_pas_request_dto() -> LoginEmailPasRequestDto {
        LoginEmailPasRequestDto{
            email: MockUser::email(),
            password: MockUser::password()
        }
    }


    #[tokio::test]
    async fn verifies_provider_factory(){
        let credentials = Credentials::mock();
        let verifies_provider_factor = VerifiesProvider::new(credentials.clone());
        let jwtprovider_factory = JWTProvider::new(credentials.clone());
        let hasura_client = MockHasuraClientBuilder::new()
            .with_email_auth_method()
            .with_existing_auth_method()
            .build();
        let user_provider_factory = MockUserProvider::new(credentials.clone(), hasura_client);

        let action = LoginWithEmailPasswdUseCase::new(
            &user_provider_factory,
            &verifies_provider_factor,
            &jwtprovider_factory
        );

        let result = action.execute(login_email_pas_request_dto()).await;
        
        assert!(result.is_ok());

        let is_correct = match result.unwrap() {
            JwtResponseDto::Success{..} => true,
            _ => false
        };
        assert!(is_correct)
    }


    #[tokio::test]
    async fn user_not_found() {
        let email = "error@test.test".to_string();
        let login_data = LoginEmailPasRequestDto{
            email: email.clone(),
            password: "123456789".to_string(),
        };

        let credentials = Credentials::mock();
        let verifies_provider_factor = VerifiesProvider::new(credentials.clone());
        let jwtprovider_factory = JWTProvider::new(credentials.clone());
        let hasura_client = MockHasuraClientBuilder::new()
            .with_user_creation()
            .with_nonexistent_auth_method()
            .with_auth_method_not_found()
            .build();
        let user_provider_factory = MockUserProvider::new(credentials.clone(), hasura_client);

        let action = LoginWithEmailPasswdUseCase::new(
            &user_provider_factory,
            &verifies_provider_factor,
            &jwtprovider_factory
        );

        let result =action.execute(login_data).await;
        
        assert!(result.is_ok());

        let is_correct_err = match result.unwrap() {
            JwtResponseDto::Error{err_msg} => {
                let error = AuthenticatorError::UserNotFound(email).client_message();
                err_msg == error
            },
            _ => false
        };

        assert!(is_correct_err);
    }



    #[tokio::test]
    async fn error_search_user() {
        let credentials = Credentials::mock();
        let verifies_provider_factor = VerifiesProvider::new(credentials.clone());
        let jwtprovider_factory = JWTProvider::new(credentials.clone());
        let hasura_client = MockHasuraClientBuilder::new()
            .with_user_creation()
            .with_auth_method_not_found()
            .build();
        let user_provider_factory = MockUserProvider::new(credentials.clone(), hasura_client);

        let action = LoginWithEmailPasswdUseCase::new(
            &user_provider_factory,
            &verifies_provider_factor,
            &jwtprovider_factory
        );

        let result = action.execute(login_email_pas_request_dto()).await;
        
        assert!(result.is_err());
    }
}