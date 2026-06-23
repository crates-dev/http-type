use crate::*;

#[test]
fn test_socket_host_type_alias() {
    let socket_host: SocketHost = "127.0.0.1".parse().unwrap();
    assert!(socket_host.is_ipv4());
    let socket_host_v6: SocketHost = "::1".parse().unwrap();
    assert!(socket_host_v6.is_ipv6());
}

#[test]
fn test_socket_port_type_alias() {
    let socket_port: SocketPort = 8080;
    assert_eq!(socket_port, 8080_u16);
}

#[test]
fn test_from_http2_parts() {
    let http_request: http::Request<()> = http::Request::builder()
        .method("GET")
        .uri("https://example.com/path?key=value")
        .version(http::Version::HTTP_2)
        .header("content-type", "text/plain")
        .body(())
        .unwrap();
    let (parts, _): (http::request::Parts, ()) = http_request.into_parts();
    let request: Request = Request::from_http2_parts(parts, b"body".to_vec(), 1);
    assert_eq!(*request.get_version(), HttpVersion::Http2);
    assert_eq!(request.get_stream_id(), 1);
    assert!(request.has_stream_id());
    assert_eq!(request.get_path(), "/path?key=value");
    assert_eq!(request.get_querys().get("key"), Some(&"value".to_string()));
    assert_eq!(
        request.try_get_pseudo_header(COLON_METHOD),
        Some("GET".to_string())
    );
    assert_eq!(
        request.try_get_pseudo_header(COLON_SCHEME),
        Some("https".to_string())
    );
    assert_eq!(
        request.try_get_pseudo_header(COLON_PATH),
        Some("/path?key=value".to_string())
    );
    assert_eq!(
        request.try_get_pseudo_header(COLON_AUTHORITY),
        Some("example.com".to_string())
    );
    assert!(request.has_header("content-type"));
}

#[tokio::test]
async fn test_stream_http2_preface_detection() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_addr: SocketAddr = listener.local_addr().unwrap();
    let connect_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let mut client: TcpStream = TcpStream::connect(local_addr).await.unwrap();
        client.write_all(CONNECTION_PREFACE).await.unwrap();
    });
    let (server_stream, _) = listener.accept().await.unwrap();
    let mut stream: Stream = Stream::new(server_stream, RequestConfig::default(), false);
    assert!(stream.is_http2_preface().await);
    connect_handle.await.unwrap();
}

#[tokio::test]
async fn test_stream_no_http2_preface() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_addr: SocketAddr = listener.local_addr().unwrap();
    let connect_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let mut client: TcpStream = TcpStream::connect(local_addr).await.unwrap();
        client.write_all(b"GET / HTTP/1.1\r\n").await.unwrap();
    });
    let (server_stream, _) = listener.accept().await.unwrap();
    let mut stream: Stream = Stream::new(server_stream, RequestConfig::default(), false);
    assert!(!stream.is_http2_preface().await);
    connect_handle.await.unwrap();
}
