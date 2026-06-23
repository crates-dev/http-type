use crate::*;

/// Implementation block for `ConnectionProtocol`.
///
/// Provides predicates to inspect the protocol variant of a connection.
impl ConnectionProtocol {
    /// Checks if this is an HTTP/1.1 connection.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the protocol is HTTP/1.1, `false` otherwise.
    #[inline(always)]
    pub fn is_http1_1(&self) -> bool {
        matches!(self, Self::Http1_1)
    }

    /// Checks if this is an HTTP/2 connection.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the protocol is HTTP/2, `false` otherwise.
    #[inline(always)]
    pub fn is_http2(&self) -> bool {
        matches!(self, Self::Http2)
    }

    /// Checks if this is an HTTP/3 connection.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the protocol is HTTP/3, `false` otherwise.
    #[inline(always)]
    pub fn is_http3(&self) -> bool {
        matches!(self, Self::Http3)
    }

    /// Checks if this connection uses a QUIC transport.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the protocol is HTTP/3, `false` otherwise.
    #[inline(always)]
    pub fn is_quic(&self) -> bool {
        self.is_http3()
    }
}

/// Implementation block for `HttpConnection`.
///
/// Provides constructors for each supported protocol and access to the
/// underlying TCP stream when applicable.
impl HttpConnection {
    /// Creates a new HTTP/1.1 connection over the given TCP stream.
    ///
    /// # Arguments
    ///
    /// - `TcpStream`: The accepted TCP stream for the connection.
    ///
    /// # Returns
    ///
    /// - `Self`: A new `HttpConnection` configured for HTTP/1.1.
    #[inline(always)]
    pub fn new_http1_1(tcp_stream: TcpStream) -> Self {
        Self {
            protocol: ConnectionProtocol::Http1_1,
            tcp_stream: Some(tcp_stream),
        }
    }

    /// Creates a new HTTP/2 connection over the given TCP stream (h2c).
    ///
    /// # Arguments
    ///
    /// - `TcpStream`: The accepted TCP stream for the connection.
    ///
    /// # Returns
    ///
    /// - `Self`: A new `HttpConnection` configured for HTTP/2.
    #[inline(always)]
    pub fn new_http2(tcp_stream: TcpStream) -> Self {
        Self {
            protocol: ConnectionProtocol::Http2,
            tcp_stream: Some(tcp_stream),
        }
    }

    /// Creates a new HTTP/3 connection (no TCP stream; transport is QUIC).
    ///
    /// # Returns
    ///
    /// - `Self`: A new `HttpConnection` configured for HTTP/3.
    #[inline(always)]
    pub fn new_http3() -> Self {
        Self {
            protocol: ConnectionProtocol::Http3,
            tcp_stream: None,
        }
    }

    /// Takes ownership of the wrapped TCP stream, if any.
    ///
    /// # Returns
    ///
    /// - `Option<TcpStream>`: The TCP stream if present, otherwise `None`.
    #[inline(always)]
    pub fn take_tcp_stream(&mut self) -> Option<TcpStream> {
        self.tcp_stream.take()
    }
}

/// Implementation block for `HttpResponseStream`.
///
/// Provides constructors and methods for managing an HTTP/1.1 or h2c response
/// stream.
impl HttpResponseStream {
    /// Creates a new HTTP/1.1 response stream.
    ///
    /// # Arguments
    ///
    /// - `&'static mut TcpStream`: The TCP stream used to send the response.
    ///
    /// # Returns
    ///
    /// - `Self`: A new `HttpResponseStream` configured for HTTP/1.1.
    #[inline(always)]
    pub fn new_http1_1(tcp_stream: &'static mut TcpStream) -> Self {
        Self {
            protocol: ConnectionProtocol::Http1_1,
            tcp_stream: Some(tcp_stream),
            closed: false,
        }
    }

    /// Marks the stream as closed.
    #[inline(always)]
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Returns whether the stream is closed.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the stream is closed, `false` otherwise.
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Tries to send raw bytes over the stream.
    ///
    /// This only works for HTTP/1.1 / h2c where the stream wraps a raw TCP stream.
    ///
    /// # Arguments
    ///
    /// - `impl AsRef<[u8]>`: The data to send.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>`: `Ok(())` on success, or a `ResponseError`
    ///   if the stream is closed or the write fails.
    pub async fn try_send(&mut self, data: impl AsRef<[u8]>) -> Result<(), ResponseError> {
        if self.closed {
            return Err(ResponseError::ConnectionClosed);
        }
        if let Some(stream) = self.tcp_stream.as_mut() {
            stream.write_all(data.as_ref()).await?;
            stream.flush().await?;
            Ok(())
        } else {
            Err(ResponseError::ConnectionClosed)
        }
    }
}
