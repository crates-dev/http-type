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
    Unknown(String),
    InvalidWebSocketRequest(String),
    InvalidHttpRequestPartsLength(usize),
}
