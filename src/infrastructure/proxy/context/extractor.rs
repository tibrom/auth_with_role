use tokio::net::TcpStream;

use crate::domain::proxy::model::{ConnectionContext, AuthData, ConnectionType};

pub struct ConnectionContextExtractor;

impl ConnectionContextExtractor {
    async fn init(stream: &TcpStream) -> ConnectionContext {
        let mut headers: Vec<(String, String)> = Vec::new();
        let mut buffer = [0; 1024];
        if let Ok(bytes_read) = stream.peek(&mut buffer).await {
            if bytes_read > 0 {
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                headers = Self::look_headers(&request);
            }
        };
        let auth_data = Self::get_auth_data(&headers);
        let connection_type =Self::detect_connection_type(&headers);
        ConnectionContext::new(auth_data, connection_type)
    }


    fn detect_connection_type(headers: &[(String, String)]) -> ConnectionType {
        let mut has_upgrade_websocket = false;
        let mut has_connection_upgrade = false;

        for (key, value) in headers {
            match key.to_ascii_lowercase().as_str() {
                "upgrade" if value.to_ascii_lowercase() == "websocket" => {
                    has_upgrade_websocket = true;
                }
                "connection" if value.to_ascii_lowercase().contains("upgrade") => {
                    has_connection_upgrade = true;
                }
                _ => {}
            }
        }

        if has_upgrade_websocket && has_connection_upgrade {
            ConnectionType::WebSocket
        } else {
            ConnectionType::Http
        }
    }
    fn get_auth_data(headers: &[(String, String)]) -> AuthData {
        for (key, value) in headers {
            match key.as_str() {
                "api_key" => return AuthData::ApiKey(value.clone()),
                _ => continue,
            }
        }
        AuthData::None
    }
    fn look_headers(request: &str) -> Vec<(String, String)> {
        let mut headers: Vec<(String, String)> = Vec::new();

        let lines: Vec<&str> = request.lines().collect();

        if !lines.is_empty() && (lines[0].starts_with("GET") || lines[0].starts_with("POST")) {
            for line in &lines[1..] {
                if line.is_empty() {
                    break; // Конец заголовков
                }
                if let Some((key, value)) = line.split_once(": ") {
                    headers.push((key.to_string(), value.to_string()));
                }
            }
        }
        headers
    }
}