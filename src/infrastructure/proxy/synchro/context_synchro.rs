use tokio::sync::mpsc;

use crate::domain::proxy::message::ProcessMessage;
use crate::domain::proxy::proxy_process::service::ContextSynchro;
use crate::domain::proxy::model::ConnectionContext;

pub struct Synchronizer{
    pub address: mpsc::Sender<ProcessMessage>,
}

impl Synchronizer {
    pub fn init(address: mpsc::Sender<ProcessMessage>) -> Self {
        Self { address }
    }
}

impl ContextSynchro for Synchronizer {
    async fn push_context(&self, context: ConnectionContext) {
        let r =self.address.send(ProcessMessage::Context(Box::new(context))).await;
    }
}