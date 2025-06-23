
use crate::domain::errors::service::AppErrorInfo;
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::domain::verifies::service::{ApiKeyVerifierService, PasswordVerifierService};

use crate::domain::verifies::factories::VerifiesProviderFactory;
use crate::domain::user::factories::UserProviderFactory;

use super::error::UserAttributeError;
use super::error_dto::ComponentErrorDTO;
use super::dto::ApiKeyDto;




const WRONG_CREDENTIALS: &str = "Incorrect login or password";

pub struct CreateApiKeyByEmailPasswdSubCase<CU, QU, V, A> {
    command_user_service: CU,
    query_user_service: QU,
    password_verifier: V,
    api_key_verifier: A,
}


impl<CU, QU, V, A> CreateApiKeyByEmailPasswdSubCase<CU, QU, V, A>
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
        let user = match self.query_user_service.get_user_by_email(&email).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(UserAttributeError::UserNotFound(email)),
            Err(e) => return self.infrastructure_error(&e)
        };

        let Some(password_hash) = user.password_hash() else {
            return Err(UserAttributeError::EmailPasswdAuthNotAllowed(email))
        };

        match self.password_verifier.is_verified(&password_hash, &password) {
            Ok(true) => {}
            Ok(false) => return Err(UserAttributeError::NotCorrectPassword),
            Err(e) => return self.infrastructure_error(&e)
        };

        let api_key = self
            .api_key_verifier
            .generate(user.id().clone());

        let api_key_hash = match self.api_key_verifier.create_hash(&api_key) {
            Ok(v) => v,
            Err(e) => return self.infrastructure_error(&e),
        };

        if let Err(e) = self
            .command_user_service
            .add_api_hash(&user.id().to_string(), &api_key_hash)
            .await
        {
            return self.infrastructure_error(&e);
        };

        Ok(ApiKeyDto{api_key: api_key_hash})
    }

    fn infrastructure_error(&self, e: &dyn AppErrorInfo) -> Result<ApiKeyDto, UserAttributeError> {
        Err(UserAttributeError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message())))
    }
}
