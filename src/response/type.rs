use super::error::Error as ResponseError;
use lombok_macros::*;
use std::collections::HashMap;

///  ResponseData
pub type ResponseData = Vec<u8>;

///  ResponseBody
pub type ResponseBody = Vec<u8>;

///  ResponseResult
pub type ResponseResult = Result<ResponseData, ResponseError>;

/// Represents an HTTP response.
///
/// # Fields
/// - `version`: The HTTP version of the response (e.g., HTTP/1.1).
/// - `status_code`: The status code of the response (e.g., 200, 404).
/// - `reason_phrase`: The reason phrase corresponding to the status code (e.g., OK, Not Found).
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the response.
/// - `response`: The serialized HTTP response including headers and body.
#[derive(Debug, Clone, Lombok)]
pub struct Response {
    pub(super) version: String,
    pub(super) status_code: usize,
    pub(super) reason_phrase: String,
    pub(super) headers: HashMap<String, String>,
    pub(super) body: ResponseBody,
    pub(super) response: ResponseData,
}
