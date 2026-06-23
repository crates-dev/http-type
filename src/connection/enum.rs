/// Represents the application-layer protocol version of a connection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionProtocol {
    /// HTTP/1.1 over TCP.
    Http1_1,
    /// HTTP/2 over TCP or TLS.
    Http2,
    /// HTTP/3 over QUIC.
    Http3,
}
