use crate::*;

impl StdError for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponseError(data) => write!(f, "Response Error{}{}", COLON_SPACE, data),
            Self::HasSendResponse(data) => write!(f, "Response Error{}{:?}", COLON_SPACE, data),
            Self::CloseError(data) => write!(f, "Close Error{}{}", COLON_SPACE, data),
            Self::WebSocketHandShakeError => write!(f, "Websocket handshake error"),
            Self::NotSupportUseThisMethod => {
                write!(f, "This method call is not supported")
            }
            Self::NotFoundStream => {
                write!(f, "Not found stream")
            }
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
