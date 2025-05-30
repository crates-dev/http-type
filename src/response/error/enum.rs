use crate::*;

#[derive(Debug, Clone)]
pub enum ResponseError {
    NotFoundStream,
    Close(String),
    Unknown(String),
    Response(String),
    WebSocketHandShake(String),
    MethodNotSupported(String),
}
