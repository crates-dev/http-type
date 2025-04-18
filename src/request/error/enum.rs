use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestError {
    HttpReadError,
    InvalidHttpRequest,
    InvalidWebSocketRequest,
    InvalidUrl,
    TcpStreamConnectError,
    RequestError,
    MethodsNotSupport,
    ReadConnectionError,
    TlsConnectorBuildError,
    SetReadTimeoutError,
    SetWriteTimeoutError,
    TlsStreamConnectError,
    MaxRedirectTimes,
    RedirectUrlDeadLoop,
    RedirectInvalidUrl,
    NeedOpenRedirect,
    Unknown,
}
