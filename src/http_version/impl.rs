use crate::*;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

impl Default for HttpVersion {
    #[inline]
    fn default() -> Self {
        Self::HTTP1_1
    }
}

impl Display for HttpVersion {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = match self {
            Self::HTTP1_1 => HTTP_VERSION_1_1,
            Self::HTTP2 => HTTP_VERSION_2,
            Self::Unknown(_) => UNKNOWN_HTTP_VERSION,
        };
        write!(f, "{}", version_str)
    }
}

impl FromStr for HttpVersion {
    type Err = String;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            version_1_1 if version_1_1 == HTTP_VERSION_1_1 => Ok(Self::HTTP1_1),
            version_2 if version_2 == HTTP_VERSION_2 => Ok(Self::HTTP2),
            _ => Ok(Self::Unknown(s.to_string())),
        }
    }
}
