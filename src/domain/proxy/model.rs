use std::collections::HashMap;
use uuid::Uuid;

use super::state::ConnectionState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Http,
    WebSocket,
}


pub type HeaderMap = HashMap<String, String>;


#[derive(getset::Getters, getset::Setters, Debug, Clone, PartialEq)]
pub struct ConnectionContext {
    #[get = "pub"]
    id: Uuid,
    #[get = "pub"]
    connection_type: ConnectionType,
    #[get = "pub"]
    target_url: String,
    #[get = "pub"]
    auth_data: AuthData,
    #[get = "pub"]
    state: ConnectionState,
}

impl ConnectionContext {
    pub fn new(
        target_url: String,
        connection_type: ConnectionType,
        client_headers: HeaderMap,
        auth_data: AuthData
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            connection_type,
            target_url,
            auth_data,
            state: ConnectionState::Initial
        }
    }

    pub fn new_state(&mut self, state: ConnectionState) {
        self.state = state
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthData {
    ApiKey(String)
}


