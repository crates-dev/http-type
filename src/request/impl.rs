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

/// Converts an I/O error to a `RequestError`.
///
/// Maps connection reset and aborted errors to `ClientDisconnected`,
/// all other I/O errors are mapped to `ReadConnection`.
impl From<std::io::Error> for RequestError {
    /// Converts an I/O error to a `RequestError`.
    ///
    /// # Arguments
    ///
    /// - `std::io::Error`: The I/O error to convert.
    ///
    /// # Returns
    ///
    /// - `RequestError`: The corresponding request error.
    #[inline(always)]
    fn from(error: std::io::Error) -> Self {
        let kind: ErrorKind = error.kind();
        if kind == ErrorKind::ConnectionReset || kind == ErrorKind::ConnectionAborted {
            return RequestError::ClientDisconnected(HttpStatus::BadRequest);
        }
        RequestError::ReadConnection(HttpStatus::BadRequest)
    }
}

/// Converts a timeout elapsed error to a `RequestError`.
///
/// Maps timeout errors to `ReadTimeout` with `HttpStatus::RequestTimeout`.
impl From<Elapsed> for RequestError {
    /// Converts a timeout elapsed error to a `RequestError`.
    ///
    /// # Arguments
    ///
    /// - `Elapsed`: The elapsed error to convert.
    ///
    /// # Returns
    ///
    /// - `RequestError`: The corresponding request error as `ReadTimeout`.
    #[inline(always)]
    fn from(_: Elapsed) -> Self {
        RequestError::ReadTimeout(HttpStatus::RequestTimeout)
    }
}

/// Converts a parse int error to a `RequestError`.
///
/// Maps parse int errors to `InvalidContentLength` with `HttpStatus::BadRequest`.
impl From<ParseIntError> for RequestError {
    /// Converts a parse int error to a `RequestError`.
    ///
    /// # Arguments
    ///
    /// - `ParseIntError`: The parse error to convert.
    ///
    /// # Returns
    ///
    /// - `RequestError`: The corresponding request error as `InvalidContentLength`.
    #[inline(always)]
    fn from(_: ParseIntError) -> Self {
        RequestError::InvalidContentLength(HttpStatus::BadRequest)
    }
}

/// Converts a response error to a `RequestError`.
///
/// Maps response errors to `WriteTimeout` with `HttpStatus::InternalServerError`.
impl From<ResponseError> for RequestError {
    /// Converts a response error to a `RequestError`.
    ///
    /// # Arguments
    ///
    /// - `ResponseError`: The response error to convert.
    ///
    /// # Returns
    ///
    /// - `RequestError`: The corresponding request error as `WriteTimeout`.
    #[inline(always)]
    fn from(_: ResponseError) -> Self {
        RequestError::WriteTimeout(HttpStatus::InternalServerError)
    }
}

impl RequestError {
    /// Gets the HTTP status associated with this error.
    ///
    /// Returns the HttpStatus enum variant that corresponds to this error.
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
    /// # Returns
    ///
    /// - `ResponseStatusCode` - The numeric HTTP status code.
    pub fn get_http_status_code(&self) -> ResponseStatusCode {
        self.get_http_status().code()
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
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            max_path_size: DEFAULT_MAX_PATH_SIZE,
            max_header_count: DEFAULT_MAX_HEADER_COUNT,
            max_header_key_size: DEFAULT_MAX_HEADER_KEY_SIZE,
            max_header_value_size: DEFAULT_MAX_HEADER_VALUE_SIZE,
            max_body_size: DEFAULT_MAX_BODY_SIZE,
            read_timeout_ms: DEFAULT_READ_TIMEOUT_MS,
        }
    }
}

