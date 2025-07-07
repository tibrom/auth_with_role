use std::collections::HashMap;

use lazy_static::lazy_static;
use tokio::{sync::{mpsc, RwLock}, task::JoinHandle};
use uuid::Uuid;


use crate::domain::proxy::proxy_controller::service::ConnectionStorage;
use crate::domain::proxy::message::ProcessMessage;

use crate::domain::proxy::model::ConnectionContext;



lazy_static! {
    static ref CONNECTION_STORAGE: RwLock<HashMap<Uuid, Connection>> = RwLock::new(HashMap::new());
}


struct Connection {
    id: Uuid,
    handle: JoinHandle<()>,
    address: mpsc::Sender<ProcessMessage>,
}




pub struct ConnectionController;

impl ConnectionController {
    fn new() -> Self {
        Self{}
    }

    pub async fn add_connection(connection: Connection) {
        let mut storage = CONNECTION_STORAGE.write().await;
        storage.insert(connection.id.clone(), connection);
    }
    
}

impl ConnectionStorage for ConnectionController {
    async fn send_context(&self, context: ConnectionContext) {
        let storage =CONNECTION_STORAGE.read().await;
        let Some(connection) = storage.get(context.id()) else {
            return ;
        };
        if let Err(result) = connection.address
            .send(ProcessMessage::Context(Box::new(context.clone()))).await {
            connection.handle.abort();
            let mut storage = CONNECTION_STORAGE.write().await;
            storage.remove(context.id());
            
        }
    }
    async fn delete_connection(&self, context: ConnectionContext) {
        let mut storage = CONNECTION_STORAGE.write().await;
        if let Some(connection) = storage.remove(context.id()) {
            connection.handle.abort(); // Прерываем задачу, если была
        }
    }
}