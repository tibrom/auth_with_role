use super::model::ConnectionContext;


#[derive(Debug, Clone)]
pub enum ProcessMessage {
    Context(Box<ConnectionContext>)
}
