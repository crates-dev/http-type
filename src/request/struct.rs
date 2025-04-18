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
#[derive(Debug, Clone, Lombok, DisplayDebug)]
pub struct Request {
    #[set(skip)]
    pub(super) method: RequestMethod,
    #[set(skip)]
    pub(super) host: RequestHost,
    #[set(skip)]
    pub(super) version: RequestVersion,
    #[set(skip)]
    pub(super) path: RequestPath,
    pub(super) querys: RequestQuerys,
    pub(super) headers: RequestHeaders,
    #[set(skip)]
    pub(super) body: RequestBody,
}
