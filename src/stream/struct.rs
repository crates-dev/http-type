use crate::*;

/// The underlying transport for a `Stream`.
///
/// Either a plain TCP stream or a TLS-wrapped TCP stream. The enum keeps
/// `Stream` sized without heap indirection so that common TCP connections pay
/// no allocation overhead. The TLS variant is large, but `Stream` is typically
/// passed by reference or leaked into a `Box`, so the size difference is not
/// on the hot path.
#[allow(clippy::large_enum_variant)]
#[derive(CustomDebug, DisplayDebug)]
pub enum StreamTransport {
    /// Plain TCP stream.
    Tcp(TcpStream),
    /// TLS-wrapped TCP stream.
    Tls(tokio_rustls::server::TlsStream<TcpStream>),
}

/// Wrapper around a TCP or TLS-wrapped TCP stream with HTTP parsing configuration.
///
/// Tracks the underlying connection and request limits, and provides helpers
/// for detecting HTTP/2 connection prefaces.
#[derive(CustomDebug, Data, DisplayDebug)]
pub struct Stream {
    /// The underlying transport (TCP or TLS).
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) transport: StreamTransport,
    /// Request parsing limits and security configuration.
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_config: RequestConfig,
    /// Whether the stream has been closed.
    #[get(type(copy))]
    #[get_mut(pub(super))]
    pub(super) closed: bool,
}

impl Stream {
    /// Creates a new `Stream` wrapping a plain TCP connection.
    ///
    /// This keeps the historical `Stream::new` API unchanged.
    #[inline(always)]
    pub fn new(stream: TcpStream, request_config: RequestConfig, closed: bool) -> Self {
        Self {
            transport: StreamTransport::Tcp(stream),
            request_config,
            closed,
        }
    }

    /// Creates a new `Stream` wrapping a TLS-wrapped TCP connection.
    #[inline(always)]
    pub fn new_tls(
        stream: tokio_rustls::server::TlsStream<TcpStream>,
        request_config: RequestConfig,
        closed: bool,
    ) -> Self {
        Self {
            transport: StreamTransport::Tls(stream),
            request_config,
            closed,
        }
    }
}
