use crate::*;

#[test]
fn connection_protocol_equality() {
    assert_eq!(ConnectionProtocol::Http1_1, ConnectionProtocol::Http1_1);
    assert_eq!(ConnectionProtocol::Http2, ConnectionProtocol::Http2);
    assert_eq!(ConnectionProtocol::Http3, ConnectionProtocol::Http3);
    assert_ne!(ConnectionProtocol::Http1_1, ConnectionProtocol::Http2);
    assert_ne!(ConnectionProtocol::Http2, ConnectionProtocol::Http3);
}

#[test]
fn connection_protocol_debug() {
    assert_eq!(format!("{:?}", ConnectionProtocol::Http1_1), "Http1_1");
    assert_eq!(format!("{:?}", ConnectionProtocol::Http2), "Http2");
    assert_eq!(format!("{:?}", ConnectionProtocol::Http3), "Http3");
}

#[test]
fn connection_protocol_predicates() {
    assert!(ConnectionProtocol::Http1_1.is_http1_1());
    assert!(!ConnectionProtocol::Http1_1.is_http2());
    assert!(!ConnectionProtocol::Http1_1.is_http3());
    assert!(ConnectionProtocol::Http2.is_http2());
    assert!(ConnectionProtocol::Http3.is_http3());
    assert!(ConnectionProtocol::Http3.is_quic());
    assert!(!ConnectionProtocol::Http2.is_quic());
}

#[test]
fn http_connection_new() {
    use tokio::net::TcpStream;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let conn: HttpConnection = runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let client_handle = runtime.spawn(async move { TcpStream::connect(addr).await.unwrap() });
        let (server_stream, _) = listener.accept().await.unwrap();
        let _ = client_handle.await.unwrap();
        HttpConnection::new_http1_1(server_stream)
    });
    assert_eq!(conn.get_protocol(), ConnectionProtocol::Http1_1);
    assert!(conn.tcp_stream.is_some());
}

#[test]
fn http_connection_new_http2() {
    use tokio::net::TcpStream;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let conn: HttpConnection = runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let client_handle = runtime.spawn(async move { TcpStream::connect(addr).await.unwrap() });
        let (server_stream, _) = listener.accept().await.unwrap();
        let _ = client_handle.await.unwrap();
        HttpConnection::new_http2(server_stream)
    });
    assert_eq!(conn.get_protocol(), ConnectionProtocol::Http2);
    assert!(conn.tcp_stream.is_some());
}

#[test]
fn http_connection_new_http3() {
    let conn: HttpConnection = HttpConnection::new_http3();
    assert_eq!(conn.get_protocol(), ConnectionProtocol::Http3);
    assert!(conn.tcp_stream.is_none());
}
