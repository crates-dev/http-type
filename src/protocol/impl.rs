use crate::*;

impl Default for Protocol {
    fn default() -> Self {
        Self::HTTP
    }
}

/// Provides utility methods for the `Protocol` type.
impl Protocol {
    /// Creates a new instance of `Protocol` with the default value of `Self::HTTP`.
    ///
    /// This is a shorthand for using the `default` method.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the current protocol is `HTTP`.
    ///
    /// Returns `true` if the protocol is `HTTP`, otherwise returns `false`.    
    pub fn is_http(&self) -> bool {
        self.to_owned() == Self::HTTP.to_owned()
    }

    /// Checks if the current protocol is `HTTPS`.
    ///
    /// Returns `true` if the protocol is `HTTPS`, otherwise returns `false`.
    pub fn is_https(&self) -> bool {
        self.to_owned() == Self::HTTPS.to_owned()
    }

    /// Returns the default port associated with the protocol.
    ///
    /// - Returns `80` for `Self::HTTP`.
    /// - Returns `443` for `Self::HTTPS`.
    pub fn get_port(&self) -> u16 {
        match self {
            Self::HTTP => 80,
            Self::HTTPS => 443,
            Self::Unknown(_) => 80,
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Self::HTTP => HTTP_LOWERCASE,
            Self::HTTPS => HTTPS_LOWERCASE,
            Self::Unknown(protocol) => protocol,
        };
        write!(f, "{}", res)
    }
}

impl FromStr for Protocol {
    type Err = &'static str;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data {
            _data if _data == HTTP_LOWERCASE => Ok(Self::HTTP),
            _data if _data == HTTPS_LOWERCASE => Ok(Self::HTTPS),
            _ => Ok(Self::Unknown(data.to_string())),
        }
    }
}
