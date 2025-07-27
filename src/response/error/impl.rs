use crate::*;

impl StdError for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundStream => {
                write!(f, "Not found stream")
            }
            Self::ConnectionClosed => {
                write!(f, "Connection has been closed")
            }
            Self::Terminated => {
                write!(f, "Current processing has been terminated")
            }
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
