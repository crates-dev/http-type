use crate::*;

/// Implements the `StdError` trait for `ResponseError`.
/// This allows `ResponseError` to be treated as a standard Rust error type.
impl StdError for ResponseError {}

/// Implements the `Display` trait for `ResponseError`.
/// This allows `ResponseError` variants to be formatted into human-readable strings.
impl Display for ResponseError {
    /// Formats the `ResponseError` variant into a human-readable string.
    ///
    /// # Arguments
    ///
    /// - `f`: A mutable reference to a `fmt::Formatter` used for writing the formatted string.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating whether the formatting was successful.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundStream => {
                write!(f, "Not found stream")
            }
            Self::ConnectionClosed => {
                write!(f, "Connection has been closed")
            }
            Self::Terminated => {
                write!(f, "Current processing has been terminated")
            }
            Self::Unknown(err) => write!(f, "Unknown error{}{}", COLON_SPACE, err),
            Self::Response(data) => write!(f, "Response error{}{}", COLON_SPACE, data),
            Self::WebSocketHandShake(err) => {
                write!(f, "Websocket handshake error{}{}", COLON_SPACE, err)
            }
            Self::MethodNotSupported(err) => {
                write!(f, "Method not supported{}{}", COLON_SPACE, err)
            }
        }
    }
}
