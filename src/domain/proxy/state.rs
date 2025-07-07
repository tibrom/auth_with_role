#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Initial,
    Authorization,
    ServerConnection,
    ProxyReady,
    Active,
    Error,
    Remove
}

impl ConnectionState {
    pub fn next(&self) -> Option<ConnectionState> {
        match self {
            ConnectionState::Initial => Some(ConnectionState::Authorization),
            ConnectionState::Authorization => Some(ConnectionState::ServerConnection),
            ConnectionState::ServerConnection => Some(ConnectionState::Remove),
            ConnectionState::Error => Some(ConnectionState::Remove),
            _ => None,
        }
    }
}



