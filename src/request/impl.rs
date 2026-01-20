use crate::*;

/// Implements the `std::error::Error` trait for `RequestError`.
impl std::error::Error for RequestError {}

/// Provides a default value for `RequestError`.
impl Default for RequestError {
    /// Provides a default value for `RequestError`.
    ///
    /// Returns a `RequestError::Unknown` with `HttpStatus::InternalServerError`.
    #[inline(always)]
    fn default() -> Self {
        RequestError::Unknown(HttpStatus::InternalServerError)
    }
}

impl RequestError {
    /// Gets the HTTP status associated with this error.
    ///
    /// Returns the HttpStatus enum variant that corresponds to this error.
    ///
    /// # Arguments
    ///
    /// - `&self` - The RequestError instance.
    ///
    /// # Returns
    ///
    /// - `HttpStatus` - The HTTP status associated with this error.
    pub fn get_http_status(&self) -> HttpStatus {
        match self {
            Self::HttpRead(status) => *status,
            Self::GetTcpStream(status) => *status,
            Self::GetTlsStream(status) => *status,
            Self::ReadConnection(status) => *status,
            Self::RequestAborted(status) => *status,
            Self::TlsStreamConnect(status) => *status,
            Self::NeedOpenRedirect(status) => *status,
            Self::MaxRedirectTimes(status) => *status,
            Self::MethodsNotSupport(status) => *status,
            Self::RedirectInvalidUrl(status) => *status,
            Self::ClientDisconnected(status) => *status,
            Self::RedirectUrlDeadLoop(status) => *status,
            Self::ClientClosedConnection(status) => *status,
            Self::IncompleteWebSocketFrame(status) => *status,
            Self::RequestTooLong(status) => *status,
            Self::PathTooLong(status) => *status,
            Self::QueryTooLong(status) => *status,
            Self::HeaderLineTooLong(status) => *status,
            Self::TooManyHeaders(status) => *status,
            Self::HeaderKeyTooLong(status) => *status,
            Self::HeaderValueTooLong(status) => *status,
            Self::ContentLengthTooLarge(status) => *status,
            Self::InvalidContentLength(status) => *status,
            Self::InvalidUrlScheme(status) => *status,
            Self::InvalidUrlHost(status) => *status,
            Self::InvalidUrlPort(status) => *status,
            Self::InvalidUrlPath(status) => *status,
            Self::InvalidUrlQuery(status) => *status,
            Self::InvalidUrlFragment(status) => *status,
            Self::ReadTimeout(status) => *status,
            Self::WriteTimeout(status) => *status,
            Self::TcpConnectionFailed(status) => *status,
            Self::TlsHandshakeFailed(status) => *status,
            Self::TlsCertificateInvalid(status) => *status,
            Self::WebSocketFrameTooLarge(status) => *status,
            Self::WebSocketOpcodeUnsupported(status) => *status,
            Self::WebSocketMaskMissing(status) => *status,
            Self::WebSocketPayloadCorrupted(status) => *status,
            Self::WebSocketInvalidUtf8(status) => *status,
            Self::WebSocketInvalidCloseCode(status) => *status,
            Self::WebSocketInvalidExtension(status) => *status,
            Self::HttpRequestPartsInsufficient(status) => *status,
            Self::TcpStreamConnect(status) => *status,
            Self::TlsConnectorBuild(status) => *status,
            Self::InvalidUrl(status) => *status,
            Self::ConfigReadError(status) => *status,
            Self::TcpStreamConnectString(status) => *status,
            Self::TlsConnectorBuildString(status) => *status,
            Self::Request(_) => HttpStatus::BadRequest,
            Self::Unknown(status) => *status,
        }
    }

    /// Gets the numeric HTTP status code associated with this error.
    ///
    /// Returns the numeric status code (e.g., 400, 404, 500) that corresponds to this error.
    ///
    /// # Arguments
    ///
    /// - `&self` - The RequestError instance.
    ///
    /// # Returns
    ///
    /// - `ResponseStatusCode` - The numeric HTTP status code.
    pub fn get_http_status_code(&self) -> ResponseStatusCode {
        self.get_http_status().code()
    }
}

impl Default for RequestConfigData {
    /// Creates a `RequestConfig` with secure default values.
    ///
    /// # Returns
    ///
    /// - `RequestConfigData ` - A new config instance with secure defaults.
    #[inline(always)]
    fn default() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            max_request_line_length: DEFAULT_MAX_REQUEST_LINE_LENGTH,
            max_path_length: DEFAULT_MAX_PATH_LENGTH,
            max_query_length: DEFAULT_MAX_QUERY_LENGTH,
            max_header_line_length: DEFAULT_MAX_HEADER_LINE_LENGTH,
            max_header_count: DEFAULT_MAX_HEADER_COUNT,
            max_header_key_length: DEFAULT_MAX_HEADER_KEY_LENGTH,
            max_header_value_length: DEFAULT_MAX_HEADER_VALUE_LENGTH,
            max_body_size: DEFAULT_MAX_BODY_SIZE,
            max_ws_frame_size: DEFAULT_MAX_WS_FRAME_SIZE,
            max_ws_frames: DEFAULT_MAX_WS_FRAMES,
            http_read_timeout_ms: DEFAULT_HTTP_READ_TIMEOUT_MS,
            ws_read_timeout_ms: DEFAULT_WS_READ_TIMEOUT_MS,
        }
    }
}

