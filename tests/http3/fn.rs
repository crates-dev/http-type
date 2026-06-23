use crate::*;

async fn h3_client_config() -> quinn::ClientConfig {
    let cert_bytes: Vec<u8> = tokio::fs::read("tests/tls/cert.pem").await.unwrap();
    let mut cert_reader: Cursor<Vec<u8>> = Cursor::new(cert_bytes);
    let certs: Vec<rustls::pki_types::CertificateDer<'static>> =
        rustls_pemfile::certs(&mut cert_reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
    let mut roots: rustls::RootCertStore = rustls::RootCertStore::empty();
    for cert in certs {
        roots.add(cert).unwrap();
    }
    let mut rustls_config: rustls::ClientConfig = rustls::ClientConfig::builder_with_provider(
        Arc::new(rustls::crypto::ring::default_provider()),
    )
    .with_protocol_versions(&[&rustls::version::TLS13])
    .unwrap()
    .with_root_certificates(roots)
    .with_no_client_auth();
    rustls_config.alpn_protocols = vec![b"h3".to_vec()];
    let quic_client_config: quinn::crypto::rustls::QuicClientConfig =
        quinn::crypto::rustls::QuicClientConfig::try_from(Arc::new(rustls_config)).unwrap();
    quinn::ClientConfig::new(Arc::new(quic_client_config))
}

#[tokio::test]
async fn test_handle_http3_connection() {
    let server_endpoint: quinn::Endpoint = quinn::Endpoint::server(
        TlsConfig::new("tests/tls/cert.pem", "tests/tls/key.pem")
            .load_quic()
            .await
            .unwrap(),
        "127.0.0.1:0".parse::<SocketAddr>().unwrap(),
    )
    .unwrap();
    let server_addr: SocketAddr = server_endpoint.local_addr().unwrap();
    let server_handle: JoinHandle<()> = tokio::spawn(async move {
        if let Some(incoming) = server_endpoint.accept().await
            && let Ok(conn) = incoming.await
        {
            let _ = Http3Connection::new(http3_test_handler).handle(conn).await;
        }
    });
    let mut client_endpoint: quinn::Endpoint =
        quinn::Endpoint::client("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
    client_endpoint.set_default_client_config(h3_client_config().await);
    let client_conn: quinn::Connection = client_endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    let (mut h3_conn, mut send_request) = h3::client::new(h3_quinn::Connection::new(client_conn))
        .await
        .unwrap();
    let driver: JoinHandle<()> = tokio::spawn(async move {
        poll_fn(|cx| h3_conn.poll_close(cx)).await;
    });
    let request: http::Request<()> = http::Request::builder()
        .method("GET")
        .uri("https://localhost/h3")
        .body(())
        .unwrap();
    let mut req_stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> =
        send_request.send_request(request).await.unwrap();
    req_stream.finish().await.unwrap();
    let response: http::Response<()> = req_stream.recv_response().await.unwrap();
    assert_eq!(response.status(), 200);
    let mut body_bytes: Vec<u8> = Vec::new();
    while let Ok(Some(data)) = req_stream.recv_data().await {
        body_bytes.extend_from_slice(data.chunk());
    }
    assert_eq!(body_bytes, b"h3 ok");
    drop(send_request);
    client_endpoint.wait_idle().await;
    driver.await.unwrap();
    server_handle.await.unwrap();
}
