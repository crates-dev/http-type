use crate::*;

impl StdError for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request => write!(f, "Request error"),
            Self::HttpRead => write!(f, "Http read error"),
            Self::InvalidUrl => write!(f, "Invalid url"),
            Self::GetTcpStream => write!(f, "Failed to get tcp stream"),
            Self::GetTlsStream => write!(f, "Failed to get tls stream"),
            Self::WebSocketRead => write!(f, "Websocket read error"),
            Self::ReadConnection => write!(f, "Connection read error"),
            Self::RequestAborted => write!(f, "Request aborted"),
            Self::SetReadTimeout => write!(f, "Failed to set read timeout"),
            Self::SetWriteTimeout => write!(f, "Failed to set write timeout"),
            Self::TlsStreamConnect => write!(f, "Tls stream connection error"),
            Self::NeedOpenRedirect => write!(f, "Open redirect required"),
            Self::MaxRedirectTimes => write!(f, "Exceeded maximum redirect attempts"),
            Self::TcpStreamConnect => write!(f, "Tcp stream connection error"),
            Self::MethodsNotSupport => write!(f, "Http method not supported"),
            Self::TlsConnectorBuild => write!(f, "Tls connector build error"),
            Self::RedirectInvalidUrl => write!(f, "Invalid redirect url"),
            Self::RedirectUrlDeadLoop => write!(f, "Redirect url dead loop detected"),
            Self::InvalidWebSocketRequest(err) => {
                write!(f, "Invalid websocket request{}{}", COLON_SPACE, err)
            }
            Self::InvalidHttpRequestPartsLength(len) => {
                write!(f, "Invalid http request parts length{}{}", COLON_SPACE, len)
            }
            Self::Unknown(err) => write!(f, "Unknown error{}{}", COLON_SPACE, err),
        }
    }
}
