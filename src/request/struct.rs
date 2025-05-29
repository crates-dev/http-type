use crate::*;

/// Represents an HTTP request.
///
/// # Fields
/// - `method`: The HTTP method of the request.
/// - `host`: The host of the request.
/// - `version`: The version of the request.
/// - `path`: The path of the request.
/// - `querys`: The query string of the request.
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the request.
#[derive(Debug, Clone, Getter, DisplayDebug)]
pub struct Request {
    pub(super) method: RequestMethod,
    pub(super) host: RequestHost,
    pub(super) version: RequestVersion,
    pub(super) path: RequestPath,
    pub(super) querys: RequestQuerys,
    pub(super) headers: RequestHeaders,
    pub(super) body: RequestBody,
}
