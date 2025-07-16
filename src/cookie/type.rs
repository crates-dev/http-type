use crate::*;

/// The raw cookie string from the HTTP request header.
pub type CookieString = String;
/// Key type used in the request cookies.
pub type CookieKey = String;
/// Value type used in the request cookies.
pub type CookieValue = String;
/// Optional value for a cookie.
pub type OptionCookiesValue = Option<CookieValue>;
/// Optional expiration date string for a cookie.
pub type OptionCookieExpires = Option<String>;
/// Optional maximum age in seconds for a cookie.
pub type OptionCookieMaxAge = Option<i64>;
/// Optional domain for a cookie.
pub type OptionCookieDomain = Option<String>;
/// Optional path for a cookie.
pub type OptionCookiePath = Option<String>;
/// Optional SameSite policy for a cookie.
pub type OptionCookieSameSite = Option<String>;
// A collection of cookies.
pub type Cookies = HashMapXxHash3_64<CookieKey, CookieValue>;
