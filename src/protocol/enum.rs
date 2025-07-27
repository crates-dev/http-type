/// Represents HTTP-related protocols.
///
/// This enum defines the different protocols that can be used in HTTP communication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    /// Represents the HTTP protocol.
    ///
    /// This is the standard protocol for unencrypted communication over the web.
    HTTP,
    /// Represents the HTTPS protocol.
    ///
    /// This is the secure version of HTTP, using encryption for communication.
    HTTPS,
    /// Represents an unknown or custom protocol.
    ///
    /// This variant is used for protocols that are not explicitly defined as HTTP or HTTPS,
    /// or when the protocol string is not recognized.
    Unknown(String),
}
