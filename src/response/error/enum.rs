use crate::*;

/// Represents various errors that can occur during HTTP response processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseError {
    /// Represents an error where the stream was not found.
    NotFoundStream,
    /// Represents an error where the connection was closed.
    ConnectionClosed,
    /// Represents a terminated operation or connection.
    Terminated,
    /// Represents an unknown error with a message.
    Unknown(String),
    /// Represents a generic response error with a message.
    Response(String),
    /// Represents an error during WebSocket handshake with a message.
    WebSocketHandShake(String),
    /// Represents an error for an unsupported HTTP method with a message.
    MethodNotSupported(String),
}
