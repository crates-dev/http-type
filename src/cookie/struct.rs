use crate::*;

/// Builder for constructing HTTP cookies.
///
/// Provides methods to set various cookie attributes like expiration,
/// domain, path, and security flags before building the final cookie string.
#[derive(Debug, Clone, Default, PartialEq, Eq, Getter, GetterMut, Setter)]
pub struct CookieBuilder {
    /// Cookie name identifier.
    pub(super) name: CookieKey,
    /// Cookie value content.
    pub(super) value: CookieValue,
    /// Optional expiration date/time string.
    pub(super) expires: Option<String>,
    /// Optional maximum age in seconds.
    pub(super) max_age: Option<i64>,
    /// Optional domain scope for the cookie.
    pub(super) domain: Option<String>,
    /// Optional path scope for the cookie.
    pub(super) path: Option<String>,
    /// Flag indicating secure (HTTPS-only) transmission.
    pub(super) secure: bool,
    /// Flag preventing JavaScript access.
    pub(super) http_only: bool,
    /// Optional SameSite policy setting.
    pub(super) same_site: Option<String>,
}

/// Parser for HTTP Cookie headers.
///
/// Provides functionality to parse Cookie header strings into key-value pairs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cookie;
