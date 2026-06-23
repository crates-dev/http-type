use crate::*;

/// Implementation of `From` trait for converting `usize` address into `&Stream`.
impl From<usize> for &'static Stream {
    /// Converts a memory address into a reference to `Stream`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Stream` instance.
    ///
    /// # Returns
    ///
    /// - `&'static Stream` - A reference to the `Stream` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Stream` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> &'static Stream {
        unsafe { &*(address as *const Stream) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&mut Stream`.
impl<'a> From<usize> for &'a mut Stream {
    /// Converts a memory address into a mutable reference to `Stream`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Stream` instance.
    ///
    /// # Returns
    ///
    /// - `&mut Stream` - A mutable reference to the `Stream` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Stream` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> &'a mut Stream {
        unsafe { &mut *(address as *mut Stream) }
    }
}

/// Implementation of `From` trait for converting `&Stream` into `usize` address.
impl From<&Stream> for usize {
    /// Converts a reference to `Stream` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&Stream` - The reference to the `Stream` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Stream` instance.
    #[inline(always)]
    fn from(stream: &Stream) -> Self {
        stream as *const Stream as usize
    }
}

/// Implementation of `From` trait for converting `&mut Stream` into `usize` address.
impl From<&mut Stream> for usize {
    /// Converts a mutable reference to `Stream` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The mutable reference to the `Stream` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Stream` instance.
    #[inline(always)]
    fn from(stream: &mut Stream) -> Self {
        stream as *mut Stream as usize
    }
}

/// Implementation of `AsRef` trait for `Stream`.
impl AsRef<Stream> for Stream {
    /// Converts `&Stream` to `&Stream` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&Stream` - A reference to the `Stream` instance.
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of `AsMut` trait for `Stream`.
impl AsMut<Stream> for Stream {
    /// Converts `&mut Stream` to `&mut Stream` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&mut Stream` - A mutable reference to the `Stream` instance.
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of `Lifetime` trait for `Stream`.
impl Lifetime for Stream {
    /// Converts a reference to the stream into a `'static` reference.
    ///
    /// # Returns
    ///
    /// - `&'static Self`: A reference to the stream with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Self` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    unsafe fn leak(&self) -> &'static Self {
        let address: usize = self.into();
        address.into()
    }

    /// Converts a reference to the stream into a `'static` mutable reference.
    ///
    /// # Returns
    ///
    /// - `&'static mut Self`: A mutable reference to the stream with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Self` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    unsafe fn leak_mut(&self) -> &'static mut Self {
        let address: usize = self.into();
        address.into()
    }
}

impl Stream {
    /// Returns a reference to the underlying TCP stream.
    ///
    /// # Panics
    ///
    /// Panics if the transport is TLS.
    #[inline(always)]
    pub fn get_stream(&self) -> &TcpStream {
        match &self.transport {
            StreamTransport::Tcp(stream) => stream,
            StreamTransport::Tls(_) => panic!("expected a TCP stream"),
        }
    }

    /// Returns a mutable reference to the underlying TCP stream.
    ///
    /// # Panics
    ///
    /// Panics if the transport is TLS.
    #[inline(always)]
    pub fn get_mut_stream(&mut self) -> &mut TcpStream {
        match &mut self.transport {
            StreamTransport::Tcp(stream) => stream,
            StreamTransport::Tls(_) => panic!("expected a TCP stream"),
        }
    }

    /// Returns a reference to the underlying TLS stream.
    ///
    /// # Panics
    ///
    /// Panics if the transport is plain TCP.
    #[inline(always)]
    pub fn get_tls_stream(&self) -> &tokio_rustls::server::TlsStream<TcpStream> {
        match &self.transport {
            StreamTransport::Tcp(_) => panic!("expected a TLS stream"),
            StreamTransport::Tls(stream) => stream,
        }
    }

    /// Returns a mutable reference to the underlying TLS stream.
    ///
    /// # Panics
    ///
    /// Panics if the transport is plain TCP.
    #[inline(always)]
    pub fn get_mut_tls_stream(&mut self) -> &mut tokio_rustls::server::TlsStream<TcpStream> {
        match &mut self.transport {
            StreamTransport::Tcp(_) => panic!("expected a TLS stream"),
            StreamTransport::Tls(stream) => stream,
        }
    }

