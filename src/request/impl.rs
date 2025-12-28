use crate::*;

/// Implements the `std::error::Error` trait for `RequestError`.
impl std::error::Error for RequestError {}

/// Implements the `Display` trait for `RequestError`, allowing it to be formatted as a string.
impl Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpRead(status) => write!(f, "Http read error [{}]", status.code()),
            Self::GetTcpStream(status) => write!(f, "Failed to get tcp stream [{}]", status.code()),
            Self::GetTlsStream(status) => write!(f, "Failed to get tls stream [{}]", status.code()),
            Self::ReadConnection(status) => write!(f, "Connection read error [{}]", status.code()),
            Self::RequestAborted(status) => write!(f, "Request aborted [{}]", status.code()),
            Self::TlsStreamConnect(status) => {
                write!(f, "Tls stream connection error [{}]", status.code())
            }
            Self::NeedOpenRedirect(status) => {
                write!(f, "Open redirect required [{}]", status.code())
            }
            Self::MaxRedirectTimes(status) => {
                write!(f, "Exceeded maximum redirect attempts [{}]", status.code())
            }
            Self::MethodsNotSupport(status) => {
                write!(f, "Http method not supported [{}]", status.code())
            }
            Self::RedirectInvalidUrl(status) => {
                write!(f, "Invalid redirect url [{}]", status.code())
            }
            Self::ClientDisconnected(status) => {
                write!(f, "Client disconnected [{}]", status.code())
            }
            Self::RedirectUrlDeadLoop(status) => {
                write!(f, "Redirect url dead loop detected [{}]", status.code())
            }
            Self::ClientClosedConnection(status) => {
                write!(f, "Client closed connection [{}]", status.code())
            }
            Self::IncompleteWebSocketFrame(status) => write!(
                f,
                "WebSocket connection closed before a complete frame was received [{}]",
                status.code()
            ),
            Self::RequestTooLong(status) => write!(f, "Request line too long [{}]", status.code()),
            Self::PathTooLong(status) => write!(f, "Path too long [{}]", status.code()),
            Self::QueryTooLong(status) => write!(f, "Query string too long [{}]", status.code()),
            Self::HeaderLineTooLong(status) => {
                write!(f, "Header line too long [{}]", status.code())
            }
            Self::TooManyHeaders(status) => write!(f, "Too many headers [{}]", status.code()),
            Self::HeaderKeyTooLong(status) => write!(f, "Header key too long [{}]", status.code()),
            Self::HeaderValueTooLong(status) => {
                write!(f, "Header value too long [{}]", status.code())
            }
            Self::ContentLengthTooLarge(status) => {
                write!(f, "Content length too large [{}]", status.code())
            }
            Self::InvalidContentLength(status) => {
                write!(f, "Invalid content length [{}]", status.code())
            }
            Self::Unknown(status) => write!(f, "Unknown error occurred [{}]", status.code()),
            Self::InvalidUrlScheme(status) => write!(f, "Invalid URL scheme [{}]", status.code()),
            Self::InvalidUrlHost(status) => write!(f, "Invalid URL host [{}]", status.code()),
            Self::InvalidUrlPort(status) => write!(f, "Invalid URL port [{}]", status.code()),
            Self::InvalidUrlPath(status) => write!(f, "Invalid URL path [{}]", status.code()),
            Self::InvalidUrlQuery(status) => write!(f, "Invalid URL query [{}]", status.code()),
            Self::InvalidUrlFragment(status) => {
                write!(f, "Invalid URL fragment [{}]", status.code())
            }
            Self::ReadTimeoutNotSet(status) => {
                write!(f, "Failed to set read timeout [{}]", status.code())
            }
            Self::WriteTimeoutNotSet(status) => {
                write!(f, "Failed to set write timeout [{}]", status.code())
            }
            Self::TcpConnectionFailed(status) => {
                write!(f, "Tcp connection failed [{}]", status.code())
            }
            Self::TlsHandshakeFailed(status) => {
                write!(f, "Tls handshake failed [{}]", status.code())
            }
            Self::TlsCertificateInvalid(status) => {
                write!(f, "Tls certificate invalid [{}]", status.code())
            }
            Self::WebSocketFrameTooLarge(status) => {
                write!(f, "WebSocket frame too large [{}]", status.code())
            }
            Self::WebSocketOpcodeUnsupported(status) => {
                write!(f, "WebSocket opcode unsupported [{}]", status.code())
            }
            Self::WebSocketMaskMissing(status) => {
                write!(f, "WebSocket mask missing [{}]", status.code())
            }
            Self::WebSocketPayloadCorrupted(status) => {
                write!(f, "WebSocket payload corrupted [{}]", status.code())
            }
            Self::WebSocketInvalidUtf8(status) => {
                write!(f, "WebSocket invalid UTF-8 [{}]", status.code())
            }
            Self::WebSocketInvalidCloseCode(status) => {
                write!(f, "WebSocket invalid close code [{}]", status.code())
            }
            Self::WebSocketInvalidExtension(status) => {
                write!(f, "WebSocket invalid extension [{}]", status.code())
            }
            Self::HttpRequestPartsInsufficient(status) => {
                write!(f, "HTTP request parts insufficient [{}]", status.code())
            }
        }
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
            Self::Unknown(status) => *status,
            Self::InvalidUrlScheme(status) => *status,
            Self::InvalidUrlHost(status) => *status,
            Self::InvalidUrlPort(status) => *status,
            Self::InvalidUrlPath(status) => *status,
            Self::InvalidUrlQuery(status) => *status,
            Self::InvalidUrlFragment(status) => *status,
            Self::ReadTimeoutNotSet(status) => *status,
            Self::WriteTimeoutNotSet(status) => *status,
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

