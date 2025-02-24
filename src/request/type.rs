use super::error::Error as RequestError;
use crate::*;

/// RequestMethod
pub type RequestMethod = String;

/// RequestHost
pub type RequestHost = String;

/// RequestPath
pub type RequestPath = String;

/// RequestQuery key
pub type RequestQueryKey = String;

/// RequestQuery value
pub type RequestQueryValue = String;

/// RequestQuery
pub type RequestQuery = HashMap<RequestQueryKey, RequestQueryValue>;

///  RequestBody
pub type RequestBody = Vec<u8>;

/// RequestHeaders key
pub type RequestHeadersKey = String;

/// RequestHeaders value
pub type RequestHeadersValue = String;

/// RequestHeaders
pub type RequestHeaders = HashMap<RequestHeadersKey, RequestHeadersValue>;

/// RequestNewResult
pub type RequestNewResult = Result<Request, RequestError>;

/// Represents an HTTP request.
///
/// # Fields
/// - `method`: The HTTP method of the request (e.g., GET, POST).
/// - `host`: The host of the request (e.g., example.com).
/// - `path`: The path of the request (e.g., /api/v1/resource).
/// - `query`: The query string of the request (e.g., ?key=value).
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the request.
#[derive(Debug, Clone, Lombok, PartialEq, Eq)]
pub struct Request {
    #[set(skip)]
    pub(super) method: RequestMethod,
    #[set(skip)]
    pub(super) host: RequestHost,
    #[set(skip)]
    pub(super) path: RequestPath,
    #[set(skip)]
    pub(super) query: RequestQuery,
    pub(super) headers: RequestHeaders,
    #[set(skip)]
    pub(super) body: RequestBody,
}
