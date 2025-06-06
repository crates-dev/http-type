use crate::*;

impl StdError for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpRead => write!(f, "Http read error"),
            Self::GetTcpStream => write!(f, "Failed to get tcp stream"),
            Self::GetTlsStream => write!(f, "Failed to get tls stream"),
            Self::ReadConnection => write!(f, "Connection read error"),
            Self::RequestAborted => write!(f, "Request aborted"),
            Self::TlsStreamConnect => write!(f, "Tls stream connection error"),
            Self::NeedOpenRedirect => write!(f, "Open redirect required"),
            Self::MaxRedirectTimes => write!(f, "Exceeded maximum redirect attempts"),
            Self::MethodsNotSupport => write!(f, "Http method not supported"),
            Self::RedirectInvalidUrl => write!(f, "Invalid redirect url"),
            Self::ClientDisconnected => write!(f, "Client disconnected"),
            Self::RedirectUrlDeadLoop => write!(f, "Redirect url dead loop detected"),
            Self::ClientClosedConnection => {
                write!(f, "Client closed connection")
            }
            Self::IncompleteWebSocketFrame => write!(
                f,
                "WebSocket connection closed before a complete frame was received"
            ),
            Self::Request(err) => write!(f, "Request error{}{}", COLON_SPACE, err),
            Self::Unknown(err) => write!(f, "Unknown error{}{}", COLON_SPACE, err),
            Self::InvalidUrl(err) => write!(f, "Invalid url{}{}", COLON_SPACE, err),
            Self::SetReadTimeout(err) => {
                write!(f, "Failed to set read timeout{}{}", COLON_SPACE, err)
            }
            Self::SetWriteTimeout(err) => {
                write!(f, "Failed to set write timeout{}{}", COLON_SPACE, err)
            }
            Self::TcpStreamConnect(err) => {
                write!(f, "Tcp stream connection error{}{}", COLON_SPACE, err)
            }
            Self::TlsConnectorBuild(err) => {
                write!(f, "Tls connector build error{}{}", COLON_SPACE, err)
            }
            Self::InvalidWebSocketFrame(err) => {
                write!(f, "Invalid websocket frame{}{}", COLON_SPACE, err)
            }

            Self::InvalidWebSocketRequest(err) => {
                write!(f, "Invalid websocket request{}{}", COLON_SPACE, err)
            }
            Self::InvalidHttpRequestPartsLength(len) => {
                write!(f, "Invalid http request parts length{}{}", COLON_SPACE, len)
            }
        }
    }
}
