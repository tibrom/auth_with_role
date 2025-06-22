use crate::domain::proxy::model::ConnectionContext;
use crate::domain::verifies::service::ApiKeyVerifierService;

pub struct AuthorizeProxyByApiKeyUseCase  {}

impl AuthorizeProxyByApiKeyUseCase {
    pub fn new() -> Self {
        Self { }
    }

    pub fn execute (&self, connection_context: ConnectionContext) -> Result<(), ()>{
        Ok(())
    }
}