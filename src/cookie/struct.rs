use crate::*;

/// A builder for constructing HTTP cookies.
///
/// This struct provides a way to create and configure HTTP cookies.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CookieBuilder {
    /// The name of the cookie.
    pub(super) name: CookieKey,
    /// The value of the cookie.
    pub(super) value: CookieValue,
    /// The expiration date of the cookie.
    pub(super) expires: OptionCookieExpires,
    /// The maximum age of the cookie in seconds.
    pub(super) max_age: OptionCookieMaxAge,
    /// The domain of the cookie.
    pub(super) domain: OptionCookieDomain,
    /// The path of the cookie.
    pub(super) path: OptionCookiePath,
    /// The `Secure` flag of the cookie.
    pub(super) secure: bool,
    /// The `HttpOnly` flag of the cookie.
    pub(super) http_only: bool,
    /// The `SameSite` policy of the cookie.
    pub(super) same_site: OptionCookieSameSite,
}

/// A utility for parsing HTTP `Cookie` headers.
///
/// This struct provides a static method to parse the `Cookie` header.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cookie;