impl RequestConfigData {
    /// Creates a config optimized for high-security environments.
    ///
    /// This configuration uses more restrictive limits to provide
    /// maximum protection against various attacks.
    ///
    /// # Returns
    ///
    /// - `RequestConfigData ` - A new config with high-security settings.
    #[inline(always)]
    pub(super) fn high_security() -> Self {
        Self {
            buffer_size: DEFAULT_HIGH_SECURITY_BUFFER_SIZE,
            max_request_line_length: DEFAULT_HIGH_SECURITY_MAX_REQUEST_LINE_LENGTH,
            max_path_length: DEFAULT_HIGH_SECURITY_MAX_PATH_LENGTH,
            max_query_length: DEFAULT_HIGH_SECURITY_MAX_QUERY_LENGTH,
            max_header_line_length: DEFAULT_HIGH_SECURITY_MAX_HEADER_LINE_LENGTH,
            max_header_count: DEFAULT_HIGH_SECURITY_MAX_HEADER_COUNT,
            max_header_key_length: DEFAULT_HIGH_SECURITY_MAX_HEADER_KEY_LENGTH,
            max_header_value_length: DEFAULT_HIGH_SECURITY_MAX_HEADER_VALUE_LENGTH,
            max_body_size: DEFAULT_HIGH_SECURITY_MAX_BODY_SIZE,
            max_ws_frame_size: DEFAULT_HIGH_SECURITY_MAX_WS_FRAME_SIZE,
            max_ws_frames: DEFAULT_HIGH_SECURITY_MAX_WS_FRAMES,
            http_read_timeout_ms: DEFAULT_HIGH_SECURITY_HTTP_READ_TIMEOUT_MS,
            ws_read_timeout_ms: DEFAULT_HIGH_SECURITY_WS_READ_TIMEOUT_MS,
        }
    }
}

/// Implementation of `Default` trait for `RequestConfig`.
impl Default for RequestConfig {
    /// Creates a new `RequestConfig` with default secure settings.
    ///
    /// This constructor initializes the configuration with standard security limits
    /// suitable for most HTTP request parsing scenarios.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestConfig` instance with default settings.
    #[inline(always)]
    fn default() -> Self {
        Self(arc_rwlock(RequestConfigData::default()))
    }
}

/// Implementation of `PartialEq` trait for `RequestConfig`.
impl PartialEq for RequestConfig {
    /// Compares two `RequestConfig` instances for equality.
    ///
    /// # Arguments
    ///
    /// - `&self` - The first `RequestConfig` instance.
    /// - `other` - The second `RequestConfig` instance to compare.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        let self_arc: &Arc<RwLock<RequestConfigData>> = self.get_0();
        let other_arc: &Arc<RwLock<RequestConfigData>> = other.get_0();
        if Arc::as_ptr(self_arc) == Arc::as_ptr(other_arc) {
            return true;
        }
        if let (Ok(s), Ok(o)) = (self.get_0().try_read(), other.get_0().try_read()) {
            *s == *o
        } else {
            false
        }
    }
}

/// Implementation of `Eq` trait for `RequestConfig`.
impl Eq for RequestConfig {}

/// Implementation of `From` trait for `RequestConfig`.
impl From<RequestConfigData> for RequestConfig {
    /// Converts a `RequestConfigData` into a `RequestConfig`.
    ///
    /// # Arguments
    ///
    /// - `RequestConfigData` - The wrapped context data.
    ///
    /// # Returns
    ///
    /// - `RequestConfig` - The newly created context instance.
    #[inline(always)]
    fn from(ctx: RequestConfigData) -> Self {
        Self(arc_rwlock(ctx))
    }
}

impl RequestConfig {
    /// Creates a new `RequestConfig` with default secure settings.
    ///
    /// This constructor initializes the configuration with standard security limits
    /// suitable for most HTTP request parsing scenarios.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestConfig` instance with default settings.
    pub async fn new() -> Self {
        Self(arc_rwlock(RequestConfigData::default()))
    }

    /// Creates a new `RequestConfig` from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The configuration.
    ///
    /// # Returns
    ///
    /// - `Result<RequestConfig, serde_json::Error>` - The parsed `RequestConfig` or an error.
    pub fn from_json<C>(json: C) -> Result<RequestConfig, serde_json::Error>
    where
        C: AsRef<str>,
    {
        serde_json::from_str(json.as_ref()).map(|data: RequestConfigData| Self::from(data))
    }

