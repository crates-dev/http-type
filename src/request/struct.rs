use crate::*;

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
