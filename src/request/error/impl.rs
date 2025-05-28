use crate::*;

impl StdError for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpReadError => write!(f, "Http read error"),
            Self::InvalidHttpRequest => write!(f, "Invalid Http request"),
            Self::InvalidWebSocketRequest => write!(f, "Invalid WebSocket request"),
            Self::InvalidUrl => write!(f, "Invalid Url"),
            Self::TcpStreamConnectError => write!(f, "Tcp stream connection error"),
            Self::RequestError => write!(f, "Request error"),
            Self::MethodsNotSupport => write!(f, "Http method not supported"),
            Self::ReadConnectionError => write!(f, "Connection read error"),
            Self::TlsConnectorBuildError => write!(f, "Tls connector build error"),
            Self::SetReadTimeoutError => write!(f, "Failed to set read timeout"),
            Self::SetWriteTimeoutError => write!(f, "Failed to set write timeout"),
            Self::TlsStreamConnectError => write!(f, "Tls stream connection error"),
            Self::MaxRedirectTimes => write!(f, "Exceeded maximum redirect attempts"),
            Self::RedirectUrlDeadLoop => write!(f, "Redirect Url dead loop detected"),
            Self::RedirectInvalidUrl => write!(f, "Invalid redirect Url"),
            Self::NeedOpenRedirect => write!(f, "Open redirect required"),
            Self::GetTcpStreamError => write!(f, "Failed to get Tcp stream"),
            Self::GetTlsStreamError => write!(f, "Failed to get Tls stream"),
            Self::RequestAborted => write!(f, "Request aborted"),
            Self::Unknown => write!(f, "Unknown error"),
        }
    }
}
