use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    HttpReadError,
    InvalidHttpRequest,
    InvalidUrl,
    TcpStreamConnectError,
    RequestError,
    MethodsNotSupport,
    ReadConnectionError,
    TlsConnectorBuildError,
    SetReadTimeoutError,
    SetWriteTimeoutError,
    TlsStreamConnectError,
    MaxRedirectTimes,
    RedirectUrlDeadLoop,
    RedirectInvalidUrl,
    NeedOpenRedirect,
    Unknown,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpReadError => write!(f, "Http read error"),
            Self::InvalidHttpRequest => write!(f, "Invalid http request"),
            Self::InvalidUrl => write!(f, "Invalid URL"),
            Self::TcpStreamConnectError => write!(f, "TCP Stream Connection Error"),
            Self::RequestError => write!(f, "Request Error"),
            Self::MethodsNotSupport => write!(f, "Unsupported HTTP Method"),
            Self::ReadConnectionError => write!(f, "Connection Read Error"),
            Self::TlsConnectorBuildError => write!(f, "TLS Connector Build Error"),
            Self::SetReadTimeoutError => write!(f, "Failed to Set Read Timeout"),
            Self::SetWriteTimeoutError => write!(f, "Failed to Set Write Timeout"),
            Self::TlsStreamConnectError => write!(f, "TLS Stream Connection Error"),
            Self::MaxRedirectTimes => write!(f, "Max Redirect Times"),
            Self::RedirectUrlDeadLoop => write!(f, "Redirect URL Dead Loop"),
            Self::RedirectInvalidUrl => write!(f, "Redirect Invalid Url"),
            Self::NeedOpenRedirect => write!(f, "Need Open Redirect"),
            Self::Unknown => write!(f, "{}", "Unknown"),
        }
    }
}