    /// Creates a new `RequestConfig` with high-security settings.
    ///
    /// This constructor initializes the configuration with more restrictive limits
    /// to provide maximum protection against various attacks in high-risk environments.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestConfig` instance with high-security settings.
    pub async fn high_security() -> Self {
        Self(arc_rwlock(RequestConfigData::high_security()))
    }

    /// Acquires a read lock on the inner configuration.
    ///
    /// This method returns a `RwLockReadGuard` that allows reading the
    /// `RequestConfigData ` values. Multiple threads can read concurrently.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuard<'_, RequestConfigData >` - A read guard providing access to the inner configuration.
    async fn read(&'_ self) -> RwLockReadGuard<'_, RequestConfigData> {
        self.get_0().read().await
    }

    /// Acquires a write lock on the inner configuration.
    ///
    /// This method returns a `RwLockWriteGuard` that allows mutating the
    /// `RequestConfigData ` values. Write access is exclusive.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuard<'_, RequestConfigData >` - A write guard providing exclusive access to the inner configuration.
    async fn write(&'_ self) -> RwLockWriteGuard<'_, RequestConfigData> {
        self.get_0().write().await
    }

    /// Sets the configuration data.
    ///
    /// # Arguments
    ///
    /// - `RequestConfigData` - The configuration data.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn data(&self, data: RequestConfigData) -> &Self {
        *self.write().await = data;
        self
    }

    /// Gets the configuration data.
    ///
    /// # Returns
    ///
    /// - `RequestConfigData ` - The inner configuration.
    pub async fn get_data(&self) -> RequestConfigData {
        *self.read().await
    }

    /// Sets the buffer size for reading operations.
    ///
    /// # Arguments
    ///
    /// - `usize` - The buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn buffer_size(&self, buffer_size: usize) -> &Self {
        self.write().await.set_buffer_size(buffer_size);
        self
    }

    /// Sets the maximum length for HTTP request line in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum request line length.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_request_line_length(&self, max_request_line_length: usize) -> &Self {
        self.write()
            .await
            .set_max_request_line_length(max_request_line_length);
        self
    }

    /// Sets the maximum length for URL path in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum path length.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_path_length(&self, max_path_length: usize) -> &Self {
        self.write().await.set_max_path_length(max_path_length);
        self
    }

    /// Sets the maximum length for query string in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum query string length.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_query_length(&self, max_query_length: usize) -> &Self {
        self.write().await.set_max_query_length(max_query_length);
        self
    }

    /// Sets the maximum length for a single header line in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum header line length.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_header_line_length(&self, max_header_line_length: usize) -> &Self {
        self.write()
            .await
            .set_max_header_line_length(max_header_line_length);
        self
    }

    /// Sets the maximum number of headers allowed in a request.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum header count.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_header_count(&self, max_header_count: usize) -> &Self {
        self.write().await.set_max_header_count(max_header_count);
        self
    }

    /// Sets the maximum length for a header key in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum header key length.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_header_key_length(&self, max_header_key_length: usize) -> &Self {
        self.write()
            .await
            .set_max_header_key_length(max_header_key_length);
        self
    }

    /// Sets the maximum length for a header value in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum header value length.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_header_value_length(&self, max_header_value_length: usize) -> &Self {
        self.write()
            .await
            .set_max_header_value_length(max_header_value_length);
        self
    }

    /// Sets the maximum size for request body in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum body size.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_body_size(&self, max_body_size: usize) -> &Self {
        self.write().await.set_max_body_size(max_body_size);
        self
    }

    /// Sets the maximum size for WebSocket frame in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum WebSocket frame size.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_ws_frame_size(&self, max_ws_frame_size: usize) -> &Self {
        self.write().await.set_max_ws_frame_size(max_ws_frame_size);
        self
    }

    /// Sets the maximum number of WebSocket frames to process in a single request.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum WebSocket frames count.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn max_ws_frames(&self, max_ws_frames: usize) -> &Self {
        self.write().await.set_max_ws_frames(max_ws_frames);
        self
    }

    /// Sets the timeout for reading HTTP request in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - The HTTP read timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn http_read_timeout_ms(&self, http_read_timeout_ms: u64) -> &Self {
        self.write()
            .await
            .set_http_read_timeout_ms(http_read_timeout_ms);
        self
    }

    /// Sets the timeout for reading WebSocket frames in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - The WebSocket read timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `&Self` - The RequestConfig instance for chaining.
    pub async fn ws_read_timeout_ms(&self, ws_read_timeout_ms: u64) -> &Self {
        self.write()
            .await
            .set_ws_read_timeout_ms(ws_read_timeout_ms);
        self
    }
}

