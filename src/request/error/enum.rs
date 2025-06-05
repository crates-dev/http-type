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
    RedirectUrlDeadLoop,
    IncompleteWebSocketFrame,
    Request(String),
    Unknown(String),
    InvalidUrl(String),
    SetReadTimeout(String),
    SetWriteTimeout(String),
    TcpStreamConnect(String),
    TlsConnectorBuild(String),
    InvalidWebSocketRequest(String),
    InvalidHttpRequestPartsLength(usize),
}
