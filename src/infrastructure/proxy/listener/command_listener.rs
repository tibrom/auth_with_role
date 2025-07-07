use tokio::sync::mpsc;

use crate::domain::proxy::message::ProcessMessage;
use crate::domain::proxy::proxy_process::service::CommandListener;


const BUFFER: usize = 10;

pub struct Listener {
    sender: mpsc::Sender<ProcessMessage>, 
    receiver: mpsc::Receiver<ProcessMessage>
}

impl Listener {
    pub fn init() -> Self {
        let (tx, rx): (mpsc::Sender<ProcessMessage>, mpsc::Receiver<ProcessMessage>) = mpsc::channel(BUFFER);
        Self { sender: tx, receiver: rx }
    }

    pub fn get_sender(&self) -> mpsc::Sender<ProcessMessage> {
        self.sender.clone()
    }

}

impl CommandListener for Listener {
    async fn receive(&mut self) -> Option<ProcessMessage> {
       self.receiver.recv().await
    }
}