/// Provides a default value for `Request`.
///
/// Returns a new `Request` instance with all fields initialized to their default values.
impl Default for Request {
    #[inline(always)]
    fn default() -> Self {
        Self {
            method: Method::default(),
            host: String::new(),
            version: HttpVersion::default(),
            path: String::new(),
            querys: hash_map_xx_hash3_64(),
            headers: hash_map_xx_hash3_64(),
            body: Vec::new(),
        }
    }
}

impl Request {
    /// Parses a query string as_ref key-value pairs.
    ///
    /// Expects format "key1=value1&key2=value2". Empty values are allowed.
    ///
    /// # Arguments
    ///
    /// - `&str` - The query string to parse.
    ///
    /// # Returns
    ///
    /// - `HashMap<String, String>` - The parsed query parameters.
    fn parse_querys<Q>(query: Q) -> RequestQuerys
    where
        Q: AsRef<str>,
    {
        let mut query_map: RequestQuerys = hash_map_xx_hash3_64();
        for pair in query.as_ref().split(AND) {
            if let Some((key, value)) = pair.split_once(EQUAL) {
                if !key.is_empty() {
                    query_map.insert(key.to_string(), value.to_string());
                }
            } else if !pair.is_empty() {
                query_map.insert(pair.to_string(), String::new());
            }
        }
        query_map
    }

    /// Parses HTTP headers from a buffered reader.
    ///
    /// This method reads header lines from the provided buffered reader until an empty line
    /// is encountered, which indicates the end of headers. It validates header count, length,
    /// and content according to the provided configuration.
    ///
    /// # Arguments
    ///
    /// - `&mut R` - A mutable reference to a buffered reader implementing `AsyncBufReadExt`.
    /// - `&RequestConfigData` - Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<(RequestHeaders, RequestHost, usize), RequestError>` - A tuple containing:
    ///   - The parsed headers as a hash map
    ///   - The host value extracted from the Host header
    ///   - The content length extracted from the Content-Length header
    ///   - Or an error if parsing fails
    async fn parse_headers<R>(
        reader: &mut R,
        config: &RequestConfigData,
    ) -> Result<(RequestHeaders, RequestHost, usize), RequestError>
    where
        R: AsyncBufReadExt + Unpin,
    {
        let buffer_size: usize = config.get_buffer_size();
        let max_header_line_length: usize = config.get_max_header_line_length();
        let max_header_count: usize = config.get_max_header_count();
        let max_header_key_length: usize = config.get_max_header_key_length();
        let max_header_value_length: usize = config.get_max_header_value_length();
        let max_body_size: usize = config.get_max_body_size();
        let mut headers: RequestHeaders = hash_map_xx_hash3_64();
        let mut host: RequestHost = String::new();
        let mut content_length: usize = 0;
        let mut header_count: usize = 0;
        loop {
            let header_line: &mut String = &mut String::with_capacity(buffer_size);
            let bytes_read: usize = AsyncBufReadExt::read_line(reader, header_line)
                .await
                .map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
            if bytes_read > max_header_line_length {
                return Err(RequestError::HeaderLineTooLong(
                    HttpStatus::RequestHeaderFieldsTooLarge,
                ));
            }
            let header_line: &str = header_line.trim();
            if header_line.is_empty() {
                break;
            }
            header_count += 1;
            if header_count > max_header_count {
                return Err(RequestError::TooManyHeaders(
                    HttpStatus::RequestHeaderFieldsTooLarge,
                ));
            }
            if let Some((key_part, value_part)) = header_line.split_once(COLON) {
                let key: String = key_part.trim().to_ascii_lowercase();
                if key.is_empty() {
                    continue;
                }
                if key.len() > max_header_key_length {
                    return Err(RequestError::HeaderKeyTooLong(
                        HttpStatus::RequestHeaderFieldsTooLarge,
                    ));
                }
                let value: String = value_part.trim().to_string();
                if value.len() > max_header_value_length {
                    return Err(RequestError::HeaderValueTooLong(
                        HttpStatus::RequestHeaderFieldsTooLarge,
                    ));
                }
                if key == HOST {
                    host = value.clone();
                } else if key == CONTENT_LENGTH {
                    match value.parse::<usize>() {
                        Ok(length) => {
                            if length > max_body_size {
                                return Err(RequestError::ContentLengthTooLarge(
                                    HttpStatus::PayloadTooLarge,
                                ));
                            }
                            content_length = length;
                        }
                        Err(_) => {
                            return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
                        }
                    }
                }
                headers.entry(key).or_default().push_back(value);
            }
        }
        Ok((headers, host, content_length))
    }

