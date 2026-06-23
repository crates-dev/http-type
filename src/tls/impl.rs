use crate::*;

impl<C, K> TlsConfig<C, K>
where
    C: AsRef<Path>,
    K: AsRef<Path>,
{
    /// Loads a TLS server configuration from the stored PEM certificate and key files.
    ///
    /// The returned `rustls::ServerConfig` advertises ALPN protocols `h2` and `http/1.1`.
    /// File I/O is performed asynchronously via `tokio::fs::read`.
    ///
    /// # Returns
    ///
    /// - `Result<rustls::ServerConfig, BoxError>`: The TLS configuration or an error.
    pub async fn load(self) -> Result<rustls::ServerConfig, BoxError> {
        let cert_bytes: Vec<u8> = tokio_fs_read(self.cert_path).await.map_err(Box::new)?;
        let mut cert_reader: Cursor<Vec<u8>> = Cursor::new(cert_bytes);
        let certs: Vec<rustls::pki_types::CertificateDer<'static>> =
            rustls_pemfile::certs(&mut cert_reader)
                .collect::<Result<Vec<_>, _>>()
                .map_err(Box::new)?;
        let key_bytes: Vec<u8> = tokio_fs_read(self.key_path).await.map_err(Box::new)?;
        let mut key_reader: Cursor<Vec<u8>> = Cursor::new(key_bytes);
        let key: rustls::pki_types::PrivateKeyDer<'static> =
            rustls_pemfile::private_key(&mut key_reader)
                .map_err(Box::new)?
                .ok_or_else(|| BoxError::from("No private key found"))?;
        let mut config: rustls::ServerConfig = rustls::ServerConfig::builder_with_provider(
            Arc::new(rustls::crypto::ring::default_provider()),
        )
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(Box::new)?;
        config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        Ok(config)
    }

    /// Loads a QUIC server configuration from the stored PEM certificate and key files.
    ///
    /// The returned `quinn::ServerConfig` advertises the ALPN protocol `h3` and is
    /// suitable for accepting HTTP/3 connections. File I/O is performed
    /// asynchronously via `tokio::fs::read`.
    ///
    /// # Returns
    ///
    /// - `Result<quinn::ServerConfig, BoxError>`: The QUIC configuration or an error.
    pub async fn load_quic(self) -> Result<quinn::ServerConfig, BoxError> {
        let cert_bytes: Vec<u8> = tokio_fs_read(self.cert_path).await.map_err(Box::new)?;
        let mut cert_reader: Cursor<Vec<u8>> = Cursor::new(cert_bytes);
        let certs: Vec<rustls::pki_types::CertificateDer<'static>> =
            rustls_pemfile::certs(&mut cert_reader)
                .collect::<Result<Vec<_>, _>>()
                .map_err(Box::new)?;
        let key_bytes: Vec<u8> = tokio_fs_read(self.key_path).await.map_err(Box::new)?;
        let mut key_reader: Cursor<Vec<u8>> = Cursor::new(key_bytes);
        let key: rustls::pki_types::PrivateKeyDer<'static> =
            rustls_pemfile::private_key(&mut key_reader)
                .map_err(Box::new)?
                .ok_or_else(|| BoxError::from("No private key found"))?;
        let mut config: rustls::ServerConfig = rustls::ServerConfig::builder_with_provider(
            Arc::new(rustls::crypto::ring::default_provider()),
        )
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(Box::new)?;
        config.max_early_data_size = u32::MAX;
        config.alpn_protocols = vec![b"h3".to_vec()];
        let quic_config: quinn::crypto::rustls::QuicServerConfig =
            quinn::crypto::rustls::QuicServerConfig::try_from(Arc::new(config))
                .map_err(Box::new)?;
        Ok(quinn::ServerConfig::with_crypto(Arc::new(quic_config)))
    }
}
