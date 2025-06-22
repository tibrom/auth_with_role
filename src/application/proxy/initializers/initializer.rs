use crate::domain::proxy::model::ConnectionContext;
use crate::domain::verifies::service::ApiKeyVerifierService;
use crate::domain::proxy::connection_manager::ConnectionManager;

pub struct ConnectionInitializerUseCase<C>{
    connection_manager: C
}

impl <C>ConnectionInitializerUseCase<C> 
    where
        C: ConnectionManager {

    fn initial(&mut self, connection_context: &ConnectionContext) -> Result<(), String> {
        self.connection_manager.save(connection_context.clone())
            .map_err(|e| "failed save new connection")?; //TODO доделать с проверкой результата
        Ok(())
    }
}