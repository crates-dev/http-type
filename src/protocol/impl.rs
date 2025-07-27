use crate::*;

/// Provides a default value for `Protocol`.
///
/// The default `Protocol` is `Protocol::Unknown` with an empty string.
impl Default for Protocol {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Provides utility methods for the `Protocol` type.
impl Protocol {
    /// Creates a new instance of `Protocol` with the default value of `Self::HTTP`.
    ///
    /// # Returns
    ///
    /// A new `Protocol` instance, defaulting to `Protocol::HTTP`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the current protocol is `HTTP`.
    ///
    /// # Arguments
    ///
    /// - `self` - A reference to the `Protocol` instance.
    ///
    /// # Returns
    ///
    /// `true` if the protocol is `HTTP`, otherwise returns `false`.
    pub fn is_http(&self) -> bool {
        self.to_owned() == Self::HTTP.to_owned()
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
    pub fn is_https(&self) -> bool {
        self.to_owned() == Self::HTTPS.to_owned()
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
        match data.to_ascii_lowercase().as_str() {
            HTTP_LOWERCASE => Ok(Self::HTTP),
            HTTPS_LOWERCASE => Ok(Self::HTTPS),
            _ => Ok(Self::Unknown(data.to_string())),
        }
    }
}
