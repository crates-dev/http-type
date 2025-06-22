use crate::*;

impl Default for HttpVersion {
    fn default() -> Self {
        Self::HTTP1_1
    }
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = match self {
            Self::HTTP0_9 => HTTP_VERSION_0_9,
            Self::HTTP1_0 => HTTP_VERSION_1_0,
            Self::HTTP1_1 => HTTP_VERSION_1_1,
            Self::HTTP2 => HTTP_VERSION_2,
            Self::HTTP3 => HTTP_VERSION_3,
            Self::Unknown(version) => version,
        };
        write!(f, "{}", version_str)
    }
}

impl FromStr for HttpVersion {
    type Err = String;

    fn from_str(version_str: &str) -> Result<Self, Self::Err> {
        match version_str {
            version_0_9 if version_0_9 == HTTP_VERSION_0_9 => Ok(Self::HTTP0_9),
            version_1_0 if version_1_0 == HTTP_VERSION_1_0 => Ok(Self::HTTP1_0),
            version_1_1 if version_1_1 == HTTP_VERSION_1_1 => Ok(Self::HTTP1_1),
            version_2 if version_2 == HTTP_VERSION_2 => Ok(Self::HTTP2),
            version_3 if version_3 == HTTP_VERSION_3 => Ok(Self::HTTP3),
            _ => Ok(Self::Unknown(version_str.to_string())),
        }
    }
}

impl HttpVersion {
    /// Checks if the current version is HTTP/0.9.
    pub fn is_http0_9(&self) -> bool {
        matches!(self, Self::HTTP0_9)
    }

    /// Checks if the current version is HTTP/1.0.
    pub fn is_http1_0(&self) -> bool {
        matches!(self, Self::HTTP1_0)
    }

    /// Checks if the current version is HTTP/1.1.
    pub fn is_http1_1(&self) -> bool {
        matches!(self, Self::HTTP1_1)
    }

    /// Checks if the current version is HTTP/2.
    pub fn is_http2(&self) -> bool {
        matches!(self, Self::HTTP2)
    }

    /// Checks if the current version is HTTP/3.
    pub fn is_http3(&self) -> bool {
        matches!(self, Self::HTTP3)
    }

    /// Checks if the current version is unknown.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    /// Checks if the current version is HTTP/1.1 or higher.
    pub fn is_http1_1_or_higher(&self) -> bool {
        match self {
            Self::HTTP1_1 | Self::HTTP2 | Self::HTTP3 => true,
            _ => false,
        }
    }

    /// Checks if the current version is http.
    pub fn is_http(&self) -> bool {
        !self.is_unknown()
    }
}
