use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseError {
    NotFoundStream,
    ConnectionClosed,
    Unknown(String),
    Response(String),
    WebSocketHandShake(String),
    MethodNotSupported(String),
}
