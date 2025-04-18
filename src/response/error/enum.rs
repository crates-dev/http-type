use crate::*;

#[derive(Debug, Clone)]
pub enum ResponseError {
    ResponseError(String),
    HasSendResponse(Response),
    CloseError(String),
    WebSocketHandShakeError,
    NotSupportUseThisMethod,
    NotFoundStream,
    Unknown,
}
