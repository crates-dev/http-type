use crate::*;

/// Represents a parsed HTTP request.
#[derive(Debug, Clone, PartialEq, Eq, Getter, DisplayDebug)]
pub struct Request {
    /// The HTTP method of the request.
    pub(super) method: RequestMethod,
    /// The host of the request.
    pub(super) host: RequestHost,
    /// The HTTP version used in the request.
    pub(super) version: RequestVersion,
    /// The request path.
    pub(super) path: RequestPath,
    /// The query string of the request.
    pub(super) querys: RequestQuerys,
    /// A collection of HTTP headers as key-value pairs.
    pub(super) headers: RequestHeaders,
    /// The binary body of the request.
    pub(super) body: RequestBody,
}
