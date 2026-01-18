use crate::*;

/// Represents a parsed HTTP response.
#[derive(Debug, Clone, PartialEq, Eq, Getter, GetterMut, Setter, DisplayDebug)]
pub struct Response {
    /// The HTTP version used in the response.
    pub(super) version: ResponseVersion,
    /// The HTTP status code.
    #[get(type(copy))]
    pub(super) status_code: ResponseStatusCode,
    /// The reason phrase associated with the status code.
    #[set(type(AsRef<str>))]
    pub(super) reason_phrase: ResponseReasonPhrase,
    /// The response headers as key-value pairs.
    #[set(pub(super))]
    pub(super) headers: ResponseHeaders,
    /// The binary body content of the response.
    #[set(type(AsRef<[u8]>))]
    pub(super) body: ResponseBody,
}
