use super::super::model::ConnectionContext;

pub trait ConnectionStorage {
    async fn send_context(&self, context: ConnectionContext);
    async fn delete_connection(&self, context: ConnectionContext);
}
