use crate::*;

use std::{
    fmt::{self, Display},
    str::FromStr,
};

impl Default for Protocol {
    #[inline]
    fn default() -> Self {
        Self::HTTP
    }
}

/// Provides utility methods for the `Protocol` type.
impl Protocol {
    /// Creates a new instance of `Protocol` with the default value of `Self::HTTP`.
    ///
    /// This is a shorthand for using the `default` method.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the current protocol is `HTTP`.
    ///
    /// Returns `true` if the protocol is `HTTP`, otherwise returns `false`.    
    #[inline]
    pub fn is_http(&self) -> bool {
        self.to_owned() == Self::HTTP.to_owned()
    }

    /// Checks if the current protocol is `HTTPS`.
    ///
    /// Returns `true` if the protocol is `HTTPS`, otherwise returns `false`.
    #[inline]
    pub fn is_https(&self) -> bool {
        self.to_owned() == Self::HTTPS.to_owned()
    }

    /// Returns the default port associated with the protocol.
    ///
    /// - Returns `80` for `Self::HTTP`.
    /// - Returns `443` for `Self::HTTPS`.
    #[inline]
    pub fn get_port(&self) -> u16 {
        match self {
            Self::HTTP => 80,
            Self::HTTPS => 443,
            Self::Unknown(_) => 80,
        }
    }
}

impl Display for Protocol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let http: String = HTTP.to_lowercase();
        let https: String = HTTPS.to_lowercase();
        let res: &str = match self {
            Self::HTTP => http.as_str(),
            Self::HTTPS => https.as_str(),
            Self::Unknown(protocol) => protocol,
        };
        write!(f, "{}", res)
    }
}

impl FromStr for Protocol {
    type Err = &'static str;

    #[inline]
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_lowercase().as_str() {
            _data if _data.eq_ignore_ascii_case(HTTP) => Ok(Self::HTTP),
            _data if _data.eq_ignore_ascii_case(HTTPS) => Ok(Self::HTTPS),
            _ => Ok(Self::Unknown(data.to_string())),
        }
    }
}
