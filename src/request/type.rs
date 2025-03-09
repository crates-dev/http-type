use super::error::Error as RequestError;
use crate::*;

/// RequestMethod
pub type RequestMethod = Methods;
/// RequestHost
pub type RequestHost = String;
/// RequestVersion
pub type RequestVersion = HttpVersion;
/// RequestPath
pub type RequestPath = String;
/// RequestQuerys key
pub type RequestQuerysKey = String;
/// RequestQuerys value
pub type RequestQuerysValue = String;
/// RequestQuerys
pub type RequestQuerys = HashMap<RequestQuerysKey, RequestQuerysValue>;
///  RequestBody
pub type RequestBody = Vec<u8>;
/// RequestUpgradeType
pub type RequestUpgradeType = UpgradeType;
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
/// - `version`: The version of the request (e.g., HTTP/1.1).
/// - `path`: The path of the request (e.g., /api/v1/resource).
/// - `query`: The query string of the request (e.g., ?key=value).
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the request.
/// - `upgrade_type`: Upgrade type of the request.
#[derive(Debug, Clone, Lombok, PartialEq, Eq)]
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
    #[set(skip)]
    pub(super) upgrade_type: RequestUpgradeType,
}
