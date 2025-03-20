use crate::*;

/// Request method
pub type RequestMethod = Methods;
/// Request host
pub type RequestHost = String;
/// Request version
pub type RequestVersion = HttpVersion;
/// Request path
pub type RequestPath = String;
/// Request querys key
pub type RequestQuerysKey = String;
/// Request querys value
pub type RequestQuerysValue = String;
/// Request querys
pub type RequestQuerys = HashMap<RequestQuerysKey, RequestQuerysValue>;
///  Request body
pub type RequestBody = Vec<u8>;
///  Request body string
pub type RequestBodyString = String;
/// Request headers key
pub type RequestHeadersKey = String;
/// Request headers value
pub type RequestHeadersValue = String;
/// Request headers
pub type RequestHeaders = HashMap<RequestHeadersKey, RequestHeadersValue>;
/// Request new result
pub type RequestNewResult = Result<Request, RequestError>;

/// Represents an HTTP request.
///
/// # Fields
/// - `method`: The HTTP method of the request (e.g., GET, POST).
/// - `host`: The host of the request (e.g., example.com).
/// - `version`: The version of the request (e.g., HTTP/1.1).
/// - `path`: The path of the request (e.g., /api/v1/resource).
/// - `query`: The query string of the request (e.g., ?key=value).
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the request.
#[derive(Debug, Clone, Lombok, PartialEq, Eq, DisplayDebug)]
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
