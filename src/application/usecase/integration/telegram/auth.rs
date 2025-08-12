use std::str::FromStr;
use uuid::Uuid;

use crate::application::error_ext::ServiceErrorExt;
use crate::application::usecase::auth_usecase::dto::{JwtResponseDto, TokenPairDto};
use crate::application::usecase::integration::telegram::errors::LinkAccountError;

use crate::domain::errors::service::{AppErrorInfo, ErrorLevel};

use crate::domain::jwt::service::{JwtClaimsService, TokenService};
use crate::domain::jwt::factories::JWTProviderFactory;

use crate::domain::settings::model::Credentials;

use crate::domain::user::models::base::UserRole;
use crate::domain::user::models::extended::ExtendedUser;
use crate::domain::user::service::{QueryUserService, CommandUserService};
use crate::domain::user::factories::UserProviderFactory;

use crate::domain::verifies::model::TelegramData;
use crate::domain::verifies::service::TelegramVerifierService;
use crate::domain::verifies::factories::VerifiesProviderFactory;



use super::dto::TelegramDataDTO;
use super::add_cred::AddTelegramCredUseCase;

use super::constants::{AUTH_TYPE, TELEGRAM_USERNAME, TELEGRAM_LAST_NAME, TELEGRAM_FIRST_NAME};


pub struct AuthTelegramUseCase<CUS, QUS, V, CP, TP> {
    credentials: Credentials,
    command_user_service: CUS,
    query_user_service: QUS,
    telegram_verifier: V,
    claims_provider: CP,
    token_provider: TP,
    add_telegram_cred_use_case: AddTelegramCredUseCase<CUS>,
}

impl<CUS, QUS, V, CP, TP> ServiceErrorExt for AuthTelegramUseCase<CUS, QUS, V, CP, TP> {}

impl <CUS, QUS, V, CP, TP> AuthTelegramUseCase<CUS, QUS, V, CP, TP>
where
    CUS: CommandUserService,
    QUS: QueryUserService,
    V: TelegramVerifierService,
    CP: JwtClaimsService,
    TP: TokenService,
{
    pub fn new<T, P, U>(
        credentials: Credentials,
        user_provider_factory: &U,
        verifies_provider_factory: &P,
        jwtprovider_factory: &T,
    ) -> Self
    where
        T: JWTProviderFactory<Claims = CP, Tokens = TP>,
        P: VerifiesProviderFactory<TelegramVerifierService = V>,
        U: UserProviderFactory<QueryUser = QUS, CommandUser = CUS>,
    {
        let claims_provider = jwtprovider_factory.claims_service();
        let token_provider = jwtprovider_factory.token_service();
        let telegram_verifier = verifies_provider_factory.telegram_verifier();
        let command_user_service = user_provider_factory.command_user();
        let query_user_service = user_provider_factory.query_user();
        let add_telegram_cred_use_case = AddTelegramCredUseCase::new(credentials.clone(), user_provider_factory);
        Self {
            credentials,
            command_user_service,
            query_user_service,
            telegram_verifier,
            claims_provider,
            token_provider,
            add_telegram_cred_use_case
        }
    }

    pub async fn execute(&self, dto: TelegramDataDTO) -> Result<JwtResponseDto, String> {
        let extended_auth_method = match self.query_user_service.get_user_by_identifier(&dto.id.to_string(), AUTH_TYPE).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                let user = match self.command_user_service.add_user().await{
                    Ok(v) => v,
                    Err(e) => return self.handler_error(e)
                };
                let user_role = UserRole::new(
                    true,
                    self.credentials.new_user_role().with_email().clone(),
                    user.id().clone(),
                );

                if let Err(e) = self.command_user_service.add_role(user_role).await{
                    return self.handler_error(e);
                }

                let extended_user = ExtendedUser::new(user.id().clone(), user.created_at().clone(), user.updated_at().clone());

                let extended_auth_method = match self.add_telegram_cred_use_case.execute(extended_user, dto.clone()).await {
                    Ok(v) => v,
                    Err(e) => return self.handler_error(e)
                };
                extended_auth_method
            },
            Err(e) => return self.handler_error(e)
        };

        let telegram_data: TelegramData = dto.into();

        let is_verified = match self.telegram_verifier.is_verified(telegram_data.clone()) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e)
        };

        if !is_verified {
            return self.handler_error(LinkAccountError::NotVerified);
        }

        
        let claims = match self.claims_provider.access_claims(&extended_auth_method) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let access_token = match self.token_provider.generate_access(claims) {
            Ok(v) => v,
            Err(e) => return self.handler_error(e),
        };

        let token_pair = TokenPairDto {
            access_token,
            refresh_token: None, // Telegram integration may not require refresh tokens
        };

        Ok(JwtResponseDto::Success { auth_data: token_pair })
    }

    fn handler_error<E: AppErrorInfo>(&self, e: E) -> Result<JwtResponseDto, String> {
        match e.level() {
            ErrorLevel::Info | ErrorLevel::Warning => Ok(JwtResponseDto::Error {
                err_msg: self.map_service_error(e),
            }),
            _ => Err(self.map_service_error(e)),
        }
    }

    async fn get_user_by_jwt(&self, token: String) -> Result<ExtendedUser, ()> {
        let user_claims = self.token_provider.validate_access(&token)
            .map_err(|_| ())?;

        let user_id =  Uuid::from_str(&user_claims.sub)
            .map_err(|_| ())?;

        let user_data = self.query_user_service.get_user_by_id(user_id)
            .await.map_err(|_| ())?;

        let Some(user) = user_data.first() else {
            return Err(());
        };

        Ok(user.user().clone())
    }
    
}

