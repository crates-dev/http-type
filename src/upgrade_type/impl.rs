use crate::*;

impl Default for UpgradeType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

impl fmt::Display for UpgradeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebSocket => write!(f, "{}", WEBSOCKET),
            Self::H2c => write!(f, "{}", H2C_LOWERCASE),
            Self::Tls(version) => write!(f, "{}", version),
            Self::Unknown(tmp_str) => write!(f, "{}", tmp_str),
        }
    }
}

impl FromStr for UpgradeType {
    type Err = ();

    fn from_str(from_str: &str) -> Result<Self, Self::Err> {
        match from_str.to_ascii_lowercase().as_str() {
            val if val == WEBSOCKET => Ok(Self::WebSocket),
            H2C_LOWERCASE => Ok(Self::H2c),
            val if val.starts_with(TLS_LOWERCASE) => Ok(Self::Tls(val.to_string())),
            other => Ok(Self::Unknown(other.to_string())),
        }
    }
}

impl UpgradeType {
    /// Checks if the current upgrade type is `WebSocket`.
    ///
    /// - `&self` - The current instance of the enum.
    ///
    /// - Returns `true` if `self` is `Self::WebSocket`, otherwise `false`.
    pub fn is_ws(&self) -> bool {
        matches!(self, &Self::WebSocket)
    }

    /// Checks if the current upgrade type is HTTP/2 cleartext (`h2c`).
    ///
    /// - `&self` - The current instance of the enum.
    ///
    /// - Returns `true` if `self` is `Self::H2c`, otherwise `false`.
    pub fn is_h2c(&self) -> bool {
        matches!(self, &Self::H2c)
    }

    /// Checks if the current upgrade type is a TLS variant (any version).
    ///
    /// - `&self` - The current instance of the enum.
    ///
    /// - Returns `true` if `self` matches `Self::Tls(_)`, otherwise `false`.
    pub fn is_tls(&self) -> bool {
        matches!(self, Self::Tls(_))
    }

    /// Checks if the current upgrade type is unknown (neither `WebSocket`, `H2c`, nor `Tls`).
    ///
    /// - `&self` - The current instance of the enum.
    ///
    /// - Returns `true` if `self` is none of the known upgrade types, otherwise `false`.
    pub fn is_unknown(&self) -> bool {
        !self.is_ws() && !self.is_h2c() && !self.is_tls()
    }
}
