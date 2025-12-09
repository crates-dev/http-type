use crate::*;

/// Implements the `Default` trait for `UpgradeType`.
///
/// Provides a default value for `UpgradeType`, which is `Self::Unknown(String::new())`.
impl Default for UpgradeType {
    /// Returns the default `UpgradeType`, which is `Self::Unknown(String::new())`.
    ///
    /// # Returns
    ///
    /// The default `UpgradeType` instance.
    #[inline(always)]
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Implements the `Display` trait for `UpgradeType`.
/// This allows `UpgradeType` variants to be formatted into human-readable strings.
impl Display for UpgradeType {
    /// Formats the `UpgradeType` variant into a human-readable string.
    ///
    /// # Arguments
    ///
    /// - `f`: A mutable reference to a `fmt::Formatter` used for writing the formatted string.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating whether the formatting was successful.
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebSocket => write!(f, "{WEBSOCKET}"),
            Self::H2c => write!(f, "{H2C_LOWERCASE}"),
            Self::Tls(version) => write!(f, "{version}"),
            Self::Unknown(tmp_str) => write!(f, "{tmp_str}"),
        }
    }
}

/// Implements the `FromStr` trait for `UpgradeType`.
/// This allows parsing string representations into `UpgradeType` variants.
impl FromStr for UpgradeType {
    /// The error type for `FromStr` implementation, which is a unit type `()` indicating no specific error information.
    type Err = ();

    /// Parses a string slice into an `UpgradeType`.
    ///
    /// The parsing is case-insensitive. It recognizes "websocket", "h2c", and strings starting with "tls" as specific types.
    /// Any other string is parsed as `Self::Unknown`.
    ///
    /// # Arguments
    ///
    /// - `from_str`: The string slice to parse.
    ///
    /// # Returns
    ///
    /// - `Ok(UpgradeType)`: The parsed `UpgradeType` variant.
    /// - `Err(())`: If parsing fails (though this implementation always returns `Ok`).
    #[inline(always)]
    fn from_str(from_str: &str) -> Result<Self, Self::Err> {
        match from_str.to_ascii_lowercase().as_str() {
            WEBSOCKET => Ok(Self::WebSocket),
            H2C_LOWERCASE => Ok(Self::H2c),
            val if val.starts_with(TLS_LOWERCASE) => Ok(Self::Tls(val.to_string())),
            other => Ok(Self::Unknown(other.to_string())),
        }
    }
}

impl UpgradeType {
    /// Checks if the current upgrade type is `WebSocket`.
    ///
    /// # Returns
    ///
    /// `true` if `self` is `Self::WebSocket`, otherwise `false`.
    #[inline(always)]
    pub fn is_ws(&self) -> bool {
        matches!(self, &Self::WebSocket)
    }

    /// Checks if the current upgrade type is HTTP/2 cleartext (`h2c`).
    ///
    /// # Returns
    ///
    /// `true` if `self` is `Self::H2c`, otherwise `false`.
    #[inline(always)]
    pub fn is_h2c(&self) -> bool {
        matches!(self, &Self::H2c)
    }

    /// Checks if the current upgrade type is a TLS variant (any version).
    ///
    /// # Returns
    ///
    /// `true` if `self` matches `Self::Tls(_)`, otherwise `false`.
    #[inline(always)]
    pub fn is_tls(&self) -> bool {
        matches!(self, Self::Tls(_))
    }

    /// Checks if the current upgrade type is unknown (neither `WebSocket`, `H2c`, nor `Tls`).
    ///
    /// # Returns
    ///
    /// `true` if `self` is none of the known upgrade types, otherwise `false`.
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        !self.is_ws() && !self.is_h2c() && !self.is_tls()
    }
}
