use crate::*;

/// Represents an HTTP response.
///
/// # Fields
/// - `version`: The HTTP version of the response.
/// - `status_code`: The status code of the response.
/// - `reason_phrase`: The reason phrase corresponding to the status code.
/// - `headers`: A collection of HTTP headers as key-value pairs.
/// - `body`: The binary body of the response.
#[derive(Debug, Clone, Data, DisplayDebug)]
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
