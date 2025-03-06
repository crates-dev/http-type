/// Represents the HTTP version used in the request or response.
///
/// This enum is used to specify the HTTP version for HTTP requests and responses.
/// It supports the most common HTTP versions: HTTP/0.9, HTTP/1.0, HTTP/1.1, HTTP/2, HTTP/3, and unknown versions.
/// The `HttpVersion` enum allows for easy comparison, cloning, and debugging of the HTTP version.
///
/// The variants include:
/// - `HTTP0_9`: Represents HTTP version 0.9.
/// - `HTTP1_0`: Represents HTTP version 1.0.
/// - `HTTP1_1`: Represents HTTP version 1.1.
/// - `HTTP2`: Represents HTTP version 2.0.
/// - `HTTP3`: Represents HTTP version 3.0.
/// - `Unknown`: Represents an unknown or unsupported HTTP version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpVersion {
    /// HTTP version 0.9
    HTTP0_9,
    /// HTTP version 1.0
    HTTP1_0,
    /// HTTP version 1.1
    HTTP1_1,
    /// HTTP version 2.0
    HTTP2,
    /// HTTP version 3.0
    HTTP3,
    /// Unknown version
    Unknown(String),
}
