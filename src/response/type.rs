use lombok_macros::*;
use std::{borrow::Cow, collections::HashMap};

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
    pub(super) version: Cow<'static, str>,
    pub(super) status_code: usize,
    pub(super) reason_phrase: Cow<'static, str>,
    pub(super) headers: HashMap<Cow<'static, str>, Cow<'static, str>>,
    pub(super) body: Vec<u8>,
    pub(super) response: Vec<u8>,
}
