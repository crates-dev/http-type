use crate::*;

/// Represents various errors that can occur during HTTP response processing.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum ResponseError {
    /// Represents an unknown error with a message.
    #[default]
    Unknown,
    /// Represents an error where the stream was not found.
    NotFoundStream,
    /// Represents an error where the connection was closed.
    ConnectionClosed,
    /// Represents a terminated operation or connection.
    Terminated,
    /// Represents a send error with a message string.
    Send(String),
    /// Represents a flush operation error with a message string.
    FlushError(String),
}
