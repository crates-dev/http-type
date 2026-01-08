use crate::*;

/// Builder for constructing HTTP cookies.
///
/// Provides methods to set various cookie attributes like expiration,
/// domain, path, and security flags before building the final cookie string.
#[derive(Debug, Clone, Default, PartialEq, Eq, Data)]
pub struct CookieBuilder<'a> {
    /// Cookie name identifier.
    pub(super) name: CookieKey<'a>,
    /// Cookie value content.
    pub(super) value: CookieValue<'a>,
    /// Expiration date/time string.
    #[new(skip)]
    pub(super) expires: &'a str,
    /// Maximum age in seconds.
    #[new(skip)]
    pub(super) max_age: i64,
    /// Domain scope for the cookie.
    #[new(skip)]
    pub(super) domain: &'a str,
    /// Path scope for the cookie.
    #[new(skip)]
    pub(super) path: &'a str,
    /// Flag indicating secure (HTTPS-only) transmission.
    #[new(skip)]
    pub(super) secure: bool,
    /// Flag preventing JavaScript access.
    #[new(skip)]
    pub(super) http_only: bool,
    /// SameSite policy setting.
    #[new(skip)]
    pub(super) same_site: &'a str,
}

/// Parser for HTTP Cookie headers.
///
/// Provides functionality to parse Cookie header strings into key-value pairs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cookie;
