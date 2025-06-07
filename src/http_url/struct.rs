use crate::*;

/// Represents the components of a parsed HTTP URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpUrlComponents {
    /// The URL scheme, such as "http" or "https".
    pub protocol: Protocol,
    /// The host part of the URL (e.g., "example.com").
    pub host: OptionString,
    /// The port number in the URL, if specified.
    pub port: OptionU16,
    /// The path in the URL (e.g., "/index.html").
    pub path: OptionString,
    /// The query string in the URL (e.g., "?a=1").
    pub query: OptionString,
    /// The fragment identifier (e.g., "#section").
    pub fragment: OptionString,
}
