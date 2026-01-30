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
pub struct RequestConfigData {
    /// Buffer size for reading operations.
    #[get(type(copy))]
    pub(super) buffer_size: usize,
    /// Maximum size for HTTP request line in bytes.
    #[get(type(copy))]
    pub(super) max_request_line_size: usize,
    /// Maximum size for URL path in bytes.
    #[get(type(copy))]
    pub(super) max_path_size: usize,
    /// Maximum size for query string in bytes.
    #[get(type(copy))]
    pub(super) max_query_size: usize,
    /// Maximum size for a single header line in bytes.
    #[get(type(copy))]
    pub(super) max_header_line_size: usize,
    /// Maximum number of headers allowed in a request.
    #[get(type(copy))]
    pub(super) max_header_count: usize,
    /// Maximum size for a header key in bytes.
    #[get(type(copy))]
    pub(super) max_header_key_size: usize,
    /// Maximum size for a header value in bytes.
    #[get(type(copy))]
    pub(super) max_header_value_size: usize,
    /// Maximum size for request body in bytes.
    #[get(type(copy))]
    pub(super) max_body_size: usize,
    /// Maximum size for WebSocket frame in bytes.
    #[get(type(copy))]
    pub(super) max_ws_frame_size: usize,
    /// Maximum number of WebSocket frames to process in a single request.
    #[get(type(copy))]
    pub(super) max_ws_frames_count: usize,
    /// Timeout for reading HTTP request in milliseconds.
    #[get(type(copy))]
    pub(super) http_read_timeout_ms: u64,
    /// Timeout for reading WebSocket frames in milliseconds.
    #[get(type(copy))]
    pub(super) ws_read_timeout_ms: u64,
}

/// Thread-safe configuration wrapper for HTTP request parsing.
///
/// This struct uses `ArcRwLock` to provide thread-safe access to `RequestConfigData `,
/// allowing concurrent reads and exclusive writes. It is the public-facing API
/// for configuring HTTP request parsing limits.
#[derive(Clone, Getter, CustomDebug, DisplayDebug)]
pub struct RequestConfig(#[get(pub(super))] pub(super) ArcRwLock<RequestConfigData>);

/// HTTP request representation.
///
/// Contains all components of an HTTP request.
#[derive(Debug, Clone, Eq, PartialEq, Getter, DisplayDebug)]
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
