use super::dto::{LoginEmailPasRequestDto, LoginEmailPasResponseDto};
use super::ServiceErrorExt;

use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};
use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::user::factories::UserProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;

use super::actions::authenticators::with_email::CreateJwtWithEmailPasswdAction;


pub struct LoginWithEmailUseCase<J, V, U> {
    jwtprovider_factory: J,
    verifies_provider_factory: V,
    user_provider_factory: U,
}

impl<J, V, U> ServiceErrorExt for LoginWithEmailUseCase<J, V, U> {}

impl<J, V, U> LoginWithEmailUseCase<J, V, U>
where
    J: JWTProviderFactory,
    V: VerifiesProviderFactory,
    U: UserProviderFactory
{
    pub fn new(
        jwtprovider_factory: J,
        verifies_provider_factory: V,
        user_provider_factory: U,
    ) -> Self {
        Self {
            jwtprovider_factory,
            verifies_provider_factory,
            user_provider_factory,
        }
    }

    pub async fn login(
        &self,
        dto: LoginEmailPasRequestDto,
    ) -> Result<LoginEmailPasResponseDto, String> {

        let create_jwt_sub_case = CreateJwtWithEmailPasswdAction::new(
            &self.user_provider_factory,
            &self.verifies_provider_factory,
            &self.jwtprovider_factory
        );

        let token_pair_dto = match create_jwt_sub_case.execute(dto.email.clone(), dto.password.clone()).await {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        Ok(LoginEmailPasResponseDto::Success {
            auth_data: token_pair_dto,
        })
    }

    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<LoginEmailPasResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(LoginEmailPasResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }
}
