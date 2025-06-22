use uuid::Uuid;

use super::model::ConnectionContext;
use super::state::ConnectionState;

pub trait ConnectionManager {
    type Error;
    fn save(&mut self, connection_context: ConnectionContext) -> Result<Uuid, Self::Error>;
    fn set_state(&mut self, state: ConnectionState, connection_id: Uuid) -> Result<Uuid, Self::Error>;
    fn remove(&self, connection_id: Uuid) -> Result<Uuid, Self::Error>;
}