    /// Checks if the connection should be kept alive.
    ///
    /// This method evaluates whether the connection should remain open based on
    /// the closed state and the keep_alive parameter.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether keep-alive is enabled for the request.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the connection should be kept alive, otherwise false.
    #[inline(always)]
    pub fn is_keep_alive(&self, keep_alive: bool) -> bool {
        !self.get_closed() && keep_alive
    }

    /// Checks whether the incoming bytes match the HTTP/2 connection preface.
    ///
    /// This method peeks at the first 24 bytes of a plain TCP stream and compares
    /// them to the HTTP/2 connection preface defined in RFC 7540. TLS streams
    /// cannot be peeked, so they always return `false`; HTTP/2 over TLS is
    /// selected via ALPN before the stream is wrapped.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the stream starts with the HTTP/2 preface.
    pub async fn is_http2_preface(&mut self) -> bool {
        let mut peek_buf: [u8; 24] = [0; 24];
        match &mut self.transport {
            StreamTransport::Tcp(stream) => match stream.peek(&mut peek_buf).await {
                Ok(len) if len >= CONNECTION_PREFACE.len() => {
                    peek_buf[..CONNECTION_PREFACE.len()] == *CONNECTION_PREFACE
                }
                _ => false,
            },
            StreamTransport::Tls(_) => false,
        }
    }

    /// Consumes the `Stream` and returns the wrapped TCP stream.
    ///
    /// # Panics
    ///
    /// Panics if the transport is TLS.
    #[inline(always)]
    pub fn into_tcp_stream(self) -> TcpStream {
        match self.transport {
            StreamTransport::Tcp(stream) => stream,
            StreamTransport::Tls(_) => panic!("cannot convert a TLS stream into a TCP stream"),
        }
    }

    /// Replaces the request configuration and returns the mutable stream.
    #[inline(always)]
    pub fn replace_request_config(&mut self, config: RequestConfig) -> &mut Self {
        self.request_config = config;
        self
    }

    /// Parses the HTTP request content from the stream.
    ///
    /// This is an internal helper function that performs the actual parsing.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>`: The parsed request or an error.
    async fn get_http_from_stream(&mut self) -> Result<Request, RequestError> {
        let config: RequestConfig = *self.get_request_config();
        let buffer_size: usize = config.get_buffer_size();
        let max_path_size: usize = config.get_max_path_size();
        match &mut self.transport {
            StreamTransport::Tcp(stream) => {
                let reader: &mut BufReader<&mut TcpStream> =
                    &mut BufReader::with_capacity(buffer_size, stream);
                Self::read_http_from_reader(reader, &config, max_path_size).await
            }
            StreamTransport::Tls(stream) => {
                let reader: &mut BufReader<&mut tokio_rustls::server::TlsStream<TcpStream>> =
                    &mut BufReader::with_capacity(buffer_size, stream);
                Self::read_http_from_reader(reader, &config, max_path_size).await
            }
        }
    }

    /// Generic HTTP request reader used for both TCP and TLS transports.
    async fn read_http_from_reader<R>(
        reader: &mut BufReader<R>,
        config: &RequestConfig,
        max_path_size: usize,
    ) -> Result<Request, RequestError>
    where
        R: AsyncRead + Unpin,
    {
        let buffer_size: usize = config.get_buffer_size();
        let mut line: String = String::with_capacity(buffer_size);
        AsyncBufReadExt::read_line(reader, &mut line).await?;
        let (method, path, version): (RequestMethod, &str, RequestVersion) =
            Request::get_http_first_line(&line)?;
        Request::check_http_path_size(path, max_path_size)?;
        let hash_index: Option<usize> = path.find(HASH);
        let query_index: Option<usize> = path.find(QUERY);
        let query: &str = Request::get_http_query(path, query_index, hash_index);
        let querys: RequestQuerys = Request::get_http_querys(query);
        let path: RequestPath = Request::get_http_path(path, query_index, hash_index);
        let (headers, host, content_size): (RequestHeaders, RequestHost, usize) =
            Request::get_http_headers(reader, config).await?;
        let body: RequestBody = Request::get_http_body(reader, content_size).await?;
        Ok(Request {
            method,
            host,
            version,
            path,
            querys,
            headers,
            pseudo_headers: hash_map_xx_hash3_64(),
            body,
            stream_id: 0,
        })
    }

