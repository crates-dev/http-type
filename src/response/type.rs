use super::error::Error as ResponseError;
use lombok_macros::*;
use std::collections::HashMap;

///  Response data
pub type ResponseData = Vec<u8>;

///  Response body
pub type ResponseBody = Vec<u8>;

/// Response version
pub type ResponseVersion = String;

/// Response status code
pub type ResponseStatusCode = usize;

/// Response reason phrase
pub type ResponseReasonPhrase = String;

///  Response result
pub type ResponseResult = Result<(), ResponseError>;

///  Close stream result
pub type CloseStreamResult = Result<(), ResponseError>;

/// Represents an HTTP response.
///
/// # Fields
/// - `version`: The HTTP version of the response (e.g., HTTP/1.1).
/// - `status_code`: The status code of the response (e.g., 200, 404).
/// - `reason_phrase`: The reason phrase corresponding to the status code (e.g., OK, Not Found).
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the response.
/// - `response`: The serialized HTTP response including headers and body.
#[derive(Debug, Clone, Lombok, PartialEq, Eq)]
pub struct Response {
    #[set(skip)]
    pub(super) version: ResponseVersion,
    pub(super) status_code: ResponseStatusCode,
    #[set(skip)]
    pub(super) reason_phrase: ResponseReasonPhrase,
    pub(super) headers: HashMap<String, String>,
    #[set(skip)]
    pub(super) body: ResponseBody,
    #[set(super)]
    pub(super) response: ResponseData,
}
