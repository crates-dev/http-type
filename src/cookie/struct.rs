use crate::*;

/// HTTP cookie builder.
///
/// Constructs and configures HTTP cookies.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CookieBuilder {
    /// Cookie name.
    pub(super) name: CookieKey,
    /// Cookie value.
    pub(super) value: CookieValue,
    /// Cookie expiration date.
    pub(super) expires: OptionCookieExpires,
    /// Cookie max age in seconds.
    pub(super) max_age: OptionCookieMaxAge,
    /// Cookie domain.
    pub(super) domain: OptionCookieDomain,
    /// Cookie path.
    pub(super) path: OptionCookiePath,
    /// Secure flag.
    pub(super) secure: bool,
    /// HttpOnly flag.
    pub(super) http_only: bool,
    /// SameSite policy.
    pub(super) same_site: OptionCookieSameSite,
}

/// HTTP cookie parser.
///
/// Parses Cookie headers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cookie;
