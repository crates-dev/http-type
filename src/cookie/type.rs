use crate::*;

/// Represents the raw cookie string from an HTTP request header.
///
/// Contains the complete cookie header value as received from the client.
pub type CookieString = String;
/// Represents the key/name of an HTTP cookie.
///
/// Used to identify individual cookies in requests and responses.
pub type CookieKey = String;
/// Represents the value/content of an HTTP cookie.
///
/// Stores the actual data associated with a cookie name/key.
pub type CookieValue = String;
/// Represents an optional cookie value.
///
/// Used when a cookie value may or may not be present.
pub type OptionCookiesValue = Option<CookieValue>;
/// Represents an optional cookie expiration date.
///
/// Stores the RFC 1123 formatted date string when present.
pub type OptionCookieExpires = Option<String>;
/// Represents an optional cookie maximum age in seconds.
///
/// Specifies the lifetime of the cookie in seconds when present.
pub type OptionCookieMaxAge = Option<i64>;
/// Represents an optional cookie domain scope.
///
/// Specifies which hosts can receive the cookie when present.
pub type OptionCookieDomain = Option<String>;
/// Represents an optional cookie path scope.
///
/// Specifies URL path that must exist in the requested URL when present.
pub type OptionCookiePath = Option<String>;
/// Represents an optional cookie SameSite policy.
///
/// Specifies if/how cookies should be restricted to first-party sites when present.
/// Possible values: "Strict", "Lax", or "None".
pub type OptionCookieSameSite = Option<String>;
/// Represents a collection of HTTP cookies.
///
/// Stores multiple cookies as key-value pairs using a high-performance hash map.
pub type Cookies = HashMapXxHash3_64<CookieKey, CookieValue>;
