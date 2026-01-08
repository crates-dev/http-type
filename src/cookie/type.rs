use crate::*;

/// Represents the raw cookie string from an HTTP request header.
///
/// Contains the complete cookie header value as received from the client.
pub type CookieString<'a> = &'a str;

/// Represents the key/name of an HTTP cookie.
///
/// Used to identify individual cookies in requests and responses.
pub type CookieKey<'a> = &'a str;

/// Represents the value/content of an HTTP cookie.
///
/// Stores the actual data associated with a cookie name/key.
pub type CookieValue<'a> = &'a str;

/// Represents a collection of HTTP cookies.
///
/// Stores multiple cookies as key-value pairs using a high-performance hash map.
pub type Cookies = HashMapXxHash3_64<String, String>;
