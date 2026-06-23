use crate::*;

#[tokio::test]
async fn test_handle_http2_connection() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let server_handle: JoinHandle<()> = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let _ = Http2Connection::new(http2_test_handler).handle(stream).await;
    });
    let client: TcpStream = TcpStream::connect(addr).await.unwrap();
    let (mut client, connection) = h2::client::handshake(client).await.unwrap();
    let driver: JoinHandle<()> = tokio::spawn(async move {
        let _ = connection.await;
    });
    let request: http::Request<()> = http::Request::builder()
        .method("GET")
        .uri("http://localhost/h2")
        .body(())
        .unwrap();
    let (response, _) = client.send_request(request, true).unwrap();
    let response: http::Response<h2::RecvStream> = response.await.unwrap();
    assert_eq!(response.status(), 200);
    let mut body: h2::RecvStream = response.into_body();
    let mut body_bytes: Vec<u8> = Vec::new();
    while let Some(chunk_result) = body.data().await {
        if let Ok(chunk) = chunk_result {
            body_bytes.extend_from_slice(&chunk);
        }
    }
    assert_eq!(body_bytes, b"h2 ok");
    drop(client);
    driver.abort();
    server_handle.abort();
}
