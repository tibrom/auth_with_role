use thiserror::Error;

/// Основная ошибка GraphQL клиента (обёртка)
#[derive(Debug, Error)]

pub enum WebSocketError {
    #[error("Failed init new socket {0}")]
    InitWebSocket(String),
    #[error("Failed read socket data{0}")]
    FailedReadData(String),
    #[error("Failed write data to socket {0}")]
    FailedWriteData(String)

}
