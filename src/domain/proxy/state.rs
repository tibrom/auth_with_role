#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Initial,
    Active,
    Terminating,
}