    /// Parses an HTTP request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `http_from_reader`.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLock<TcpStream>` - The TCP stream to read from.
    /// - `&RequestConfigData` - Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed request or an error.
    pub async fn http_from_stream(
        stream: &ArcRwLockStream,
        config: &RequestConfigData,
    ) -> Result<Request, RequestError> {
        let buffer_size: usize = config.get_buffer_size();
        let max_request_line_length: usize = config.get_max_request_line_length();
        let max_path_length: usize = config.get_max_path_length();
        let max_query_length: usize = config.get_max_query_length();
        let http_read_timeout_ms: u64 = config.get_http_read_timeout_ms();
        timeout(Duration::from_millis(http_read_timeout_ms), async move {
            let mut buf_stream: RwLockWriteGuard<'_, TcpStream> = stream.write().await;
            let reader: &mut BufReader<&mut TcpStream> =
                &mut BufReader::with_capacity(buffer_size, &mut buf_stream);
            let mut request_line: String = String::with_capacity(buffer_size);
            let bytes_read: usize = AsyncBufReadExt::read_line(reader, &mut request_line)
                .await
                .map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
            if bytes_read > max_request_line_length {
                return Err(RequestError::RequestTooLong(HttpStatus::BadRequest));
            }
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            let parts_len: usize = parts.len();
            if parts_len < 3 {
                return Err(RequestError::HttpRequestPartsInsufficient(
                    HttpStatus::BadRequest,
                ));
            }
            let method: RequestMethod = parts[0]
                .parse::<RequestMethod>()
                .unwrap_or(Method::Unknown(parts[0].to_string()));
            let full_path: &str = parts[1];
            if full_path.len() > max_path_length {
                return Err(RequestError::PathTooLong(HttpStatus::URITooLong));
            }
            let full_path: RequestPath = full_path.to_string();
            let version: RequestVersion = parts[2]
                .parse::<RequestVersion>()
                .unwrap_or(RequestVersion::Unknown(parts[2].to_string()));
            let hash_index: Option<usize> = full_path.find(HASH);
            let query_index: Option<usize> = full_path.find(QUERY);
            let query_string: String = query_index.map_or_else(String::new, |i| {
                let temp: &str = &full_path[i + 1..];
                if hash_index.is_none() || hash_index.unwrap() <= i {
                    return temp.to_owned();
                }
                temp.split(HASH).next().unwrap_or_default().to_owned()
            });
            if query_string.len() > max_query_length {
                return Err(RequestError::QueryTooLong(HttpStatus::URITooLong));
            }
            let querys: RequestQuerys = Self::parse_querys(&query_string);
            let path: RequestPath = if let Some(i) = query_index.or(hash_index) {
                full_path[..i].to_owned()
            } else {
                full_path.to_owned()
            };
            let (headers, host, content_length): (RequestHeaders, RequestHost, usize) =
                Self::parse_headers(reader, config).await?;
            let mut body: RequestBody = Vec::with_capacity(content_length);
            if content_length > 0 {
                body.resize(content_length, 0);
                AsyncReadExt::read_exact(reader, &mut body)
                    .await
                    .map_err(|_| RequestError::ReadConnection(HttpStatus::BadRequest))?;
            }
            Ok(Request {
                method,
                host,
                version,
                path,
                querys,
                headers,
                body,
            })
        })
        .await
        .map_err(|_| RequestError::ReadTimeout(HttpStatus::RequestTimeout))?
    }

