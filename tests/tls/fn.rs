use crate::*;

#[tokio::test]
async fn load_tls_config_success() {
    let config: rustls::ServerConfig = TlsConfig::new("tests/tls/cert.pem", "tests/tls/key.pem")
        .load()
        .await
        .unwrap();
    assert_eq!(
        config.alpn_protocols,
        vec![b"h2".to_vec(), b"http/1.1".to_vec()]
    );
}

#[tokio::test]
async fn load_quic_config_success() {
    let config: quinn::ServerConfig = TlsConfig::new("tests/tls/cert.pem", "tests/tls/key.pem")
        .load_quic()
        .await
        .unwrap();
    let _ = config;
}

#[tokio::test]
async fn load_tls_config_missing_files() {
    assert!(
        TlsConfig::new("tests/tls/not-found.pem", "tests/tls/key.pem")
            .load()
            .await
            .is_err()
    );
    assert!(
        TlsConfig::new("tests/tls/cert.pem", "tests/tls/not-found.pem")
            .load()
            .await
            .is_err()
    );
}
