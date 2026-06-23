use crate::*;

/// TLS/QUIC server configuration loader.
///
/// Stores the certificate and key paths and provides methods to build either a
/// `rustls::ServerConfig` (for TLS) or a `quinn::ServerConfig` (for QUIC/HTTP/3).
#[derive(New)]
pub struct TlsConfig<C, K> {
    /// Path to the PEM-encoded certificate chain.
    pub(super) cert_path: C,
    /// Path to the PEM-encoded private key.
    pub(super) key_path: K,
}
