use crate::*;

#[derive(Debug, Clone)]
pub enum ResponseError {
    NotFoundStream,
    ConnectionClosed,
    Unknown(String),
    Response(String),
    WebSocketHandShake(String),
    MethodNotSupported(String),
}
