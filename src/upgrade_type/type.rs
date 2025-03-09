/// Represents different upgrade types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpgradeType {
    Http,
    WebSocket,
    Unknown(String),
}
