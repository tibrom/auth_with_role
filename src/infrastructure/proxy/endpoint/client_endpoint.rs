use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;



use crate::domain::proxy::proxy_process::service::TransportEndpoint;
use crate::infrastructure::proxy::websocket::error::WebSocketError;



pub struct EndpointClient{
    client_write: SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
    client_read: SplitStream<WebSocketStream<TcpStream>>,
}
impl EndpointClient {
    
    pub async fn init(stream: TcpStream) -> Result<Self, WebSocketError> {
        let client_ws_stream = accept_async(stream)
            .await
            .map_err(|e| WebSocketError::InitWebSocket(e.to_string()))?;
        println!("Новое WebSocket соединение с клиентом установлено");
        let (client_write,
            client_read) = client_ws_stream.split();
        Ok(Self { client_write, client_read })
    }
}

impl TransportEndpoint for EndpointClient  {
    type Error = WebSocketError;
    type Message = Message;
    async fn receive(&mut self) -> Option<Result<Self::Message, Self::Error>> {
        let msg = self.client_read.next().await
            .map(|r| r.map_err(|e| WebSocketError::FailedReadData(e.to_string())));
        msg
    }
    async fn send(&mut self, msg: Self::Message) -> Result<(), Self::Error> {
        self.client_write.send(msg).await
            .map_err(|e| WebSocketError::FailedWriteData(e.to_string()))
    }
}