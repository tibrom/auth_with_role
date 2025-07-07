use tokio::{sync::mpsc, task::JoinHandle};

use crate::application::handlers::message_handler::MessageHandler;
use crate::domain::proxy::message::ProcessMessage;
use crate::domain::settings::model::Credentials;

use crate::infrastructure::verifies::factory::VerifiesProvider;
use crate::infrastructure::jwt::factory::JWTProvider;
use crate::infrastructure::user::factory::UserProvider;

use crate::infrastructure::proxy::connection_controller::ConnectionController;



const BUFFER: usize = 100;

pub struct MessageController{
    pub address: mpsc::Sender<ProcessMessage>,
    pub handle: JoinHandle<()>,


}

impl MessageController {
    pub fn init(credentials: Credentials) -> Self {
        let message_handler = MessageHandler::new(
            JWTProvider::new(credentials.clone()),
            VerifiesProvider::new(credentials.clone()),
            UserProvider::new(credentials),
            ConnectionController,
        );
        let (tx, mut rx): (mpsc::Sender<ProcessMessage>, mpsc::Receiver<ProcessMessage>) = mpsc::channel(BUFFER);

        let handle = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                message_handler.execute(msg).await;
            }
        });

        Self {
            address: tx,
            handle,
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<ProcessMessage> {
        self.address.clone()
    }
}