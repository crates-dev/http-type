use super::r#type::HttpVersion;
use http_constant::*;
use std::fmt::{self, Display};

impl Default for HttpVersion {
    fn default() -> Self {
        Self::HTTP1_1
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = match self {
            Self::HTTP1_1 => HTTP_VERSION_1_1,
            Self::HTTP2 => HTTP_VERSION_2,
            Self::Unknown(_) => UNKNOWN_HTTP_VERSION,
        };
        write!(f, "{}", version_str)
    }
}
