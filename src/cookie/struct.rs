use crate::*;

/// A builder for constructing HTTP cookies with various attributes.
#[derive(Debug, Clone, Default)]
pub struct CookieBuilder {
    /// The name of the cookie.
    pub(super) name: CookieKey,
    /// The value of the cookie.
    pub(super) value: CookieValue,
    /// Optional expiration date string (e.g., "Wed, 21 Oct 2015 07:28:00 GMT").
    pub(super) expires: OptionCookieExpires,
    /// Optional maximum age in seconds.
    pub(super) max_age: OptionCookieMaxAge,
    /// Optional domain for the cookie.
    pub(super) domain: OptionCookieDomain,
    /// Optional path for the cookie.
    pub(super) path: OptionCookiePath,
    /// Whether the cookie should only be sent over HTTPS.
    pub(super) secure: bool,
    /// Whether the cookie should be inaccessible to JavaScript.
    pub(super) http_only: bool,
    /// Optional SameSite policy ("Strict", "Lax", or "None").
    pub(super) same_site: OptionCookieSameSite,
}

/// A simple cookie structure for parsing HTTP Cookie headers.
#[derive(Debug, Clone, Default)]
pub struct Cookie;
