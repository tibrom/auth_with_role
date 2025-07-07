use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;

use super::state::ConnectionState;

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionType {
    Http,
    WebSocket,
    Unknown,
}

#[derive(getset::Getters, getset::Setters, Debug, Clone, PartialEq)]
pub struct ConnectionContext {
    #[get = "pub"]
    id: Uuid,
    #[get = "pub"]
    auth_data: AuthData,
    #[get = "pub"]
    state: ConnectionState,
    #[get = "pub"]
    access_token: Option<String>,
    #[get = "pub"]
    connection_type: ConnectionType
}

impl ConnectionContext {
    pub fn new(
        auth_data: AuthData,
        connection_type: ConnectionType
    ) -> Self {
        let id = Uuid::new_v4();
        Self {
            id: id.clone(),
            auth_data,
            state: ConnectionState::Initial,
            access_token: None,
            connection_type
        }
        }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state
    }
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthData {
    ApiKey(String),
    None
}



