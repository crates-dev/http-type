use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestError {
    Request,
    HttpRead,
    InvalidUrl,
    GetTcpStream,
    GetTlsStream,
    WebSocketRead,
    ReadConnection,
    RequestAborted,
    SetReadTimeout,
    SetWriteTimeout,
    TlsStreamConnect,
    NeedOpenRedirect,
    MaxRedirectTimes,
    TcpStreamConnect,
    MethodsNotSupport,
    TlsConnectorBuild,
    RedirectInvalidUrl,
    RedirectUrlDeadLoop,
    InvalidWebSocketRequest(String),
    InvalidHttpRequestPartsLength(usize),
    Unknown(String),
}