impl RequestConfig {
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
        serde_json::from_str(json.as_ref())
    }

    /// Creates a new `RequestConfig` with low-security settings.
    ///
    /// This constructor initializes the configuration with less restrictive limits
    /// for environments where higher limits are needed.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestConfig` instance with low-security settings.
    #[inline(always)]
    pub fn low_security() -> Self {
        Self {
            buffer_size: DEFAULT_LOW_SECURITY_BUFFER_SIZE,
            max_path_size: DEFAULT_LOW_SECURITY_MAX_PATH_SIZE,
            max_header_count: DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT,
            max_header_key_size: DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE,
            max_header_value_size: DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE,
            max_body_size: DEFAULT_LOW_SECURITY_MAX_BODY_SIZE,
            read_timeout_ms: DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS,
        }
    }

    /// Creates a new `RequestConfig` with high-security settings.
    ///
    /// This constructor initializes the configuration with more restrictive limits
    /// to provide maximum protection against various attacks in high-risk environments.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestConfig` instance with high-security settings.
    #[inline(always)]
    pub fn high_security() -> Self {
        Self {
            buffer_size: DEFAULT_HIGH_SECURITY_BUFFER_SIZE,
            max_path_size: DEFAULT_HIGH_SECURITY_MAX_PATH_SIZE,
            max_header_count: DEFAULT_HIGH_SECURITY_MAX_HEADER_COUNT,
            max_header_key_size: DEFAULT_HIGH_SECURITY_MAX_HEADER_KEY_SIZE,
            max_header_value_size: DEFAULT_HIGH_SECURITY_MAX_HEADER_VALUE_SIZE,
            max_body_size: DEFAULT_HIGH_SECURITY_MAX_BODY_SIZE,
            read_timeout_ms: DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS,
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

impl Http {
    /// Parses the first line of HTTP request into method, path, and version components.
    ///
    /// # Arguments
    ///
    /// - `&str`: The first line string of HTTP request to parse.
    ///
    /// # Returns
    ///
    /// - `Result<(RequestMethod, &str, RequestVersion), RequestError>`: A tuple containing:
    ///   - The parsed HTTP method
    ///   - The full path string
    ///   - The parsed HTTP version
    ///   - Or an error if parsing fails
    #[inline(always)]
    fn parse_first_line(line: &str) -> Result<(RequestMethod, &str, RequestVersion), RequestError> {
        let mut parts: SplitWhitespace<'_> = line.split_whitespace();
        let method_str: &str = parts
            .next()
            .ok_or(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ))?;
        let full_path: &str = parts
            .next()
            .ok_or(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ))?;
        let version_str: &str = parts
            .next()
            .ok_or(RequestError::HttpRequestPartsInsufficient(
                HttpStatus::BadRequest,
            ))?;
        let method: RequestMethod = method_str
            .parse::<RequestMethod>()
            .unwrap_or_else(|_| Method::Unknown(method_str.to_string()));
        let version: RequestVersion = version_str
            .parse::<RequestVersion>()
            .unwrap_or_else(|_| RequestVersion::Unknown(version_str.to_string()));
        Ok((method, full_path, version))
    }

    /// Validates the path length against the maximum allowed size.
    ///
    /// # Arguments
    ///
    /// - `&str`: The path string to check.
    /// - `usize`: The maximum allowed path size.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>`: Ok if valid, or an error if the path is too long.
    #[inline(always)]
    fn check_path_size(path: &str, max_size: usize) -> Result<(), RequestError> {
        if path.len() > max_size && max_size != DEFAULT_LOW_SECURITY_MAX_PATH_SIZE {
            return Err(RequestError::PathTooLong(HttpStatus::URITooLong));
        }
        Ok(())
    }

    /// Parses the query string from the full path.
    ///
    /// Handles both query parameters (after `?`) and hash fragments (after `#`),
    /// ensuring proper parsing when both are present.
    ///
    /// # Arguments
    ///
    /// - `&str`: The full path string containing the query.
    /// - `Option<usize>`: The index of the query separator (`?`), if present.
    /// - `Option<usize>`: The index of the hash separator (`#`), if present.
    ///
    /// # Returns
    ///
    /// - `&str`: The parsed query string slice, or empty string if no query.
    #[inline(always)]
    fn get_query_slice<'a>(
        path: &'a str,
        query_index: Option<usize>,
        hash_index: Option<usize>,
    ) -> &'a str {
        query_index.map_or(EMPTY_STR, |i: usize| {
            let temp: &'a str = &path[i + 1..];
            match hash_index {
                None => temp,
                Some(hash_idx) if hash_idx <= i => temp,
                Some(hash_idx) => &temp[..hash_idx - i - 1],
            }
        })
    }

    /// Parses the request path without query string or hash fragment.
    ///
    /// # Arguments
    ///
    /// - `&str`: The full path string.
    /// - `Option<usize>`: The index of the query separator (`?`), if present.
    /// - `Option<usize>`: The index of the hash separator (`#`), if present.
    ///
    /// # Returns
    ///
    /// - `RequestPath`: The request path without query or hash.
    #[inline(always)]
    fn parse_path(
        path: &str,
        query_index: Option<usize>,
        hash_index: Option<usize>,
    ) -> RequestPath {
        match query_index.or(hash_index) {
            Some(i) => path[..i].to_owned(),
            None => path.to_owned(),
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
    /// - `RequestQuerys` - The parsed query parameters.
    #[inline(always)]
    fn parse_querys(query: &str) -> RequestQuerys {
        let estimated_capacity: usize = query.matches(AND).count() + 1;
        let mut query_map: RequestQuerys = HashMapXxHash3_64::with_capacity_and_hasher(
            estimated_capacity,
            BuildHasherDefault::default(),
        );
        for pair in query.split(AND) {
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

    /// Checks if the header count exceeds the maximum allowed.
    ///
    /// # Arguments
    ///
    /// - `usize`: The current number of headers parsed.
    /// - `usize`: The maximum allowed number of headers.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>`: Returns an error if the limit is exceeded and not in low security mode.
    #[inline(always)]
    fn check_header_count(count: usize, max_count: usize) -> Result<(), RequestError> {
        if count > max_count && max_count != DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT {
            return Err(RequestError::TooManyHeaders(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        Ok(())
    }

    /// Checks if a header key exceeds the maximum allowed length.
    ///
    /// # Arguments
    ///
    /// - `&str`: The header key to check.
    /// - `usize`: The maximum allowed length for a header key.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>`: Returns an error if the limit is exceeded and not in low security mode.
    #[inline(always)]
    fn check_header_key_size(key: &str, max_size: usize) -> Result<(), RequestError> {
        if key.len() > max_size && max_size != DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE {
            return Err(RequestError::HeaderKeyTooLong(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        Ok(())
    }

    /// Checks if a header value exceeds the maximum allowed length.
    ///
    /// # Arguments
    ///
    /// - `&str`: The header value to check.
    /// - `usize`: The maximum allowed length for a header value.
    ///
    /// # Returns
    ///
    /// - `Result<(), RequestError>`: Returns an error if the limit is exceeded and not in low security mode.
    #[inline(always)]
    fn check_header_value_size(value: &str, max_size: usize) -> Result<(), RequestError> {
        if value.len() > max_size && max_size != DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE {
            return Err(RequestError::HeaderValueTooLong(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        Ok(())
    }

    /// Parses the Content-Length header value and checks it against max body size.
    ///
    /// # Arguments
    ///
    /// - `&str`: The Content-Length header value string.
    /// - `usize`: The maximum allowed body size.
    ///
    /// # Returns
    ///
    /// - `Result<usize, RequestError>`: The parsed content length or an error.
    #[inline(always)]
    fn check_body_size(value: &str, max_size: usize) -> Result<usize, RequestError> {
        let length: usize = value.parse::<usize>()?;
        if length > max_size && max_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE {
            return Err(RequestError::ContentLengthTooLarge(
                HttpStatus::PayloadTooLarge,
            ));
        }
        Ok(length)
    }

    /// Parses HTTP headers from a buffered reader.
    ///
    /// This method reads header lines from the provided buffered reader until an empty line
    /// is encountered, which indicates the end of headers. It checks header count, length,
    /// and content according to the provided configuration.
    ///
    /// # Arguments
    ///
    /// - `&mut AsyncBufReadExt + Unpin`: A mutable reference to a buffered reader implementing `AsyncBufReadExt`.
    /// - `&RequestConfig`: Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<(RequestHeaders, RequestHost, usize), RequestError>`: A tuple containing:
    ///   - The parsed headers as a hash map
    ///   - The host value parsed from the Host header
    ///   - The content length parsed from the Content-Length header
    ///   - Or an error if parsing fails
    async fn parse_headers<R>(
        reader: &mut R,
        config: &RequestConfig,
    ) -> Result<(RequestHeaders, RequestHost, usize), RequestError>
    where
        R: AsyncBufReadExt + Unpin,
    {
        let buffer_size: usize = config.get_buffer_size();
        let max_header_count: usize = config.get_max_header_count();
        let max_header_key_size: usize = config.get_max_header_key_size();
        let max_header_value_size: usize = config.get_max_header_value_size();
        let max_body_size: usize = config.get_max_body_size();
        let mut headers: RequestHeaders =
            HashMapXxHash3_64::with_capacity_and_hasher(B_16, BuildHasherDefault::default());
        let mut host: RequestHost = String::new();
        let mut content_size: usize = 0;
        let mut header_count: usize = 0;
        let mut header_line_buffer: String = String::with_capacity(buffer_size);
        loop {
            header_line_buffer.clear();
            AsyncBufReadExt::read_line(reader, &mut header_line_buffer).await?;
            let header_line: &str = header_line_buffer.trim();
            if header_line.is_empty() {
                break;
            }
            header_count += 1;
            Self::check_header_count(header_count, max_header_count)?;
            let (key_part, value_part): (&str, &str) = match header_line.split_once(COLON) {
                Some(parts) => parts,
                None => continue,
            };
            let key_trimmed: &str = key_part.trim();
            if key_trimmed.is_empty() {
                continue;
            }
            let key: String = key_trimmed.to_ascii_lowercase();
            Self::check_header_key_size(&key, max_header_key_size)?;
            let value: String = value_part.trim().to_string();
            Self::check_header_value_size(&value, max_header_value_size)?;
            match key.as_str() {
                HOST => host = value.clone(),
                CONTENT_LENGTH => {
                    content_size = Self::check_body_size(&value, max_body_size)?;
                }
                _ => {}
            }
            headers.entry(key).or_default().push_back(value);
        }
        Ok((headers, host, content_size))
    }

    /// Reads the request body from the buffered reader.
    ///
    /// # Arguments
    ///
    /// - `&mut BufReader<&mut TcpStream>`: The buffered reader to read from.
    /// - `usize`: The expected content size.
    ///
    /// # Returns
    ///
    /// - `Result<RequestBody, RequestError>`: The body bytes or an error.
    #[inline(always)]
    async fn parse_body(
        reader: &mut BufReader<&mut TcpStream>,
        content_size: usize,
    ) -> Result<RequestBody, RequestError> {
        let mut body: RequestBody = Vec::with_capacity(content_size);
        if content_size > 0 {
            body.resize(content_size, 0);
            AsyncReadExt::read_exact(reader, &mut body).await?;
        }
        Ok(body)
    }

    /// Parses the HTTP request content from the stream.
    ///
    /// This is an internal helper function that performs the actual parsing.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLock<TcpStream>`: The TCP stream to read from.
    /// - `&RequestConfig`: Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>`: The parsed request or an error.
    async fn parse_from_stream(
        stream: &ArcRwLockStream,
        config: &RequestConfig,
    ) -> Result<Request, RequestError> {
        let buffer_size: usize = config.get_buffer_size();
        let max_path_size: usize = config.get_max_path_size();
        let mut buf_stream: RwLockWriteGuard<'_, TcpStream> = stream.write().await;
        let reader: &mut BufReader<&mut TcpStream> =
            &mut BufReader::with_capacity(buffer_size, &mut buf_stream);
        let mut line: String = String::with_capacity(buffer_size);
        AsyncBufReadExt::read_line(reader, &mut line).await?;
        let (method, path, version): (RequestMethod, &str, RequestVersion) =
            Self::parse_first_line(&line)?;
        Self::check_path_size(path, max_path_size)?;
        let hash_index: Option<usize> = path.find(HASH);
        let query_index: Option<usize> = path.find(QUERY);
        let query_slice: &str = Self::get_query_slice(path, query_index, hash_index);
        let querys: RequestQuerys = Self::parse_querys(query_slice);
        let path: RequestPath = Self::parse_path(path, query_index, hash_index);
        let (headers, host, content_size): (RequestHeaders, RequestHost, usize) =
            Self::parse_headers(reader, config).await?;
        let body: RequestBody = Self::parse_body(reader, content_size).await?;
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
}

impl Ws {
    /// Reads data from the stream with optional timeout handling.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream`: The TCP stream to read from.
    /// - `&mut [u8]`: The buffer to read data into.
    /// - `Option<Duration>`: The optional timeout duration. If Some, timeout is applied; if None, no timeout.
    /// - `&mut bool`: Mutable reference to track if we got a client response.
    ///
    /// # Returns
    ///
    /// - `Result<Option<usize>, RequestError>`: The number of bytes read, None for timeout/ping, or an error.
    async fn read(
        stream: &ArcRwLockStream,
        buffer: &mut [u8],
        duration_opt: Option<Duration>,
        is_client_response: &mut bool,
    ) -> Result<Option<usize>, RequestError> {
        if let Some(duration) = duration_opt {
            return match timeout(duration, stream.write().await.read(buffer)).await {
                Ok(result) => match result {
                    Ok(len) => Ok(Some(len)),
                    Err(error) => Err(error.into()),
                },
                Err(error) => {
                    if !*is_client_response {
                        return Err(error.into());
                    }
                    *is_client_response = false;
                    stream.try_send_body(&PING_FRAME).await?;
                    Ok(None)
                }
            };
        }
        match stream.write().await.read(buffer).await {
            Ok(len) => Ok(Some(len)),
            Err(error) => Err(error.into()),
        }
    }

    /// Handles a decoded WebSocket Text or Binary frame and accumulates payload data.
    ///
    /// # Arguments
    ///
    /// - `&Request`: The request to update on completion.
    /// - `&WebSocketFrame`: The decoded WebSocket frame.
    /// - `&mut Vec<u8>`: The accumulated frame data.
    ///
    /// # Returns
    ///
    /// - `Result<Option<Request>, RequestError>`: Some(request) if frame is complete, None to continue, or error.
    #[inline(always)]
    fn parse_frame(
        request: &Request,
        frame: &WebSocketFrame,
        full_frame: &mut Vec<u8>,
    ) -> Result<Option<Request>, RequestError> {
        let payload_data: &[u8] = frame.get_payload_data();
        full_frame.extend_from_slice(payload_data);
        if *frame.get_fin() {
            let mut result: Request = request.clone();
            result.body = full_frame.clone();
            return Ok(Some(result));
        }
        Ok(None)
    }
}

impl Request {
    /// Parses an HTTP request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `http_from_reader`.
    /// If the timeout is DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS, no timeout is applied.
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
        let timeout_ms: u64 = config.get_read_timeout_ms();
        if timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
            return Http::parse_from_stream(stream, config).await;
        }
        let duration: Duration = Duration::from_millis(timeout_ms);
        timeout(duration, Http::parse_from_stream(stream, config)).await?
    }

    /// Parses a WebSocket request from a TCP stream.
    ///
    /// Wraps the stream in a buffered reader and delegates to `ws_from_reader`.
    /// If the timeout is DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS, no timeout is applied.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLock<TcpStream>`: The TCP stream to read from.
    /// - `&RequestConfig`: Configuration for security limits and buffer settings.
    ///
    /// # Returns
    ///
    /// - `Result<Request, RequestError>`: The parsed WebSocket request or an error.
    pub async fn ws_from_stream(
        &self,
        stream: &ArcRwLockStream,
        config: &RequestConfig,
    ) -> Result<Request, RequestError> {
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
            let len: usize = match Ws::read(
                stream,
                &mut temp_buffer,
                duration_opt,
                &mut is_client_response,
            )
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
                        match Ws::parse_frame(self, &frame, &mut full_frame) {
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
    pub fn try_get_header_size<K>(&self, key: K) -> Option<usize>
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
    pub fn get_header_size<K>(&self, key: K) -> usize
    where
        K: AsRef<str>,
    {
        self.try_get_header_size(key).unwrap()
    }

    /// Retrieves the total number of header values across all headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all header values.
    #[inline(always)]
    pub fn get_headers_values_size(&self) -> usize {
        self.headers.values().map(|values| values.len()).sum()
    }

    /// Retrieves the number of unique headers.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of unique header keys.
    #[inline(always)]
    pub fn get_headers_size(&self) -> usize {
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
            values.iter().any(|v| v == value.as_ref())
        } else {
            false
        }
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
        self.try_get_header_back(UPGRADE)
            .and_then(|data| data.parse::<UpgradeType>().ok())
            .unwrap_or_default()
    }

    /// Retrieves the body content of the request as a UTF-8 encoded string.
    ///
    /// This method uses `String::from_utf8_lossy` to convert the byte slice returned by `self.get_body()` as a string.
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn get_body_json<T>(&self) -> T
    where
        T: DeserializeOwned,
    {
        self.try_get_body_json().unwrap()
    }

    /// Checks whether the request method is GET.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is GET.
    #[inline(always)]
    pub fn is_get_method(&self) -> bool {
        self.get_method().is_get()
    }

    /// Checks whether the request method is POST.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is POST.
    #[inline(always)]
    pub fn is_post_method(&self) -> bool {
        self.get_method().is_post()
    }

    /// Checks whether the request method is PUT.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is PUT.
    #[inline(always)]
    pub fn is_put_method(&self) -> bool {
        self.get_method().is_put()
    }

    /// Checks whether the request method is DELETE.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is DELETE.
    #[inline(always)]
    pub fn is_delete_method(&self) -> bool {
        self.get_method().is_delete()
    }

    /// Checks whether the request method is PATCH.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is PATCH.
    #[inline(always)]
    pub fn is_patch_method(&self) -> bool {
        self.get_method().is_patch()
    }

    /// Checks whether the request method is HEAD.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is HEAD.
    #[inline(always)]
    pub fn is_head_method(&self) -> bool {
        self.get_method().is_head()
    }

    /// Checks whether the request method is OPTIONS.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is OPTIONS.
    #[inline(always)]
    pub fn is_options_method(&self) -> bool {
        self.get_method().is_options()
    }

    /// Checks whether the request method is CONNECT.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is CONNECT.
    #[inline(always)]
    pub fn is_connect_method(&self) -> bool {
        self.get_method().is_connect()
    }

    /// Checks whether the request method is TRACE.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the method is TRACE.
    #[inline(always)]
    pub fn is_trace_method(&self) -> bool {
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

    /// Checks whether the HTTP version is HTTP/0.9.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/0.9.
    #[inline(always)]
    pub fn is_http0_9_version(&self) -> bool {
        self.get_version().is_http0_9()
    }

    /// Checks whether the HTTP version is HTTP/1.0.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/1.0.
    #[inline(always)]
    pub fn is_http1_0_version(&self) -> bool {
        self.get_version().is_http1_0()
    }

    /// Checks whether the HTTP version is HTTP/1.1.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/1.1.
    #[inline(always)]
    pub fn is_http1_1_version(&self) -> bool {
        self.get_version().is_http1_1()
    }

    /// Checks whether the HTTP version is HTTP/2.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/2.
    #[inline(always)]
    pub fn is_http2_version(&self) -> bool {
        self.get_version().is_http2()
    }

    /// Checks whether the HTTP version is HTTP/3.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/3.
    #[inline(always)]
    pub fn is_http3_version(&self) -> bool {
        self.get_version().is_http3()
    }

    /// Checks if the HTTP version is HTTP/1.1 or higher.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP/1.1 or higher.
    #[inline(always)]
    pub fn is_http1_1_or_higher_version(&self) -> bool {
        self.get_version().is_http1_1_or_higher()
    }

    /// Checks whether the version belongs to the HTTP family.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the version is HTTP.
    #[inline(always)]
    pub fn is_http_version(&self) -> bool {
        self.get_version().is_http()
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

    /// Checks whether the WebSocket upgrade is enabled for this request.
    ///
    /// This method determines if the `UPGRADE` header indicates a WebSocket connection.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether WebSocket upgrade is enabled.
    #[inline(always)]
    pub fn is_ws_upgrade_type(&self) -> bool {
        self.get_upgrade_type().is_ws()
    }

    /// Checks if the current upgrade type is HTTP/2 cleartext (h2c).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is h2c.
    #[inline(always)]
    pub fn is_h2c_upgrade_type(&self) -> bool {
        self.get_upgrade_type().is_h2c()
    }

    /// Checks if the current upgrade type is TLS (any version).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is TLS.
    #[inline(always)]
    pub fn is_tls_upgrade_type(&self) -> bool {
        self.get_upgrade_type().is_tls()
    }

    /// Checks whether the upgrade type is unknown.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the upgrade type is unknown.
    #[inline(always)]
    pub fn is_unknown_upgrade_type(&self) -> bool {
        self.get_upgrade_type().is_unknown()
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
                return self.is_ws_upgrade_type();
            }
        }
        self.is_http1_1_or_higher_version() || self.is_ws_upgrade_type()
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
