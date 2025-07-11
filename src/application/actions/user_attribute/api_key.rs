
use crate::domain::errors::service::AppErrorInfo;
use crate::domain::user::models::base::AuthMethod;
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::domain::verifies::service::{ApiKeyVerifierService, PasswordVerifierService};

use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::user::factories::UserProviderFactory;

use crate::application::error_dto::ComponentErrorDTO;

use super::error::UserAttributeError;
use super::dto::ApiKeyDto;



const AUTH_TYPE: &str = "apikey"; 

const WRONG_CREDENTIALS: &str = "Incorrect login or password";

pub struct CreateApiKeyByEmailPasswdAction<CU, QU, V, A> {
    command_user_service: CU,
    query_user_service: QU,
    password_verifier: V,
    api_key_verifier: A,
}


impl<CU, QU, V, A> CreateApiKeyByEmailPasswdAction<CU, QU, V, A>
where
    CU: CommandUserService,
    QU: QueryUserService,
    V: PasswordVerifierService,
    A: ApiKeyVerifierService,
{
    pub fn new_old(
        command_user_service: CU,
        query_user_service: QU,
        password_verifier: V,
        api_key_verifier: A,
    ) -> Self {
        Self {
            command_user_service,
            query_user_service,
            password_verifier,
            api_key_verifier,
        }
    }

    pub fn new<VP, UP>(
        user_provider_factory: &UP,
        verifies_provider_factory: &VP,
    ) -> Self
    where
        VP: VerifiesProviderFactory<ApiKeyVerifier = A, PasswordVerifier = V>,
        UP: UserProviderFactory<QueryUser = QU, CommandUser = CU>
    {
        let command_user_service = user_provider_factory.command_user();
        let query_user_service = user_provider_factory.query_user();
        let api_key_verifier = verifies_provider_factory.api_key_verifier();
        let password_verifier = verifies_provider_factory.password_verifier();
        Self {
            command_user_service,
            query_user_service,
            password_verifier,
            api_key_verifier,
        }
    }


    pub async fn execute(
        &self,
        email: String,
        password: String
    ) -> Result<ApiKeyDto, UserAttributeError> {
        let user = match self.query_user_service.get_user_by_identifier(&email).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(UserAttributeError::UserNotFound(email)),
            Err(e) => return self.infrastructure_error(&e)
        };

        let Some(password_hash) = user.secret() else {
            return Err(UserAttributeError::EmailPasswdAuthNotAllowed(email))
        };

        match self.password_verifier.is_verified(&password_hash, &password) {
            Ok(true) => {}
            Ok(false) => return Err(UserAttributeError::NotCorrectPassword),
            Err(e) => return self.infrastructure_error(&e)
        };

        let mut  api_key = self
            .api_key_verifier
            .generate();

    
        let identifier = self.api_key_verifier.extract_identifier(&api_key)
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        let identifier_is = self.command_user_service.auth_identifier_is_free(identifier.clone()).await
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        if !identifier_is {
            return Err(UserAttributeError::NotCorrectApiKey);
        };
        let api_key_hash =  self.api_key_verifier.create_hash(&api_key)
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        let auth_method = AuthMethod::new(
            user.user_id().clone(),
            AUTH_TYPE.clone().to_string(),
            identifier,
            Some(api_key_hash)
        );

        self.command_user_service.add_auth_method(auth_method).await
            .map_err(|e| Self::map_infrastructure_error(&e))?;

        Ok(ApiKeyDto{api_key: api_key})
    }

    fn infrastructure_error(&self, e: &dyn AppErrorInfo) -> Result<ApiKeyDto, UserAttributeError> {
        Err(UserAttributeError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message())))
    }
    fn map_infrastructure_error(e: &dyn AppErrorInfo) -> UserAttributeError {
        UserAttributeError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message()))
    }
}
