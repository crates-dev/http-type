use super::error::Error as RequestError;
use crate::*;

/// RequestMethod
pub type RequestMethod = String;

/// RequestHost
pub type RequestHost = String;

/// RequestPath
pub type RequestPath = String;

/// RequestQuery
pub type RequestQuery = HashMap<String, String>;

///  RequestBody
pub type RequestBody = Vec<u8>;

/// RequestHeaders
pub type RequestHeaders = HashMap<String, String>;

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
    pub(crate) method: RequestMethod,
    #[set(skip)]
    pub(crate) host: RequestHost,
    #[set(skip)]
    pub(crate) path: RequestPath,
    #[set(skip)]
    pub(crate) query: RequestQuery,
    pub(crate) headers: RequestHeaders,
    #[set(skip)]
    pub(crate) body: RequestBody,
}
