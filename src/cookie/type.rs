use crate::*;

/// The raw cookie string from an HTTP request header.
pub type CookieString = String;
/// The key of a cookie.
pub type CookieKey = String;
/// The value of a cookie.
pub type CookieValue = String;
/// An optional cookie value.
pub type OptionCookiesValue = Option<CookieValue>;
/// An optional cookie expiration date.
pub type OptionCookieExpires = Option<String>;
/// An optional cookie maximum age in seconds.
pub type OptionCookieMaxAge = Option<i64>;
/// An optional cookie domain.
pub type OptionCookieDomain = Option<String>;
/// An optional cookie path.
pub type OptionCookiePath = Option<String>;
/// An optional cookie `SameSite` policy.
pub type OptionCookieSameSite = Option<String>;
/// A collection of cookies.
pub type Cookies = HashMapXxHash3_64<CookieKey, CookieValue>;
