use crate::*;

impl<F, Fut> Http2Connection<F>
where
    F: Fn(Request, Response) -> Fut + Send + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    /// Handles an HTTP/2 connection accepted on any async I/O stream.
    ///
    /// This method performs the HTTP/2 handshake, accepts each request stream,
    /// decodes it into an `http_type::Request`, invokes the handler closure
    /// with a default response, and sends the returned response back over h2.
    ///
    /// # Arguments
    ///
    /// - `IO`: The bidirectional async I/O stream.
    ///
    /// # Returns
    ///
    /// - `Result<(), BoxError>`: `Ok(())` when the connection closes cleanly, or an
    ///   error if the handshake or a stream fails.
    pub async fn handle<IO>(self, io: IO) -> Result<(), BoxError>
    where
        IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let mut connection: h2::server::Connection<IO, Bytes> = h2::server::handshake(io).await?;
        loop {
            let accept_result: Http2AcceptResult = connection.accept().await;
            match accept_result {
                Some(Ok((request, respond))) => {
                    let (parts, mut body): (http::request::Parts, h2::RecvStream) =
                        request.into_parts();
                    let mut body_bytes: RequestBody = Vec::new();
                    while let Some(chunk_result) = body.data().await {
                        if let Ok(chunk) = chunk_result {
                            body_bytes.extend_from_slice(&chunk);
                        }
                    }
                    let stream_id: u32 = body.stream_id().as_u32();
                    let http_request: Request =
                        Request::from_http2_parts(parts, body_bytes, stream_id);
                    let response: Response = (self.handler)(http_request, Response::default()).await;
                    let _ = Http2Response::new(&response, respond).send().await;
                }
                Some(Err(error)) => {
                    eprintln!("HTTP/2 stream error: {error}");
                }
                None => break,
            }
        }
        Ok(())
    }
}

impl<'a> Http2Response<'a> {
    /// Sends the wrapped `http_type::Response` over the h2 `SendResponse` handle.
    ///
    /// This method converts the http-type response into an HTTP/2 HEADERS frame
    /// followed by a DATA frame.
    ///
    /// # Returns
    ///
    /// - `Result<(), BoxError>`: `Ok(())` on success, or an error if the response
    ///   cannot be sent.
    pub async fn send(self) -> Result<(), BoxError> {
        let response: &Response = self.response;
        let mut respond: Http2SendResponse = self.respond;
        let status: u16 = response.get_status_code() as u16;
        let status_code: http::StatusCode = http::StatusCode::from_u16(status)?;
        let mut http2_response: http::Response<()> =
            http::Response::builder().status(status_code).body(())?;
        for (key, values) in response.get_headers().iter() {
            let key_lower: &str = key.as_str();
            if HTTP2_FORBIDDEN_HEADERS.contains(&key_lower) {
                continue;
            }
            if let Ok(name) = http::header::HeaderName::from_bytes(key_lower.as_bytes()) {
                for value in values.iter() {
                    if let Ok(val) = http::header::HeaderValue::from_str(value) {
                        http2_response.headers_mut().append(name.clone(), val);
                    }
                }
            }
        }
        let mut send_stream = respond.send_response(http2_response, false)?;
        let body: &[u8] = response.get_body();
        if body.is_empty() {
            send_stream.send_data(Bytes::new(), true)?;
        } else {
            send_stream.send_data(Bytes::copy_from_slice(body), true)?;
        }
        Ok(())
    }
}
