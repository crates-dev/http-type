use std::{borrow::Cow, collections::HashMap};

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
#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub(crate) method: Cow<'a, str>,
    pub(crate) host: Cow<'a, str>,
    pub(crate) path: Cow<'a, str>,
    pub(crate) query: Cow<'a, str>,
    pub(crate) hash: Cow<'a, str>,
    pub(crate) headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub(crate) body: Vec<u8>,
}
