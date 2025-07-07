use crate::domain::proxy::message::ProcessMessage;
use super::super::model::ConnectionContext;
use std::fmt::Debug;

//обертка над подключением
pub trait TransportEndpoint {
    type Message: Send + 'static;
    type Error: Debug;

    async fn send(&mut self, msg: Self::Message) -> Result<(), Self::Error>;
    
    async fn receive(&mut self) -> Option<Result<Self::Message, Self::Error>>;
}

//Создает подключение к hasura для прокси сервера
pub trait ServerConnector {
    type Message: Send + 'static;
    type Error;
    type Endpoint: TransportEndpoint<Message = Self::Message, Error = Self::Error>;

    async fn connect(&mut self, access_token: String) -> Result<
        Self::Endpoint,
        Self::Error,
    >;
}
//слушает сообщения от контроллера внутри прокси сопроцесса
pub trait CommandListener {
    async fn receive(&mut self) -> Option<ProcessMessage>;
}
//отправляет сообщения в контроллер внутри прокси сопроцесса
pub trait ContextSynchro {
    async fn push_context(&self, context: ConnectionContext);
}