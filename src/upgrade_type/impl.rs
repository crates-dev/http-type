use crate::*;

impl Default for UpgradeType {
    #[inline]
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

impl fmt::Display for UpgradeType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpgradeType::Http => write!(f, "{}", HTTP),
            UpgradeType::WebSocket => write!(f, "{}", WEBSOCKET),
            UpgradeType::Unknown(tmp_str) => write!(f, "{}", tmp_str),
        }
    }
}

impl FromStr for UpgradeType {
    type Err = ();

    #[inline]
    fn from_str(from_str: &str) -> Result<Self, Self::Err> {
        match from_str {
            tmp_from_str if tmp_from_str.eq_ignore_ascii_case(HTTP) => Ok(UpgradeType::Http),
            tmp_from_str if tmp_from_str.eq_ignore_ascii_case(WEBSOCKET) => {
                Ok(UpgradeType::WebSocket)
            }
            _ => Ok(UpgradeType::Unknown(from_str.to_string())),
        }
    }
}

impl UpgradeType {
    /// Checks if the current value is `Http`.
    ///
    /// - `self` - The current instance of the enum.
    ///
    /// - Returns `true` if `self` is `Self::Http`, otherwise `false`.
    #[inline]
    pub fn is_http(&self) -> bool {
        *self == Self::Http
    }

    /// Checks if the current value is `WebSocket`.
    ///
    /// - `self` - The current instance of the enum.
    ///
    /// - Returns `true` if `self` is `Self::WebSocket`, otherwise `false`.
    #[inline]
    pub fn is_websocket(&self) -> bool {
        *self == Self::WebSocket
    }

    /// Checks if the current value is neither `Http` nor `WebSocket`.
    ///
    /// - `self` - The current instance of the enum.
    ///
    /// - Returns `true` if `self` is neither `Self::Http` nor `Self::WebSocket`, otherwise `false`.
    #[inline]
    pub fn is_unknown(&self) -> bool {
        !self.is_http() && !self.is_websocket()
    }
}
