/// Represents different upgrade types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpgradeType {
    /// WebSocket protocol upgrade
    WebSocket,
    /// HTTP/2 cleartext upgrade (h2c)
    H2c,
    /// TLS upgrade (rare, experimental)
    Tls(String),
    /// Other custom or unknown upgrade protocols
    Unknown(String),
}
