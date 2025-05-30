use crate::*;

impl StdError for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundStream => {
                write!(f, "Not found stream")
            }
            Self::Close(data) => write!(f, "Close error{}{}", COLON_SPACE, data),
            Self::Unknown(err) => write!(f, "Unknown error{}{}", COLON_SPACE, err),
            Self::Response(data) => write!(f, "Response error{}{}", COLON_SPACE, data),
            Self::WebSocketHandShake(err) => {
                write!(f, "Websocket handshake error{}{}", COLON_SPACE, err)
            }
            Self::MethodNotSupported(err) => {
                write!(f, "Method not supported{}{}", COLON_SPACE, err)
            }
        }
    }
}
