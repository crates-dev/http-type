use crate::*;

impl<F, Fut> Http3Connection<F>
where
    F: Fn(Request, Response) -> Fut + Send + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    /// Handles an HTTP/3 connection over a QUIC connection.
    ///
    /// This method builds an h3 server connection over the given Quinn connection,
    /// accepts each request stream, decodes it into an `http_type::Request`,
    /// invokes the handler closure with a default response, and sends the returned
    /// response back over the h3 request stream.
    ///
    /// # Arguments
    ///
    /// - `quinn::Connection`: The accepted QUIC connection.
    ///
    /// # Returns
    ///
    /// - `Result<(), BoxError>`: `Ok(())` when the connection closes cleanly, or an
    ///   error if a stream fails.
    pub async fn handle(self, quic_conn: quinn::Connection) -> Result<(), BoxError> {
        let mut connection: Http3ServerConnection = h3::server::builder()
            .build(h3_quinn::Connection::new(quic_conn))
            .await?;
        loop {
            match connection.accept().await {
                Ok(Some(request_resolver)) => match request_resolver.resolve_request().await {
                    Ok((request, mut stream)) => {
                        let stream_id: u64 = stream.id().into_inner();
                        let (parts, ()): (http::request::Parts, ()) = request.into_parts();
                        let mut body_bytes: RequestBody = Vec::new();
                        while let Ok(Some(chunk)) = stream.recv_data().await {
                            body_bytes.extend_from_slice(chunk.chunk());
                        }
                        let http_request: Request =
                            Request::from_http2_parts(parts, body_bytes, stream_id as u32);
                        let response: Response =
                            (self.handler)(http_request, Response::default()).await;
                        let _ = Http3Response::new(&response, &mut stream).send().await;
                    }
                    Err(error) => {
                        eprintln!("HTTP/3 request resolution error: {error}");
                    }
                },
                Ok(None) => break,
                Err(_error) => {
                    break;
                }
            }
        }
        Ok(())
    }
}

impl<'a> Http3Response<'a> {
    /// Sends the wrapped `http_type::Response` over the h3 request stream.
    ///
    /// # Returns
    ///
    /// - `Result<(), BoxError>`: `Ok(())` on success, or an error if the response
    ///   cannot be sent.
    pub async fn send(self) -> Result<(), BoxError> {
        let response: &Response = self.response;
        let stream: &mut Http3RequestStream = self.stream;
        let status: u16 = response.get_status_code() as u16;
        let status_code: http::StatusCode = http::StatusCode::from_u16(status)?;
        let mut http3_response: http::Response<()> =
            http::Response::builder().status(status_code).body(())?;
        for (key, values) in response.get_headers().iter() {
            let key_lower: &str = key.as_str();
            if HTTP2_FORBIDDEN_HEADERS.contains(&key_lower) {
                continue;
            }
            if let Ok(name) = http::header::HeaderName::from_bytes(key_lower.as_bytes()) {
                for value in values.iter() {
                    if let Ok(val) = http::header::HeaderValue::from_str(value) {
                        http3_response.headers_mut().append(name.clone(), val);
                    }
                }
            }
        }
        stream.send_response(http3_response).await?;
        let body: &[u8] = response.get_body();
        if !body.is_empty() {
            stream.send_data(Bytes::copy_from_slice(body)).await?;
        }
        stream.finish().await?;
        Ok(())
    }
}
