use crate::*;

/// Builder for constructing HTTP cookies.
///
/// Provides methods to set various cookie attributes like expiration,
/// domain, path, and security flags before building the final cookie string.
#[derive(Debug, Clone, Default, PartialEq, Eq, Setter, Getter, GetterMut)]
pub struct CookieBuilder {
    /// Cookie name identifier.
    pub(super) name: CookieKey,
    /// Cookie value content.
    pub(super) value: CookieValue,
    /// Expiration date/time string.
    pub(super) expires: String,
    /// Maximum age in seconds.
    pub(super) max_age: i64,
    /// Domain scope for the cookie.
    pub(super) domain: String,
    /// Path scope for the cookie.
    pub(super) path: String,
    /// Flag indicating secure (HTTPS-only) transmission.
    pub(super) secure: bool,
    /// Flag preventing JavaScript access.
    pub(super) http_only: bool,
    /// SameSite policy setting.
    pub(super) same_site: String,
}

/// Parser for HTTP Cookie headers.
///
/// Provides functionality to parse Cookie header strings into key-value pairs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cookie;
