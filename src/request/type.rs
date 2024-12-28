use super::error::Error as RequestError;
use lombok_macros::*;
use std::{borrow::Cow, collections::HashMap};

/// RequestMethod
pub type RequestMethod = Cow<'static, str>;

/// RequestHost
pub type RequestHost = Cow<'static, str>;

/// RequestPath
pub type RequestPath = Cow<'static, str>;

/// RequestQuery
pub type RequestQuery = Cow<'static, str>;

/// RequestHash
pub type RequestHash = Cow<'static, str>;

///  RequestBody
pub type RequestBody = Vec<u8>;

/// RequestHeaders
pub type RequestHeaders = HashMap<Cow<'static, str>, Cow<'static, str>>;

/// RequestNewResult
pub type RequestNewResult = Result<Request, RequestError>;

/// Represents an HTTP request.
///
/// # Fields
/// - `method`: The HTTP method of the request (e.g., GET, POST).
/// - `host`: The host of the request (e.g., example.com).
/// - `path`: The path of the request (e.g., /api/v1/resource).
/// - `query`: The query string of the request (e.g., ?key=value).
/// - `hash`: The fragment identifier of the request (e.g., #section).
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the request.
#[derive(Debug, Clone, Lombok)]
pub struct Request {
    pub(crate) method: RequestMethod,
    pub(crate) host: RequestHost,
    pub(crate) path: RequestPath,
    pub(crate) query: RequestQuery,
    pub(crate) hash: RequestHash,
    pub(crate) headers: RequestHeaders,
    pub(crate) body: RequestBody,
}