    /// Parses an HTTP request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `http_from_reader`.
    /// If the timeout is DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS, no timeout is applied.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed request or an error.
    pub async fn try_get_http_request(&mut self) -> Result<Request, RequestError> {
        if self.get_closed() {
            return Err(RequestError::ServerClosedConnection(HttpStatus::BadRequest));
        }
        let timeout_ms: u64 = self.get_request_config().get_read_timeout_ms();
        if timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
            return self.get_http_from_stream().await;
        }
        let duration: Duration = Duration::from_millis(timeout_ms);
        timeout(duration, self.get_http_from_stream()).await?
    }

    /// Parses a WebSocket request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `ws_from_reader`.
    /// If the timeout is DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS, no timeout is applied.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>`: The parsed WebSocket request or an error.
    pub async fn try_get_websocket_request(&mut self) -> Result<RequestBody, RequestError> {
        if self.get_closed() {
            return Err(RequestError::ServerClosedConnection(HttpStatus::BadRequest));
        }
        let config: RequestConfig = *self.get_request_config();
        let buffer_size: usize = config.get_buffer_size();
        let read_timeout_ms: u64 = config.get_read_timeout_ms();
        let mut dynamic_buffer: Vec<u8> = Vec::with_capacity(buffer_size);
        let mut temp_buffer: Vec<u8> = vec![0; buffer_size];
        let mut full_frame: Vec<u8> = Vec::new();
        let mut is_client_response: bool = false;
        let duration_opt: Option<Duration> =
            if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
                None
            } else {
                let adjusted_timeout_ms: u64 = (read_timeout_ms >> 1) + (read_timeout_ms & 1);
                Some(Duration::from_millis(adjusted_timeout_ms))
            };
        loop {
            let len: usize = match self
                .get_websocket_from_stream(&mut temp_buffer, duration_opt, &mut is_client_response)
                .await
            {
                Ok(Some(len)) => len,
                Ok(None) => continue,
                Err(error) => return Err(error),
            };
            if len == 0 {
                return Err(RequestError::IncompleteWebSocketFrame(
                    HttpStatus::BadRequest,
                ));
            }
            dynamic_buffer.extend_from_slice(&temp_buffer[..len]);
            while let Some((frame, consumed)) = WebSocketFrame::decode_ws_frame(&dynamic_buffer) {
                is_client_response = true;
                dynamic_buffer.drain(0..consumed);
                match frame.get_opcode() {
                    WebSocketOpcode::Close => {
                        return Err(RequestError::ClientClosedConnection(HttpStatus::BadRequest));
                    }
                    WebSocketOpcode::Ping | WebSocketOpcode::Pong => continue,
                    WebSocketOpcode::Text | WebSocketOpcode::Binary => {
                        match frame.build_full_frame(&mut full_frame) {
                            Ok(Some(result)) => return Ok(result),
                            Ok(None) => continue,
                            Err(error) => return Err(error),
                        }
                    }
                    _ => {
                        return Err(RequestError::WebSocketOpcodeUnsupported(
                            HttpStatus::NotImplemented,
                        ));
                    }
                }
            }
        }
    }

    /// Reads data from the stream with optional timeout handling.
    ///
    /// # Arguments
    ///
    /// - `&mut [u8]`: The buffer to read data into.
    /// - `Option<Duration>`: The optional timeout duration. If Some, timeout is applied; if None, no timeout.
    /// - `&mut bool`: Mutable reference to track if we got a client response.
    ///
    /// # Returns
    ///
    /// - `Result<Option<usize>, RequestError>`: The number of bytes read, None for timeout/ping, or an error.
    pub(crate) async fn get_websocket_from_stream(
        &mut self,
        buffer: &mut [u8],
        duration_opt: Option<Duration>,
        is_client_response: &mut bool,
    ) -> Result<Option<usize>, RequestError> {
        if let Some(duration) = duration_opt {
            return match &mut self.transport {
                StreamTransport::Tcp(stream) => match timeout(duration, stream.read(buffer)).await {
                    Ok(result) => match result {
                        Ok(len) => Ok(Some(len)),
                        Err(error) => Err(error.into()),
                    },
                    Err(error) => {
                        if !*is_client_response {
                            return Err(error.into());
                        }
                        *is_client_response = false;
                        self.try_send(&PING_FRAME).await?;
                        Ok(None)
                    }
                },
                StreamTransport::Tls(stream) => match timeout(duration, stream.read(buffer)).await {
                    Ok(result) => match result {
                        Ok(len) => Ok(Some(len)),
                        Err(error) => Err(error.into()),
                    },
                    Err(error) => {
                        if !*is_client_response {
                            return Err(error.into());
                        }
                        *is_client_response = false;
                        self.try_send(&PING_FRAME).await?;
                        Ok(None)
                    }
                },
            };
        }
        match &mut self.transport {
            StreamTransport::Tcp(stream) => match stream.read(buffer).await {
                Ok(len) => Ok(Some(len)),
                Err(error) => Err(error.into()),
            },
            StreamTransport::Tls(stream) => match stream.read(buffer).await {
                Ok(len) => Ok(Some(len)),
                Err(error) => Err(error.into()),
            },
        }
    }

    /// Sends data over the stream.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The data to send (must implement AsRef<[u8]>).
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send<D>(&mut self, data: D) -> Result<(), ResponseError>
    where
        D: AsRef<[u8]>,
    {
        if self.get_closed() {
            return Err(ResponseError::ConnectionClosed);
        }
        match &mut self.transport {
            StreamTransport::Tcp(stream) => stream.write_all(data.as_ref()).await?,
            StreamTransport::Tls(stream) => stream.write_all(data.as_ref()).await?,
        }
        Ok(())
    }

    /// Sends data over the stream.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The data to send (must implement AsRef<[u8]>).
    ///
    /// # Panics
    ///
    /// Panics if the write operation fails.
    pub async fn send<D>(&mut self, data: D)
    where
        D: AsRef<[u8]>,
    {
        self.try_send(data).await.unwrap();
    }

    /// Sends multiple data.
    ///
    /// # Arguments
    ///
    /// - `IntoIterator<Item = AsRef<[u8]>>` - The data list to send.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send_list<I, D>(&mut self, data_iter: I) -> Result<(), ResponseError>
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        if self.get_closed() {
            return Err(ResponseError::ConnectionClosed);
        }
        match &mut self.transport {
            StreamTransport::Tcp(stream) => {
                for data in data_iter {
                    stream.write_all(data.as_ref()).await?;
                }
            }
            StreamTransport::Tls(stream) => {
                for data in data_iter {
                    stream.write_all(data.as_ref()).await?;
                }
            }
        }
        Ok(())
    }

    /// Sends multiple data.
    ///
    /// # Arguments
    ///
    /// - `IntoIterator<Item = AsRef<[u8]>>` - The data list to send.
    ///
    /// # Panics
    ///
    /// Panics if any write operation fails.
    pub async fn send_list<I, D>(&mut self, data_iter: I)
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        self.try_send_list(data_iter).await.unwrap();
    }

    /// Flushes all buffered data to the stream.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_flush(&mut self) -> Result<(), ResponseError> {
        if self.get_closed() {
            return Err(ResponseError::ConnectionClosed);
        }
        match &mut self.transport {
            StreamTransport::Tcp(stream) => stream.flush().await?,
            StreamTransport::Tls(stream) => stream.flush().await?,
        }
        Ok(())
    }

    /// Flushes all buffered data to the stream.
    ///
    /// # Panics
    ///
    /// Panics if the flush operation fails.
    pub async fn flush(&mut self) {
        self.try_flush().await.unwrap();
    }
}
