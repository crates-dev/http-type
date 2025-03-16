use super::error::Error as ResponseError;
use crate::*;

///  Response body
pub type ResponseBody = Vec<u8>;
///  Response headers key
pub type ResponseHeadersKey = String;
///  Response headers value
pub type ResponseHeadersValue = String;
///  Response headers
pub type ResponseHeaders = HashMap<ResponseHeadersKey, ResponseHeadersValue>;
/// Response version
pub type ResponseVersion = String;
/// Response status code
pub type ResponseStatusCode = usize;
/// Response reason phrase
pub type ResponseReasonPhrase = String;
///  Response result
pub type ResponseResult = Result<(), ResponseError>;

/// Represents an HTTP response.
///
/// # Fields
/// - `version`: The HTTP version of the response (e.g., HTTP/1.1).
/// - `status_code`: The status code of the response (e.g., 200, 404).
/// - `reason_phrase`: The reason phrase corresponding to the status code (e.g., OK, Not Found).
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the response.
#[derive(Debug, Clone, Lombok, PartialEq, Eq, DisplayDebug)]
pub struct Response {
    #[set(skip)]
    pub(super) version: ResponseVersion,
    pub(super) status_code: ResponseStatusCode,
    #[set(skip)]
    pub(super) reason_phrase: ResponseReasonPhrase,
    pub(super) headers: ResponseHeaders,
    #[set(skip)]
    pub(super) body: ResponseBody,
}
