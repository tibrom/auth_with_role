use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use tokio_tungstenite::tungstenite::{http, Error as WsError};
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use http::Request;

use crate::domain::proxy::proxy_process::service::TransportEndpoint;
use crate::infrastructure::proxy::websocket::error::WebSocketError;

pub struct ServerEndpoint {
    pub server_write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub server_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl ServerEndpoint {
    pub async fn init(request: Request<()>) -> Result<Self, WsError> {
        let (server_ws_stream, _) = connect_async(request)
            .await
            .expect("Ошибка при подключении к Hasura");

        let (server_write, server_read) = server_ws_stream.split();

        Ok(Self {
            server_write,
            server_read,
        })
    }
}


impl TransportEndpoint for ServerEndpoint  {
    type Error = WebSocketError;
    type Message = Message;
    async fn receive(&mut self) -> Option<Result<Self::Message, Self::Error>> {
        let msg = self.server_read.next().await
            .map(|r| r.map_err(|e| WebSocketError::FailedReadData(e.to_string())));
        msg
    }
    async fn send(&mut self, msg: Self::Message) -> Result<(), Self::Error> {
        self.server_write.send(msg).await
            .map_err(|e| WebSocketError::FailedWriteData(e.to_string()))
    }
}
