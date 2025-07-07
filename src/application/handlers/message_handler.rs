use uuid::Uuid;

use crate::application::error_dto::ComponentErrorDTO;
use crate::domain::errors::service::AppErrorInfo;
use crate::domain::jwt::factories::JWTProviderFactory;
use crate::domain::proxy::message::ProcessMessage;
use crate::domain::proxy::model::{AuthData, ConnectionContext};
use crate::domain::proxy::state::{self, ConnectionState};
use crate::domain::user::factories::UserProviderFactory;
use crate::domain::verifies::factories::VerifiesProviderFactory;

use crate::domain::proxy::proxy_controller::service::ConnectionStorage;

use crate::application::usecase::proxy_usecase::authorize::AuthorizeUseCase;
use super::error::UpdateStateError;


pub struct MessageHandler<J, V, U, C>  {
    jwtprovider_factory: J,
    verifies_provider_factory: V,
    user_provider_factory: U,
    connection_controller: C
    
}

impl <J, V, U, C>MessageHandler<J, V, U, C>
where
    J: JWTProviderFactory,
    V: VerifiesProviderFactory,
    U: UserProviderFactory,
    C: ConnectionStorage {
    
    pub fn new(jwtprovider_factory: J, verifies_provider_factory: V, user_provider_factory: U, connection_controller: C) -> Self {
        Self { jwtprovider_factory, verifies_provider_factory, user_provider_factory, connection_controller }
    }

    pub async fn execute(&self, message: ProcessMessage) {
        let ProcessMessage::Context(context) = message else {
            return ;
        };
        let mut next_state = context.state().next();
        let mut new_context = match self.handle_context(*context.clone()).await {
            Ok(v) => v,
            Err(e) => {
                next_state = Some(ConnectionState::Error);
                *context.clone()
            }
        };
        if let Some(state) = next_state {
            new_context.set_state(state);
            self.connection_controller.send_context(new_context);

        };
    }

    async fn handle_context(&self, mut context: ConnectionContext) -> Result<ConnectionContext, UpdateStateError> {

        match context.state() {
            ConnectionState::Authorization => {
                let token = self.authorization(context.auth_data().clone()).await
                    .map_err(|e| self.infrastructure_error(&e))?;
                context.set_access_token(token);
                    
            },
            ConnectionState::Remove => {
                //TODO логика удаления ручки просцесса
                
            },
            _ => {}
        }
        Ok(context)
    }

    async fn authorization(&self, auth_data: AuthData) ->  Result<String, UpdateStateError>{
        let authorize_use_case = AuthorizeUseCase::new(
            &self.user_provider_factory,
            &self.verifies_provider_factory,
            &self.jwtprovider_factory
        );
        authorize_use_case.execute(auth_data).await
            .map_err(|e| self.infrastructure_error(&e))
    }

    fn infrastructure_error(&self, e: &dyn AppErrorInfo) -> UpdateStateError {
        UpdateStateError::InfrastructureError(ComponentErrorDTO::new(e.level(), e.log_message(), e.client_message()))
    }
    
}