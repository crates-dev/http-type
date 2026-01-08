use crate::*;

/// Provides a default value for `Protocol`.
///
/// The default `Protocol` is `Protocol::Unknown` with an empty string.
impl Default for Protocol {
    #[inline(always)]
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Provides utility methods for the `Protocol` type.
impl Protocol {
    /// Checks if the current protocol is `HTTP`.
    ///
    /// # Arguments
    ///
    /// - `self` - A reference to the `Protocol` instance.
    ///
    /// # Returns
    ///
    /// `true` if the protocol is `HTTP`, otherwise returns `false`.
    #[inline(always)]
    pub fn is_http(&self) -> bool {
        *self == Self::Http
    }

    /// Checks if the current protocol is `HTTPS`.
    ///
    /// # Arguments
    ///
    /// - `self` - A reference to the `Protocol` instance.
    ///
    /// # Returns
    ///
    /// `true` if the protocol is `HTTPS`, otherwise returns `false`.
    #[inline(always)]
    pub fn is_https(&self) -> bool {
        *self == Self::Https
    }

    /// Returns the default port associated with the protocol.
    ///
    /// # Arguments
    ///
    /// - `self` - A reference to the `Protocol` instance.
    ///
    /// # Returns
    ///
    /// The default port number for the protocol. Returns `80` for `HTTP` and unknown protocols,
    /// and `443` for `HTTPS`.
    #[inline(always)]
    pub fn get_port(&self) -> u16 {
        match self {
            Self::Http => 80,
            Self::Https => 443,
            Self::Unknown(_) => 80,
        }
    }
}

impl Display for Protocol {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Self::Http => HTTP_LOWERCASE,
            Self::Https => HTTPS_LOWERCASE,
            Self::Unknown(protocol) => protocol,
        };
        write!(f, "{res}")
    }
}

impl FromStr for Protocol {
    type Err = &'static str;

    #[inline(always)]
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_ascii_lowercase().as_str() {
            HTTP_LOWERCASE => Ok(Self::Http),
            HTTPS_LOWERCASE => Ok(Self::Https),
            _ => Ok(Self::Unknown(data.to_string())),
        }
    }
}
