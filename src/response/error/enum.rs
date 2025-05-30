use crate::*;

#[derive(Debug, Clone)]
pub enum ResponseError {
    NotFoundStream,
    Close(String),
    Response(String),
    HasSendResponse(Response),
    WebSocketHandShake(String),
    MethodNotSupported(String),
    Unknown(String),
}
