use super::dto::{SignUpRequestDto, SignUpResponseDto};
use crate::application::auth::dto::TokenPairDto;
use crate::application::sign_up::dto::UserDataPairDto;
use crate::domain::user::model::{AllowedRoles, UserNameEmailPasswordHash};
use crate::domain::verifies::service::PasswordVerifierService;
use crate::domain::settings::service::CredentialsService;
use crate::domain::user::service::RemoteUserService;

const WRONG_CREDENTIALS: &str = "Incorrect login or password";
const INTERNAL_ERROR_SERVER: &str = "Internal error";


pub struct SignUpWithEmailUseCase<U, V, C>{
    user_provider: U,
    verifier: V,
    credentials_provider: C

}

impl<U, V, C> SignUpWithEmailUseCase<U, V, C>
where
    V: PasswordVerifierService,
    U: RemoteUserService,
    C: CredentialsService {

    pub fn new(user_provider: U, verifier: V, credentials_provider: C) -> Self {
        Self {
            user_provider,
            verifier,
            credentials_provider
        }
    }

    pub async fn sign_up(&self, user: SignUpRequestDto) -> Result<SignUpResponseDto, String> {
        let password_hash = self.verifier.create_hash(&user.password)
            .map_err(|e| format!("Password hash {e}"))?;

        let new_user_data= UserNameEmailPasswordHash::new(&user.username, &user.email, &password_hash);
        let new_user = match self.user_provider.create_user(new_user_data).await {
            Ok(u) => u,
            Err(e) => {
                tracing::error!("User not created: {e}");
                return Err(format!("User not created: {e}"));
            }
        };
        let default_role = self.credentials_provider.get_credentials()
            .map(|v| v.new_user_role().clone())
            .map_err(|e| format!("Credentials not allowed"))?;

        let allowed_roles = AllowedRoles::new_default(&default_role, new_user.id());

        match self.user_provider.add_role(allowed_roles).await {
            Ok(u) => {
                let email = u.email().clone().unwrap_or_else(||"".to_string());
                let user = UserDataPairDto {
                    username: u.username().clone(),
                    email
                };
                Ok(SignUpResponseDto::Success { user })
            },
            Err(e) => {

                Err("Error search new user".to_string())
            }
        }
    }
    

    fn internal_error(&self) -> Result<SignUpResponseDto, String> {
        Err(INTERNAL_ERROR_SERVER.to_string())
    }
}