impl Default for RequestConfig {
    /// Creates a `RequestConfig` with secure default values.
    ///
    /// # Returns
    ///
    /// - `RequestConfig` - A new config instance with secure defaults.
    #[inline(always)]
    fn default() -> Self {
        Self {
            min_buffer_size: KB_2,
            buffer_size: DEFAULT_BUFFER_SIZE,
            max_request_line_length: KB_16,
            max_path_length: KB_8,
            max_query_length: KB_16,
            max_header_line_length: KB_16,
            max_header_count: 200,
            max_header_key_length: KB_512,
            max_header_value_length: KB_16,
            max_body_size: MB_100,
            max_ws_frame_size: MB_10,
            max_ws_frames: 5000,
            http_read_timeout_ms: 60000,
            ws_read_timeout_ms: 60000,
        }
    }
}

impl RequestConfig {
    /// Creates a new `RequestConfig` with default values.
    ///
    /// # Returns
    ///
    /// - `RequestConfig` - A new config instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a config optimized for high-security environments.
    ///
    /// This configuration uses more restrictive limits to provide
    /// maximum protection against various attacks.
    ///
    /// # Returns
    ///
    /// - `RequestConfig` - A new config with high-security settings.
    pub fn high_security() -> Self {
        Self {
            min_buffer_size: KB_512,
            buffer_size: KB_4,
            max_request_line_length: KB_2,
            max_path_length: KB_1,
            max_query_length: KB_2,
            max_header_line_length: KB_2,
            max_header_count: 50,
            max_header_key_length: KB_128,
            max_header_value_length: KB_2,
            max_body_size: MB_1,
            max_ws_frame_size: KB_256,
            max_ws_frames: 100,
            http_read_timeout_ms: 10000,
            ws_read_timeout_ms: 10000,
        }
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
    /// Creates a new instance of `Request`.
    ///
    /// # Returns
    ///
    /// - `Request` - A new request instance with default values.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses an HTTP request from a buffered TCP stream reader.
    ///
    /// Reads request line, headers and body from the stream and constructs a Request object.
    ///
    /// # Arguments
    ///
    /// - `&mut BufReader<&mut TcpStream>` - The buffered TCP stream reader.
    /// - `&RequestConfig` - Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed request or an error.
    pub async fn http_from_reader(
        reader: &mut BufReader<&mut TcpStream>,
        config: &RequestConfig,
    ) -> Result<Request, RequestError> {
        let buffer_size: usize = config.buffer_size.max(config.min_buffer_size);
        let mut request_line: String = String::with_capacity(buffer_size);
        let timeout_duration: Duration = Duration::from_millis(config.http_read_timeout_ms);
        let bytes_read: usize = timeout(
            timeout_duration,
            AsyncBufReadExt::read_line(reader, &mut request_line),
        )
        .await
        .map_err(|_| RequestError::ReadTimeoutNotSet(HttpStatus::RequestTimeout))?
        .map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
        if bytes_read > config.max_request_line_length {
            return Err(RequestError::RequestTooLong(HttpStatus::BadRequest));
        }
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        let parts_len: usize = parts.len();
        if parts_len < 3 {
            return Err(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ));
        }
        let full_path: &str = parts[1];
        if full_path.len() > config.max_path_length {
            return Err(RequestError::PathTooLong(HttpStatus::URITooLong));
        }
        let method: RequestMethod = parts[0]
            .parse::<RequestMethod>()
            .unwrap_or(Method::Unknown(parts[0].to_string()));
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
        if query_string.len() > config.max_query_length {
            return Err(RequestError::QueryTooLong(HttpStatus::URITooLong));
        }
        let querys: RequestQuerys = Self::parse_querys(&query_string);
        let path: RequestPath = if let Some(i) = query_index.or(hash_index) {
            full_path[..i].to_owned()
        } else {
            full_path.to_owned()
        };
        let mut headers: RequestHeaders = hash_map_xx_hash3_64();
        let mut host: RequestHost = String::new();
        let mut content_length: usize = 0;
        let mut header_count: usize = 0;
        loop {
            let mut header_line: String = String::with_capacity(buffer_size);
            let timeout_duration: Duration = Duration::from_millis(config.http_read_timeout_ms);
            let bytes_read: usize = timeout(
                timeout_duration,
                AsyncBufReadExt::read_line(reader, &mut header_line),
            )
            .await
            .map_err(|_| RequestError::ReadTimeoutNotSet(HttpStatus::RequestTimeout))?
            .map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
            if bytes_read > config.max_header_line_length {
                return Err(RequestError::HeaderLineTooLong(
                    HttpStatus::RequestHeaderFieldsTooLarge,
                ));
            }
            let header_line: &str = header_line.trim();
            if header_line.is_empty() {
                break;
            }
            header_count += 1;
            if header_count > config.max_header_count {
                return Err(RequestError::TooManyHeaders(
                    HttpStatus::RequestHeaderFieldsTooLarge,
                ));
            }
            if let Some((key_part, value_part)) = header_line.split_once(COLON) {
                let key: String = key_part.trim().to_ascii_lowercase();
                if key.is_empty() {
                    continue;
                }
                if key.len() > config.max_header_key_length {
                    return Err(RequestError::HeaderKeyTooLong(
                        HttpStatus::RequestHeaderFieldsTooLarge,
                    ));
                }
                let value: String = value_part.trim().to_string();
                if value.len() > config.max_header_value_length {
                    return Err(RequestError::HeaderValueTooLong(
                        HttpStatus::RequestHeaderFieldsTooLarge,
                    ));
                }
                if key == HOST {
                    host = value.clone();
                } else if key == CONTENT_LENGTH {
                    match value.parse::<usize>() {
                        Ok(length) => {
                            if length > config.max_body_size {
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
        let mut body: RequestBody = Vec::with_capacity(content_length);
        if content_length > 0 {
            body.resize(content_length, 0);
            let timeout_duration: Duration = Duration::from_millis(config.http_read_timeout_ms);
            timeout(
                timeout_duration,
                AsyncReadExt::read_exact(reader, &mut body),
            )
            .await
            .map_err(|_| RequestError::ReadTimeoutNotSet(HttpStatus::RequestTimeout))?
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
    }

    /// Parses an HTTP request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `http_from_reader`.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLock<TcpStream>` - The TCP stream to read from.
    /// - `&RequestConfig` - Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed request or an error.
    pub async fn http_from_stream(
        stream: &ArcRwLockStream,
        config: &RequestConfig,
    ) -> Result<Request, RequestError> {
        let mut buf_stream: RwLockWriteGuard<'_, TcpStream> = stream.write().await;
        let mut reader: BufReader<&mut TcpStream> = BufReader::new(&mut buf_stream);
        Self::http_from_reader(&mut reader, config).await
    }

    /// Parses a WebSocket request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `ws_from_reader`.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLock<TcpStream>` - The TCP stream to read from.
    /// - `&RequestConfig` - Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed WebSocket request or an error.
    pub async fn ws_from_stream(
        &mut self,
        stream: &ArcRwLockStream,
        config: &RequestConfig,
    ) -> Result<Request, RequestError> {
        let mut buf_stream: RwLockWriteGuard<'_, TcpStream> = stream.write().await;
        let mut reader: BufReader<&mut TcpStream> = BufReader::new(&mut buf_stream);
        self.ws_from_reader(&mut reader, config).await
    }

    /// Parses a WebSocket request from a buffered TCP stream.
    ///
    /// Handles WebSocket frames including text, binary, ping, pong and close frames.
    /// Assembles the request body from frame payload data.
    ///
    /// # Arguments
    ///
    /// - `&mut BufReader<&mut TcpStream>` - The buffered TCP stream reader.
    /// - `&RequestConfig` - Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>` - The parsed WebSocket request or an error.
    pub async fn ws_from_reader(
        &mut self,
        reader: &mut BufReader<&mut TcpStream>,
        config: &RequestConfig,
    ) -> Result<Request, RequestError> {
        let buffer_size: usize = config.buffer_size.max(config.min_buffer_size);
        let mut dynamic_buffer: Vec<u8> = Vec::with_capacity(buffer_size);
        let temp_buffer_size: usize = buffer_size;
        let mut temp_buffer: Vec<u8> = vec![0; temp_buffer_size];
        let mut full_frame: Vec<u8> = Vec::with_capacity(config.max_ws_frame_size);
        let mut frame_count: usize = 0;
        loop {
            let timeout_duration: Duration = Duration::from_millis(config.ws_read_timeout_ms);
            let len: usize = match timeout(timeout_duration, reader.read(&mut temp_buffer)).await {
                Ok(result) => match result {
                    Ok(len) => len,
                    Err(err) => {
                        if err.kind() == ErrorKind::ConnectionReset
                            || err.kind() == ErrorKind::ConnectionAborted
                        {
                            return Err(RequestError::ClientDisconnected(HttpStatus::BadRequest));
                        }
                        return Err(RequestError::Unknown(HttpStatus::InternalServerError));
                    }
                },
                Err(_) => {
                    return Err(RequestError::ReadTimeoutNotSet(HttpStatus::RequestTimeout));
                }
            };
            if len == 0 {
                return Err(RequestError::IncompleteWebSocketFrame(
                    HttpStatus::BadRequest,
                ));
            }
            dynamic_buffer.extend_from_slice(&temp_buffer[..len]);
            while let Some((frame, consumed)) = WebSocketFrame::decode_ws_frame(&dynamic_buffer) {
                dynamic_buffer.drain(0..consumed);
                frame_count += 1;
                if frame_count > config.max_ws_frames {
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
                        if payload_data.len() > config.max_ws_frame_size {
                            return Err(RequestError::WebSocketFrameTooLarge(
                                HttpStatus::PayloadTooLarge,
                            ));
                        }
                        if full_frame.len() + payload_data.len() > config.max_ws_frame_size {
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
