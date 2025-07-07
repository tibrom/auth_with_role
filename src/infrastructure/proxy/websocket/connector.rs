use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header::AUTHORIZATION;

use crate::domain::proxy::proxy_process::service::ServerConnector;
use crate::domain::settings::model::Credentials;
use crate::infrastructure::proxy::websocket::error::WebSocketError;

use super::endpoint::ServerEndpoint;


pub struct Connector {
    credentials: Credentials
}

impl Connector {
    fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl ServerConnector for Connector {
    type Message = Message;
    type Endpoint = ServerEndpoint;
    type Error = WebSocketError;
    async fn connect(&mut self, access_token: String) -> Result<Self::Endpoint, Self::Error> {
        let service_url = self.credentials.hasura_ws_url().clone();

        let mut request = service_url.into_client_request()
            .map_err(|e| WebSocketError::InitWebSocket(e.to_string()))?;

        request.headers_mut().insert("Sec-WebSocket-Protocol", "graphql-ws".parse().unwrap());
        request.headers_mut().insert(AUTHORIZATION, access_token.parse().unwrap());

        let endpoint_server = ServerEndpoint::init(request).await
            .map_err(|e| WebSocketError::InitWebSocket(e.to_string()))?;
        Ok(endpoint_server)
    }
}