    /// Parses a WebSocket request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `ws_from_reader`.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLock<TcpStream>` - The TCP stream to read from.
    /// - `&RequestConfigData` - Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed WebSocket request or an error.
    pub async fn ws_from_stream(
        &self,
        stream: &ArcRwLockStream,
        config: &RequestConfigData,
    ) -> Result<Request, RequestError> {
        let buffer_size: usize = config.get_buffer_size();
        let max_ws_frame_size: usize = config.get_max_ws_frame_size();
        let ws_read_timeout_ms: u64 = config.get_ws_read_timeout_ms();
        let max_ws_frames: usize = config.get_max_ws_frames();
        let mut dynamic_buffer: Vec<u8> = Vec::with_capacity(buffer_size);
        let mut temp_buffer: Vec<u8> = vec![0; buffer_size];
        let mut full_frame: Vec<u8> = Vec::with_capacity(max_ws_frame_size);
        let mut frame_count: usize = 0;
        let mut is_client_response: bool = false;
        let ws_read_timeout_ms: u64 = (ws_read_timeout_ms >> 1) + (ws_read_timeout_ms & 1);
        let timeout_duration: Duration = Duration::from_millis(ws_read_timeout_ms);
        loop {
            let len: usize = match timeout(
                timeout_duration,
                stream.write().await.read(&mut temp_buffer),
            )
            .await
            {
                Ok(result) => match result {
                    Ok(len) => len,
                    Err(error) => {
                        if error.kind() == ErrorKind::ConnectionReset
                            || error.kind() == ErrorKind::ConnectionAborted
                        {
                            return Err(RequestError::ClientDisconnected(HttpStatus::BadRequest));
                        }
                        return Err(RequestError::Unknown(HttpStatus::InternalServerError));
                    }
                },
                Err(_) => {
                    if !is_client_response {
                        return Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout));
                    }
                    is_client_response = false;
                    stream
                        .try_send_body(&PING_FRAME)
                        .await
                        .map_err(|_| RequestError::WriteTimeout(HttpStatus::InternalServerError))?;
                    let _ = stream.try_flush().await;
                    continue;
                }
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
                frame_count += 1;
                if frame_count > max_ws_frames {
                    return Err(RequestError::TooManyHeaders(
                        HttpStatus::RequestHeaderFieldsTooLarge,
                    ));
                }
                match frame.get_opcode() {
                    WebSocketOpcode::Close => {
                        return Err(RequestError::ClientClosedConnection(HttpStatus::BadRequest));
                    }
                    WebSocketOpcode::Ping | WebSocketOpcode::Pong => {
                        continue;
                    }
                    WebSocketOpcode::Text | WebSocketOpcode::Binary => {
                        let payload_data: &[u8] = frame.get_payload_data();
                        if payload_data.len() > max_ws_frame_size {
                            return Err(RequestError::WebSocketFrameTooLarge(
                                HttpStatus::PayloadTooLarge,
                            ));
                        }
                        if full_frame.len() + payload_data.len() > max_ws_frame_size {
                            return Err(RequestError::WebSocketFrameTooLarge(
                                HttpStatus::PayloadTooLarge,
                            ));
                        }
                        full_frame.extend_from_slice(payload_data);
                        if *frame.get_fin() {
                            let mut request: Request = self.clone();
                            request.body = full_frame;
                            return Ok(request);
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

    /// Tries to get a query parameter value by key.
    ///
    /// The key type must implement AsRef<str> conversion.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The query parameter key (implements AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `Option<RequestQuerysValue>` - The parameter value if exists.
    #[inline(always)]
    pub fn try_get_query<K>(&self, key: K) -> Option<RequestQuerysValue>
    where
        K: AsRef<str>,
    {
        self.querys.get(key.as_ref()).cloned()
    }

    /// Gets a query parameter value by key.
    ///
    /// The key type must implement AsRef<str> conversion.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The query parameter key (implements AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `RequestQuerysValue` - The parameter value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the query parameter key is not found.
    #[inline(always)]
    pub fn get_query<K>(&self, key: K) -> RequestQuerysValue
    where
        K: AsRef<str>,
    {
        self.try_get_query(key).unwrap()
    }

    /// Tries to retrieve the value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValue>` - The optional header values.
    #[inline(always)]
    pub fn try_get_header<K>(&self, key: K) -> Option<RequestHeadersValue>
    where
        K: AsRef<str>,
    {
        self.headers.get(key.as_ref()).cloned()
    }

    /// Retrieves the value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValue` - The optional header values.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header<K>(&self, key: K) -> RequestHeadersValue
    where
        K: AsRef<str>,
    {
        self.try_get_header(key).unwrap()
    }

    /// Tries to retrieve the first value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValueItem>` - The first header value if exists.
    #[inline(always)]
    pub fn try_get_header_front<K>(&self, key: K) -> Option<RequestHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.headers
            .get(key.as_ref())
            .and_then(|values| values.front().cloned())
    }

    /// Retrieves the first value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValueItem` - The first header value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_front<K>(&self, key: K) -> RequestHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_header_front(key).unwrap()
    }

    /// Tries to retrieve the last value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `Option<RequestHeadersValueItem>` - The last header value if exists.
    #[inline(always)]
    pub fn try_get_header_back<K>(&self, key: K) -> Option<RequestHeadersValueItem>
    where
        K: AsRef<str>,
    {
        self.headers
            .get(key.as_ref())
            .and_then(|values| values.back().cloned())
    }

    /// Retrieves the last value of a request header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `RequestHeadersValueItem` - The last header value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_back<K>(&self, key: K) -> RequestHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_header_back(key).unwrap()
    }

    /// Tries to retrieve the number of values for a specific header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The count of values for the header if exists.
    #[inline(always)]
    pub fn try_get_header_length<K>(&self, key: K) -> Option<usize>
    where
        K: AsRef<str>,
    {
        self.headers.get(key.as_ref()).map(|values| values.len())
    }

    /// Retrieves the number of values for a specific header.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `usize` - The count of values for the header.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_length<K>(&self, key: K) -> usize
    where
        K: AsRef<str>,
    {
        self.try_get_header_length(key).unwrap()
    }

    /// Retrieves the total number of header values across all headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all header values.
    #[inline(always)]
    pub fn get_headers_values_length(&self) -> usize {
        self.headers.values().map(|values| values.len()).sum()
    }

    /// Retrieves the number of unique headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of unique header keys.
    #[inline(always)]
    pub fn get_headers_length(&self) -> usize {
        self.headers.len()
    }

    /// Checks if a specific header exists.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key to check (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the header exists.
    #[inline(always)]
    pub fn has_header<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.headers.contains_key(key.as_ref())
    }

