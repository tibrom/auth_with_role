use crate::application::error_ext::ServiceErrorExt;
use crate::application::usecase::sign_up_usecase::dto::{ApiKeyDto, CreateApiKeyRequestDto, CreateApiKeyResponseDto, SignUpRequestDto};
use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::user::models::base::AuthMethod;
use crate::domain::user::service::{CommandUserService, QueryUserService};
use crate::domain::verifies::service::{ApiKeyVerifierService, PasswordVerifierService};

use crate::domain::user::factories::UserProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;

use super::error::UserAttributeError;

const AUTH_TYPE: &str = "apikey";

const WRONG_CREDENTIALS: &str = "Incorrect login or password";

pub struct CreateApiKeyUseCase<CU, QU, V, A> {
    command_user_service: CU,
    query_user_service: QU,
    password_verifier: V,
    api_key_verifier: A,
}


impl<CU, QU, V, A> ServiceErrorExt for CreateApiKeyUseCase<CU, QU, V, A> {}


impl<CU, QU, V, A> CreateApiKeyUseCase<CU, QU, V, A>
where
    CU: CommandUserService,
    QU: QueryUserService,
    V: PasswordVerifierService,
    A: ApiKeyVerifierService,
{
    pub fn new<VP, UP>(user_provider_factory: &UP, verifies_provider_factory: &VP) -> Self
    where
        VP: VerifiesProviderFactory<ApiKeyVerifier = A, PasswordVerifier = V>,
        UP: UserProviderFactory<QueryUser = QU, CommandUser = CU>,
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
        sing_up_user: CreateApiKeyRequestDto
    ) -> Result<CreateApiKeyResponseDto, String> {
        let user = match self.query_user_service.get_user_by_identifier(&sing_up_user.email, AUTH_TYPE).await {
            Ok(Some(user)) => user,
            Ok(None) => return self.handler_error(UserAttributeError::UserNotFound(sing_up_user.email)),
            Err(e) => return self.handler_error(e),
        };

        let Some(password_hash) = user.secret() else {
            return self.handler_error(UserAttributeError::EmailPasswdAuthNotAllowed(sing_up_user.email));
        };

        match self
            .password_verifier
            .is_verified(&password_hash, &sing_up_user.password)
        {
            Ok(true) => {}
            Ok(false) => return self.handler_error(UserAttributeError::NotCorrectPassword),
            Err(e) => return self.handler_error(e),
        };

        let api_key = self.api_key_verifier.generate();

        let identifier = match self.api_key_verifier.extract_identifier(&api_key) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let identifier_is = match self.command_user_service.auth_identifier_is_free(identifier.clone(), AUTH_TYPE).await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        if !identifier_is {
            return self.handler_error(UserAttributeError::NotCorrectApiKey);
        };
        let api_key_hash = match self.api_key_verifier.create_hash(&api_key) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        let auth_method = AuthMethod::new(
            user.user_id().clone(),
            AUTH_TYPE.to_string(),
            identifier,
            Some(api_key_hash),
        );

        if let Err(e) = self.command_user_service.add_auth_method(auth_method).await {
            return self.handler_error(e);
        }
            
        let api_key = ApiKeyDto { api_key: api_key };

        Ok(CreateApiKeyResponseDto::Success { auth_data: api_key })
    }


    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<CreateApiKeyResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(CreateApiKeyResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }
}
