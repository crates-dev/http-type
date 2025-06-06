use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestError {
    HttpRead,
    GetTcpStream,
    GetTlsStream,
    ReadConnection,
    RequestAborted,
    TlsStreamConnect,
    NeedOpenRedirect,
    MaxRedirectTimes,
    MethodsNotSupport,
    RedirectInvalidUrl,
    ClientDisconnected,
    RedirectUrlDeadLoop,
    ClientClosedConnection,
    IncompleteWebSocketFrame,
    Request(String),
    Unknown(String),
    InvalidUrl(String),
    SetReadTimeout(String),
    SetWriteTimeout(String),
    TcpStreamConnect(String),
    TlsConnectorBuild(String),
    InvalidWebSocketFrame(String),
    InvalidWebSocketRequest(String),
    InvalidHttpRequestPartsLength(usize),
}