    /// Checks if a header contains a specific value.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key to check (must implement AsRef<str>).
    /// - `AsRef<str>` - The value to search for (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the header contains the value.
    #[inline(always)]
    pub fn has_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        if let Some(values) = self.headers.get(key.as_ref()) {
            values.contains(&value.as_ref().to_owned())
        } else {
            false
        }
    }

    /// Retrieves the body content of the request as a UTF-8 encoded string.
    ///
    /// This method uses `String::from_utf8_lossy` to convert the byte slice returned by `self.get_body()` as_ref a string.
    /// If the byte slice contains invalid UTF-8 sequences, they will be replaced with the Unicode replacement character ().
    ///
    /// # Returns
    ///
    /// - `String` - The body content as a string.
    #[inline(always)]
    pub fn get_body_string(&self) -> String {
        String::from_utf8_lossy(self.get_body()).into_owned()
    }

    /// Deserializes the body content of the request as_ref a specified type `T`.
    ///
    /// This method first retrieves the body content as a byte slice using `self.get_body()`.
    /// It then attempts to deserialize the byte slice as_ref the specified type `T` using `json_from_slice`.
    ///
    /// # Arguments
    ///
    /// - `DeserializeOwned` - The target type to deserialize as_ref (must implement DeserializeOwned).
    ///
    /// # Returns
    ///
    /// - `Result<T, serde_json::Error>` - The deserialization result.
    pub fn try_get_body_json<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(self.get_body())
    }

    /// Deserializes the body content of the request as_ref a specified type `T`.
    ///
    /// This method first retrieves the body content as a byte slice using `self.get_body()`.
    /// It then attempts to deserialize the byte slice as_ref the specified type `T` using `json_from_slice`.
    ///
    /// # Arguments
    ///
    /// - `DeserializeOwned` - The target type to deserialize as_ref (must implement DeserializeOwned).
    ///
    /// # Returns
    ///
    /// - `T` - The deserialized body content.
    ///
    /// # Panics
    ///
    /// This function will panic if the deserialization fails.
    pub fn get_body_json<T>(&self) -> T
    where
        T: DeserializeOwned,
    {
        self.try_get_body_json().unwrap()
    }

    /// Converts the request to a formatted string representation.
    ///
    /// This method provides a human-readable summary of the request, including its method,
    /// host, version, path, query parameters, headers, and body information.
    ///
    /// # Returns
    ///
    /// - `String` - The formatted request details.
    #[inline(always)]
    pub fn get_string(&self) -> String {
        let body: &Vec<u8> = self.get_body();
        let body_type: &'static str = if std::str::from_utf8(body).is_ok() {
            PLAIN
        } else {
            BINARY
        };
        format!(
            "[Request] => [method]: {}; [host]: {}; [version]: {}; [path]: {}; [querys]: {:?}; [headers]: {:?}; [body]: {} bytes {};",
            self.get_method(),
            self.get_host(),
            self.get_version(),
            self.get_path(),
            self.get_querys(),
            self.get_headers(),
            body.len(),
            body_type
        )
    }

    /// Retrieves the upgrade type from the request headers.
    ///
    /// This method looks for the `UPGRADE` header and attempts to parse its value
    /// as_ref an `UpgradeType`. If the header is missing or the value is invalid,
    /// it returns the default `UpgradeType`.
    ///
    /// # Returns
    ///
    /// - `UpgradeType` - The parsed upgrade type.
    #[inline(always)]
    pub fn get_upgrade_type(&self) -> UpgradeType {
        let upgrade_type: UpgradeType = self
            .try_get_header_back(UPGRADE)
            .and_then(|data| data.parse::<UpgradeType>().ok())
            .unwrap_or_default();
        upgrade_type
    }

    /// Checks whether the WebSocket upgrade is enabled for this request.
    ///
    /// This method determines if the `UPGRADE` header indicates a WebSocket connection.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether WebSocket upgrade is enabled.
    #[inline(always)]
    pub fn is_ws(&self) -> bool {
        self.get_upgrade_type().is_ws()
    }

    /// Checks if the current upgrade type is HTTP/2 cleartext (h2c).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is h2c.
    #[inline(always)]
    pub fn is_h2c(&self) -> bool {
        self.get_upgrade_type().is_h2c()
    }

    /// Checks if the current upgrade type is TLS (any version).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is TLS.
    #[inline(always)]
    pub fn is_tls(&self) -> bool {
        self.get_upgrade_type().is_tls()
    }

    /// Checks whether the upgrade type is unknown.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is unknown.
    #[inline(always)]
    pub fn is_unknown_upgrade(&self) -> bool {
        self.get_upgrade_type().is_unknown()
    }

    /// Checks if the HTTP version is HTTP/1.1 or higher.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/1.1 or higher.
    #[inline(always)]
    pub fn is_http1_1_or_higher(&self) -> bool {
        self.get_version().is_http1_1_or_higher()
    }

    /// Checks whether the HTTP version is HTTP/0.9.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/0.9.
    #[inline(always)]
    pub fn is_http0_9(&self) -> bool {
        self.get_version().is_http0_9()
    }

    /// Checks whether the HTTP version is HTTP/1.0.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/1.0.
    #[inline(always)]
    pub fn is_http1_0(&self) -> bool {
        self.get_version().is_http1_0()
    }

    /// Checks whether the HTTP version is HTTP/1.1.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/1.1.
    #[inline(always)]
    pub fn is_http1_1(&self) -> bool {
        self.get_version().is_http1_1()
    }

    /// Checks whether the HTTP version is HTTP/2.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/2.
    #[inline(always)]
    pub fn is_http2(&self) -> bool {
        self.get_version().is_http2()
    }

    /// Checks whether the HTTP version is HTTP/3.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/3.
    #[inline(always)]
    pub fn is_http3(&self) -> bool {
        self.get_version().is_http3()
    }

    /// Checks whether the HTTP version is unknown.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is unknown.
    #[inline(always)]
    pub fn is_unknown_version(&self) -> bool {
        self.get_version().is_unknown()
    }

    /// Checks whether the version belongs to the HTTP family.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP.
    #[inline(always)]
    pub fn is_http(&self) -> bool {
        self.get_version().is_http()
    }

    /// Checks whether the request method is GET.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is GET.
    #[inline(always)]
    pub fn is_get(&self) -> bool {
        self.get_method().is_get()
    }

    /// Checks whether the request method is POST.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is POST.
    #[inline(always)]
    pub fn is_post(&self) -> bool {
        self.get_method().is_post()
    }

    /// Checks whether the request method is PUT.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is PUT.
    #[inline(always)]
    pub fn is_put(&self) -> bool {
        self.get_method().is_put()
    }

    /// Checks whether the request method is DELETE.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is DELETE.
    #[inline(always)]
    pub fn is_delete(&self) -> bool {
        self.get_method().is_delete()
    }

    /// Checks whether the request method is PATCH.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is PATCH.
    #[inline(always)]
    pub fn is_patch(&self) -> bool {
        self.get_method().is_patch()
    }

    /// Checks whether the request method is HEAD.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is HEAD.
    #[inline(always)]
    pub fn is_head(&self) -> bool {
        self.get_method().is_head()
    }

    /// Checks whether the request method is OPTIONS.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is OPTIONS.
    #[inline(always)]
    pub fn is_options(&self) -> bool {
        self.get_method().is_options()
    }

    /// Checks whether the request method is CONNECT.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is CONNECT.
    #[inline(always)]
    pub fn is_connect(&self) -> bool {
        self.get_method().is_connect()
    }

    /// Checks whether the request method is TRACE.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is TRACE.
    #[inline(always)]
    pub fn is_trace(&self) -> bool {
        self.get_method().is_trace()
    }

    /// Checks whether the request method is UNKNOWN.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is UNKNOWN.
    #[inline(always)]
    pub fn is_unknown_method(&self) -> bool {
        self.get_method().is_unknown()
    }

    /// Determines if a keep-alive connection should be enabled for this request.
    ///
    /// This function checks the `Connection` header and the HTTP version to determine
    /// if keep-alive should be enabled. The logic is as follows:
    ///
    /// 1. If the `Connection` header exists:
    ///    - Returns `true` if the header value is "keep-alive" (case-insensitive).
    ///    - Returns `false` if the header value is "close" (case-insensitive).
    /// 2. If no `Connection` header is present:
    ///    - Returns `true` if the HTTP version is 1.1 or higher.
    ///    - Returns `false` otherwise.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether keep-alive should be enabled.
    #[inline(always)]
    pub fn is_enable_keep_alive(&self) -> bool {
        if let Some(connection_value) = self.try_get_header_back(CONNECTION) {
            if connection_value.eq_ignore_ascii_case(KEEP_ALIVE) {
                return true;
            } else if connection_value.eq_ignore_ascii_case(CLOSE) {
                return self.is_ws();
            }
        }
        self.is_http1_1_or_higher() || self.is_ws()
    }

    /// Determines if keep-alive should be disabled for this request.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether keep-alive should be disabled.
    #[inline(always)]
    pub fn is_disable_keep_alive(&self) -> bool {
        !self.is_enable_keep_alive()
    }
}
