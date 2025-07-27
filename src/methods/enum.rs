/// Defines the `Method` enum, representing HTTP request methods.
///
/// This enum provides a comprehensive list of standard HTTP methods,
/// such as GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT, and TRACE.
/// It also includes an `UNKNOWN` variant for unrecognized methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Method {
    /// Represents the HTTP `GET` method.
    GET,
    /// Represents the HTTP `POST` method.
    POST,
    /// Represents the HTTP `PUT` method.
    PUT,
    /// Represents the HTTP `DELETE` method.
    DELETE,
    /// Represents the HTTP `PATCH` method.
    PATCH,
    /// Represents the HTTP `HEAD` method.
    HEAD,
    /// Represents the HTTP `OPTIONS` method.
    OPTIONS,
    /// Represents the HTTP `CONNECT` method.
    CONNECT,
    /// Represents the HTTP `TRACE` method.
    TRACE,
    /// Unknown
    UNKNOWN(String),
}
