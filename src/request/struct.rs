use crate::*;

/// Configuration for HTTP request parsing security limits.
///
/// This struct defines various limits and constraints to prevent
/// denial-of-service attacks and other security vulnerabilities
/// when parsing HTTP requests.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Getter,
    GetterMut,
    Setter,
    DisplayDebug,
    Deserialize,
    Serialize,
)]
pub struct RequestConfig {
    /// Buffer size for reading operations.
    #[get(type(copy))]
    pub(super) buffer_size: usize,
    /// Maximum length for HTTP request line in bytes.
    #[get(type(copy))]
    pub(super) max_request_line_length: usize,
    /// Maximum length for URL path in bytes.
    #[get(type(copy))]
    pub(super) max_path_length: usize,
    /// Maximum length for query string in bytes.
    #[get(type(copy))]
    pub(super) max_query_length: usize,
    /// Maximum length for a single header line in bytes.
    #[get(type(copy))]
    pub(super) max_header_line_length: usize,
    /// Maximum number of headers allowed in a request.
    #[get(type(copy))]
    pub(super) max_header_count: usize,
    /// Maximum length for a header key in bytes.
    #[get(type(copy))]
    pub(super) max_header_key_length: usize,
    /// Maximum length for a header value in bytes.
    #[get(type(copy))]
    pub(super) max_header_value_length: usize,
    /// Maximum size for request body in bytes.
    #[get(type(copy))]
    pub(super) max_body_size: usize,
    /// Maximum size for WebSocket frame in bytes.
    #[get(type(copy))]
    pub(super) max_ws_frame_size: usize,
    /// Maximum number of WebSocket frames to process in a single request.
    #[get(type(copy))]
    pub(super) max_ws_frames: usize,
    /// Timeout for reading HTTP request in milliseconds.
    #[get(type(copy))]
    pub(super) http_read_timeout_ms: u64,
    /// Timeout for reading WebSocket frames in milliseconds.
    #[get(type(copy))]
    pub(super) ws_read_timeout_ms: u64,
}

/// HTTP request representation.
///
/// Contains all components of an HTTP request.
#[derive(Debug, Clone, PartialEq, Eq, Getter, DisplayDebug)]
pub struct Request {
    /// HTTP request method.
    pub(super) method: RequestMethod,
    /// Request host.
    pub(super) host: RequestHost,
    /// HTTP protocol version.
    pub(super) version: RequestVersion,
    /// Request path.
    pub(super) path: RequestPath,
    /// URL query parameters.
    pub(super) querys: RequestQuerys,
    /// HTTP headers collection.
    pub(super) headers: RequestHeaders,
    /// Request body content.
    pub(super) body: RequestBody,
}
