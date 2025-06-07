/// Defines the `Protocol` enum, representing HTTP-related protocols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    /// Represents the HTTP protocol.
    HTTP,
    /// Represents the HTTPS protocol.
    HTTPS,
    /// Unknown
    Unknown(String),
}
