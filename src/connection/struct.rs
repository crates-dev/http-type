use crate::*;

/// A transport-agnostic handle for an active HTTP connection.
///
/// `HttpConnection` abstracts over the underlying transport so that the rest of
/// the server can work with HTTP/1.1, HTTP/2, and HTTP/3 connections uniformly.
#[derive(Data, Debug)]
pub struct HttpConnection {
    /// The application-layer protocol for this connection.
    #[get(pub, type(copy))]
    #[set(skip)]
    pub(super) protocol: ConnectionProtocol,
    /// The raw TCP stream when the connection is HTTP/1.1 or HTTP/2 (h2c).
    #[get(skip)]
    #[set(skip)]
    pub tcp_stream: Option<TcpStream>,
}

/// A transport-agnostic handle for sending a response.
///
/// This type is intentionally simple: for HTTP/1.1 it wraps the raw TCP stream,
/// while HTTP/2 and HTTP/3 handlers receive their own sender types from the
/// underlying crates and do not use this wrapper directly.
#[derive(Data, Debug)]
pub struct HttpResponseStream {
    /// The application-layer protocol for this stream.
    #[get(pub, type(copy))]
    #[set(skip)]
    pub(super) protocol: ConnectionProtocol,
    /// The raw TCP stream for HTTP/1.1 or h2c responses.
    #[get(skip)]
    #[set(skip)]
    pub(super) tcp_stream: Option<&'static mut TcpStream>,
    /// Whether the stream has been closed.
    #[get(skip)]
    #[set(skip)]
    pub(super) closed: bool,
}
