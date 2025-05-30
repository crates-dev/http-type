use crate::*;

impl StdError for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundStream => {
                write!(f, "Not found stream")
            }
            Self::Close(data) => write!(f, "Close{}{}", COLON_SPACE, data),
            Self::Response(data) => write!(f, "Response{}{}", COLON_SPACE, data),
            Self::HasSendResponse(data) => {
                write!(f, "Has send response{}{:?}", COLON_SPACE, data)
            }
            Self::WebSocketHandShake(err) => {
                write!(f, "Websocket handshake error{}{}", COLON_SPACE, err)
            }
            Self::MethodNotSupported(err) => {
                write!(f, "Method not supported{}{}", COLON_SPACE, err)
            }
            Self::Unknown(err) => write!(f, "Unknown{}{}", COLON_SPACE, err),
        }
    }
}
