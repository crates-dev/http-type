use crate::*;

impl StdError for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpReadError => write!(f, "HttpReadError"),
            Self::InvalidHttpRequest => write!(f, "InvalidHttpRequest"),
            Self::InvalidWebSocketRequest => write!(f, "InvalidWebSocketRequest"),
            Self::InvalidUrl => write!(f, "InvalidUrl"),
            Self::TcpStreamConnectError => write!(f, "TcpStreamConnectError"),
            Self::RequestError => write!(f, "RequestError"),
            Self::MethodsNotSupport => write!(f, "HttpMethodNotSupported"),
            Self::ReadConnectionError => write!(f, "ConnectionReadError"),
            Self::TlsConnectorBuildError => write!(f, "TlsConnectorBuildError"),
            Self::SetReadTimeoutError => write!(f, "SetReadTimeoutError"),
            Self::SetWriteTimeoutError => write!(f, "SetWriteTimeoutError"),
            Self::TlsStreamConnectError => write!(f, "TlsStreamConnectError"),
            Self::MaxRedirectTimes => write!(f, "MaxRedirectAttemptsExceeded"),
            Self::RedirectUrlDeadLoop => write!(f, "RedirectUrlDeadLoop"),
            Self::RedirectInvalidUrl => write!(f, "RedirectInvalidUrl"),
            Self::NeedOpenRedirect => write!(f, "OpenRedirectRequired"),
            Self::GetTcpStreamError => write!(f, "GetTcpStreamError"),
            Self::GetTlsStreamError => write!(f, "GetTlsStreamError"),
            Self::Unknown => write!(f, "UnknownError"),
        }
    }
}
