use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ResponseError(String),
    HasSendResponse(Response),
    CloseError(String),
    WebSocketHandShakeError,
    Unknown,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponseError(data) => write!(f, "Response Error{}{}", COLON_SPACE, data),
            Self::HasSendResponse(data) => write!(f, "Response Error{}{:?}", COLON_SPACE, data),
            Self::CloseError(data) => write!(f, "Close Error{}{}", COLON_SPACE, data),
            Self::WebSocketHandShakeError => write!(f, "Websocket handshake error"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
