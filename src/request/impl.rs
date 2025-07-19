use crate::*;

impl Default for Request {
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
    /// - An initialized `Request` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Request` object from a TCP stream.
    ///
    /// # Parameters
    /// - `reader`: A mut reference to a `&mut BufReader<&mut TcpStream>`.
    /// - `buffer_size`: Request buffer size.
    ///
    /// # Returns
    /// - `Ok`: A `Request` object populated with the HTTP request data.
    /// - `Err`: An `RequestError` if the request is invalid or cannot be read.
    pub async fn http_from_reader(
        reader: &mut BufReader<&mut TcpStream>,
        buffer_size: usize,
    ) -> RequestReaderHandleResult {
        let mut request_line: String = String::with_capacity(buffer_size);
        let _ = AsyncBufReadExt::read_line(reader, &mut request_line).await;
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        let parts_len: usize = parts.len();
        if parts_len < 3 {
            return Err(RequestError::InvalidHttpRequestPartsLength(parts_len));
        }
        let method: RequestMethod = parts[0].parse::<RequestMethod>().unwrap_or_default();
        let full_path: RequestPath = parts[1].to_string();
        let version: RequestVersion = parts[2].parse::<RequestVersion>().unwrap_or_default();
        let hash_index: OptionUsize = full_path.find(HASH_SYMBOL);
        let query_index: OptionUsize = full_path.find(QUERY_SYMBOL);
        let query_string: String = query_index.map_or(String::new(), |i| {
            let temp: &str = &full_path[i + 1..];
            if hash_index.is_none() || hash_index.unwrap() <= i {
                return temp.to_string();
            }
            temp.split(HASH_SYMBOL)
                .next()
                .unwrap_or_default()
                .to_string()
        });
        let querys: RequestQuerys = Self::parse_querys(&query_string);
        let path: RequestPath = if let Some(i) = query_index.or(hash_index) {
            full_path[..i].to_string()
        } else {
            full_path
        };
        let mut headers: RequestHeaders = hash_map_xx_hash3_64();
        let mut host: RequestHost = String::new();
        let mut content_length: usize = 0;
        loop {
            let mut header_line: String = String::with_capacity(buffer_size);
            let _ = AsyncBufReadExt::read_line(reader, &mut header_line).await;
            let header_line: &str = header_line.trim();
            if header_line.is_empty() {
                break;
            }
            if let Some((key_part, value_part)) = header_line.split_once(COLON_SPACE_SYMBOL) {
                let key: String = key_part.trim().to_ascii_lowercase();
                if key.is_empty() {
                    continue;
                }
                let value: String = value_part.trim().to_string();
                if key == HOST {
                    host = value.clone();
                } else if key == CONTENT_LENGTH {
                    content_length = value.parse().unwrap_or(0);
                }
                headers
                    .entry(key)
                    .or_insert_with(VecDeque::new)
                    .push_back(value);
            }
        }
        let mut body: RequestBody = vec![0; content_length];
        if content_length > 0 {
            let _ = AsyncReadExt::read_exact(reader, &mut body).await;
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

    /// Creates a new `Request` object from a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A reference to a `&ArcRwLockStream` representing the incoming connection.
    /// - `buffer_size`: Request buffer size.
    ///
    /// # Returns
    /// - `Ok`: A `Request` object populated with the HTTP request data.
    /// - `Err`: An `RequestError` if the request is invalid or cannot be read.
    pub async fn http_from_stream(
        stream: &ArcRwLockStream,
        buffer_size: usize,
    ) -> RequestReaderHandleResult {
        let mut buf_stream: RwLockWriteGuard<'_, TcpStream> = stream.write().await;
        let mut reader: BufReader<&mut TcpStream> = BufReader::new(&mut buf_stream);
        Self::http_from_reader(&mut reader, buffer_size).await
    }

    /// Creates a new `Request` object from a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A reference to a `&ArcRwLockStream` representing the incoming connection.
    /// - `buffer_size`: Request buffer size.
    /// - `request`: A reference to a `Request` object. This object is used as a template.
    ///
    /// # Returns
    /// - `Ok`: A `Request` object populated with the HTTP request data.
    /// - `Err`: An `RequestError` if the request is invalid or cannot be read.
    pub async fn ws_from_stream(
        stream: &ArcRwLockStream,
        buffer_size: usize,
        request: &mut Self,
    ) -> RequestReaderHandleResult {
        let mut buf_stream: RwLockWriteGuard<'_, TcpStream> = stream.write().await;
        let mut reader: BufReader<&mut TcpStream> = BufReader::new(&mut buf_stream);
        Self::ws_from_reader(&mut reader, buffer_size, request).await
    }

    /// Reads a WebSocket request from a TCP stream and constructs a `Request` object.
    ///
    /// This function reads data from the provided `BufReader` wrapped around a `TcpStream`.
    /// It attempts to read up to 1024 bytes into a buffer and constructs a `Request` object
    /// based on the received data. The request body is set using the received bytes.
    ///
    /// # Arguments
    /// - `reader` - A mutable reference to a `BufReader` wrapping a `TcpStream`.
    ///   This reader is used to read the incoming WebSocket request data.
    /// - `buffer_size`: - Request buffer size.
    /// - `request` - A reference to a `Request` object. This object is used as a template.
    ///
    /// # Returns
    /// - `Ok(Request)` - A `Request` object constructed from the received data.
    ///   - If no data is read (`Ok(0)`), an empty `Request` object is returned.
    ///   - If data is successfully read, the request body is set with the received bytes.
    /// - `Err(RequestError::InvalidWebSocketRequest)` - If an error occurs while reading from the stream.
    pub async fn ws_from_reader(
        reader: &mut BufReader<&mut TcpStream>,
        buffer_size: usize,
        request: &mut Self,
    ) -> RequestReaderHandleResult {
        let mut dynamic_buffer: Vec<u8> = Vec::with_capacity(buffer_size);
        let mut temp_buffer: Vec<u8> = vec![0; buffer_size];
        let mut full_frame: Vec<u8> = Vec::new();
        let mut error_handle = || {
            request.body.clear();
        };
        loop {
            let len: usize = match reader.read(&mut temp_buffer).await {
                Ok(len) => len,
                Err(err) => {
                    error_handle();
                    if err.kind() == ErrorKind::ConnectionReset
                        || err.kind() == ErrorKind::ConnectionAborted
                    {
                        return Err(RequestError::ClientDisconnected);
                    }
                    return Err(RequestError::InvalidWebSocketRequest(err.to_string()));
                }
            };
            if len == 0 {
                error_handle();
                return Err(RequestError::IncompleteWebSocketFrame);
            }
            dynamic_buffer.extend_from_slice(&temp_buffer[..len]);
            while let Some((frame, consumed)) = WebSocketFrame::decode_ws_frame(&dynamic_buffer) {
                dynamic_buffer.drain(0..consumed);
                match frame.get_opcode() {
                    WebSocketOpcode::Close => {
                        error_handle();
                        return Err(RequestError::ClientClosedConnection);
                    }
                    WebSocketOpcode::Ping | WebSocketOpcode::Pong => {
                        continue;
                    }
                    WebSocketOpcode::Text | WebSocketOpcode::Binary => {
                        full_frame.extend_from_slice(frame.get_payload_data());
                        if *frame.get_fin() {
                            let mut request: Request = request.clone();
                            request.body = full_frame;
                            return Ok(request);
                        }
                    }
                    _ => {
                        error_handle();
                        return Err(RequestError::InvalidWebSocketFrame(
                            "Unsupported opcode".into(),
                        ));
                    }
                }
            }
        }
    }

    /// Parse querys
    ///
    /// # Parameters
    /// - `query`: &str
    ///
    /// # Returns
    /// - RequestQuerys
    fn parse_querys(query: &str) -> RequestQuerys {
        let mut query_map: RequestQuerys = hash_map_xx_hash3_64();
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

    /// Retrieves the value of a query parameter by its key.
    ///
    /// # Parameters
    /// - `key`: The query parameter's key, which can be of any type that implements `Into<RequestQuerysKey>`.
    ///
    /// # Returns
    /// - `OptionRequestQuerysValue`: Returns `Some(value)` if the key exists in the query parameters,
    ///   or `None` if the key does not exist.
    pub fn get_query<K>(&self, key: K) -> OptionRequestQuerysValue
    where
        K: Into<RequestQuerysKey>,
    {
        self.querys.get(&key.into()).cloned()
    }

    /// Retrieves the value of a request header by its key.
    ///
    /// # Parameters
    /// - `key`: The header's key, which can be of any type that implements `Into<RequestHeadersKey>`.
    ///
    /// # Returns
    /// - `OptionRequestHeadersValue`: Returns `Some(value)` if the key exists in the request headers,
    ///   or `None` if the key does not exist.
    pub fn get_header<K>(&self, key: K) -> OptionRequestHeadersValue
    where
        K: Into<RequestHeadersKey>,
    {
        self.headers.get(&key.into()).cloned()
    }

    /// Retrieves the first value of a request header by its key.
    ///
    /// # Parameters
    /// - `key`: The header's key, which can be of any type that implements `Into<RequestHeadersKey>`.
    ///
    /// # Returns
    /// - `OptionRequestHeadersValueItem`: Returns `Some(value)` if the key exists and has at least one value,
    ///   or `None` if the key does not exist or has no values.
    pub fn get_header_front<K>(&self, key: K) -> OptionRequestHeadersValueItem
    where
        K: Into<RequestHeadersKey>,
    {
        self.headers
            .get(&key.into())
            .and_then(|values| values.front().cloned())
    }

    /// Retrieves the last value of a request header by its key.
    ///
    /// # Parameters
    /// - `key`: The header's key, which can be of any type that implements `Into<RequestHeadersKey>`.
    ///
    /// # Returns
    /// - `OptionRequestHeadersValueItem`: Returns `Some(value)` if the key exists and has at least one value,
    ///   or `None` if the key does not exist or has no values.
    pub fn get_header_back<K>(&self, key: K) -> OptionRequestHeadersValueItem
    where
        K: Into<RequestHeadersKey>,
    {
        self.headers
            .get(&key.into())
            .and_then(|values| values.back().cloned())
    }

    /// Retrieves the number of values for a specific header.
    ///
    /// # Parameters
    /// - `key`: The header's key, which can be of any type that implements `Into<RequestHeadersKey>`.
    ///
    /// # Returns
    /// - `usize`: The number of values for the specified header. Returns 0 if the header does not exist.
    pub fn get_header_len<K>(&self, key: K) -> usize
    where
        K: Into<RequestHeadersKey>,
    {
        self.headers
            .get(&key.into())
            .map(|values| values.len())
            .unwrap_or(0)
    }

    /// Retrieves the total number of header values across all headers.
    ///
    /// # Returns
    /// - `usize`: The total count of all header values.
    pub fn get_headers_values_len(&self) -> usize {
        self.headers.values().map(|values| values.len()).sum()
    }

    /// Retrieves the number of unique headers.
    ///
    /// # Returns
    /// - `usize`: The number of unique header keys.
    pub fn get_headers_len(&self) -> usize {
        self.headers.len()
    }

    /// Checks if a specific header exists.
    ///
    /// # Parameters
    /// - `key`: The header key to check, which will be converted into a `RequestHeadersKey`.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the header exists, `false` otherwise.
    pub fn has_header<K>(&self, key: K) -> bool
    where
        K: Into<RequestHeadersKey>,
    {
        self.headers.contains_key(&key.into())
    }

    /// Checks if a header contains a specific value.
    ///
    /// # Parameters
    /// - `key`: The header key to check, which will be converted into a `RequestHeadersKey`.
    /// - `value`: The value to search for in the header.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the header exists and contains the specified value, `false` otherwise.
    pub fn has_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: Into<RequestHeadersKey>,
        V: Into<RequestHeadersValueItem>,
    {
        let key: RequestHeadersKey = key.into();
        let value: RequestHeadersValueItem = value.into();
        if let Some(values) = self.headers.get(&key) {
            values.contains(&value)
        } else {
            false
        }
    }

    /// Retrieves the body content of the object as a UTF-8 encoded string.
    ///
    /// This method uses `String::from_utf8_lossy` to convert the byte slice returned by `self.get_body()` into a string.
    /// If the byte slice contains invalid UTF-8 sequences, they will be replaced with the Unicode replacement character (�).
    ///
    /// # Returns
    /// A `String` containing the body content.
    pub fn get_body_string(&self) -> String {
        String::from_utf8_lossy(self.get_body()).into_owned()
    }

    /// Deserializes the body content of the object into a specified type `T`.
    ///
    /// This method first retrieves the body content as a UTF-8 encoded string using `self.get_body()`.
    /// It then attempts to deserialize the string into the specified type `T` using `json_from_slice`.
    ///
    /// # Type Parameters
    /// - `T`: The target type to deserialize into. It must implement the `DeserializeOwned` trait.
    ///
    /// # Returns
    /// - `Ok(T)`: The deserialized object of type `T` if the deserialization is successful.
    /// - `Err(ResultJsonError)`: An error if the deserialization fails.
    pub fn get_body_json<T>(&self) -> ResultJsonError<T>
    where
        T: DeserializeOwned,
    {
        json_from_slice(self.get_body())
    }

    /// Converts the request to a formatted string representation.
    ///
    /// - Returns: A `String` containing formatted request details.
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
    /// - Returns: The `UpgradeType` extracted from the `UPGRADE` header.
    ///            If the header is missing or invalid, returns the default `UpgradeType`.
    pub fn get_upgrade_type(&self) -> UpgradeType {
        let upgrade_type: UpgradeType = self
            .get_header_back(UPGRADE)
            .and_then(|data| data.parse::<UpgradeType>().ok())
            .unwrap_or_default();
        upgrade_type
    }

    /// Checks whether the WebSocket upgrade is enabled.
    ///
    /// - Returns: `true` if the upgrade type is WebSocket; otherwise, `false`.
    pub fn is_ws(&self) -> bool {
        self.get_upgrade_type().is_ws()
    }

    /// Checks if the current upgrade type is HTTP/2 cleartext (h2c).
    ///
    /// - `&self` - The current instance (usually a request or context struct).
    ///
    /// - Returns `true` if the upgrade type is `h2c`, otherwise `false`.
    pub fn is_h2c(&self) -> bool {
        self.get_upgrade_type().is_h2c()
    }

    /// Checks if the current upgrade type is TLS (any version).
    ///
    /// - `&self` - The current instance (usually a request or context struct).
    ///
    /// - Returns `true` if the upgrade type is any `Tls` variant, otherwise `false`.
    pub fn is_tls(&self) -> bool {
        self.get_upgrade_type().is_tls()
    }

    /// Checks whether the upgrade type is unknown.
    ///
    /// - Returns: `true` if the upgrade type is unknown; otherwise, `false`.
    pub fn is_unknown_upgrade(&self) -> bool {
        self.get_upgrade_type().is_unknown()
    }

    /// Checks if the HTTP version is HTTP/1.1 or higher.
    ///
    /// - Returns: `true` if the HTTP version is 1.1 or higher; otherwise, `false`.
    pub fn is_http1_1_or_higher(&self) -> bool {
        self.get_version().is_http1_1_or_higher()
    }

    /// Checks whether the HTTP version is HTTP/0.9.
    ///
    /// - Returns: `true` if the version is HTTP/0.9; otherwise, `false`.
    pub fn is_http0_9(&self) -> bool {
        self.get_version().is_http0_9()
    }

    /// Checks whether the HTTP version is HTTP/1.0.
    ///
    /// - Returns: `true` if the version is HTTP/1.0; otherwise, `false`.
    pub fn is_http1_0(&self) -> bool {
        self.get_version().is_http1_0()
    }

    /// Checks whether the HTTP version is HTTP/1.1.
    ///
    /// - Returns: `true` if the version is HTTP/1.1; otherwise, `false`.
    pub fn is_http1_1(&self) -> bool {
        self.get_version().is_http1_1()
    }

    /// Checks whether the HTTP version is HTTP/2.
    ///
    /// - Returns: `true` if the version is HTTP/2; otherwise, `false`.
    pub fn is_http2(&self) -> bool {
        self.get_version().is_http2()
    }

    /// Checks whether the HTTP version is HTTP/3.
    ///
    /// - Returns: `true` if the version is HTTP/3; otherwise, `false`.
    pub fn is_http3(&self) -> bool {
        self.get_version().is_http3()
    }

    /// Checks whether the HTTP version is unknown.
    ///
    /// - Returns: `true` if the version is unknown; otherwise, `false`.
    pub fn is_unknown_version(&self) -> bool {
        self.get_version().is_unknown()
    }

    /// Checks whether the version belongs to the HTTP family.
    ///
    /// - Returns: `true` if the version is a valid HTTP version; otherwise, `false`.
    pub fn is_http(&self) -> bool {
        self.get_version().is_http()
    }

    /// Checks whether the request method is `GET`.
    ///
    /// - Returns: `true` if the method is `GET`; otherwise, `false`.
    pub fn is_get(&self) -> bool {
        self.get_method().is_get()
    }

    /// Checks whether the request method is `POST`.
    ///
    /// - Returns: `true` if the method is `POST`; otherwise, `false`.
    pub fn is_post(&self) -> bool {
        self.get_method().is_post()
    }

    /// Checks whether the request method is `PUT`.
    ///
    /// - Returns: `true` if the method is `PUT`; otherwise, `false`.
    pub fn is_put(&self) -> bool {
        self.get_method().is_put()
    }

    /// Checks whether the request method is `DELETE`.
    ///
    /// - Returns: `true` if the method is `DELETE`; otherwise, `false`.
    pub fn is_delete(&self) -> bool {
        self.get_method().is_delete()
    }

    /// Checks whether the request method is `PATCH`.
    ///
    /// - Returns: `true` if the method is `PATCH`; otherwise, `false`.
    pub fn is_patch(&self) -> bool {
        self.get_method().is_patch()
    }

    /// Checks whether the request method is `HEAD`.
    ///
    /// - Returns: `true` if the method is `HEAD`; otherwise, `false`.
    pub fn is_head(&self) -> bool {
        self.get_method().is_head()
    }

    /// Checks whether the request method is `OPTIONS`.
    ///
    /// - Returns: `true` if the method is `OPTIONS`; otherwise, `false`.
    pub fn is_options(&self) -> bool {
        self.get_method().is_options()
    }

    /// Checks whether the request method is `CONNECT`.
    ///
    /// - Returns: `true` if the method is `CONNECT`; otherwise, `false`.
    pub fn is_connect(&self) -> bool {
        self.get_method().is_connect()
    }

    /// Checks whether the request method is `TRACE`.
    ///
    /// - Returns: `true` if the method is `TRACE`; otherwise, `false`.
    pub fn is_trace(&self) -> bool {
        self.get_method().is_trace()
    }

    /// Checks whether the request method is `UNKNOWN`.
    ///
    /// - Returns: `true` if the method is `UNKNOWN`; otherwise, `false`.
    pub fn is_unknown_method(&self) -> bool {
        self.get_method().is_unknown()
    }

    /// Determines if keep-alive connection should be enabled for this request.
    ///
    /// This function checks the Connection header and HTTP version to determine if
    /// keep-alive should be enabled. The logic is as follows:
    ///
    /// 1. If Connection header exists:
    ///    - Returns true if header value is "keep-alive"
    ///    - Returns false if header value is "close"
    /// 2. If no Connection header:
    ///    - Returns true if HTTP version is 1.1 or higher
    ///    - Returns false otherwise
    ///
    /// # Returns
    /// - `bool`: true if keep-alive should be enabled, false otherwise
    pub fn is_enable_keep_alive(&self) -> bool {
        if let Some(connection_value) = self.get_header_back(CONNECTION) {
            if connection_value.eq_ignore_ascii_case(KEEP_ALIVE) {
                return true;
            } else if connection_value.eq_ignore_ascii_case(CLOSE) {
                return self.is_ws();
            }
        }
        self.is_http1_1_or_higher() || self.is_ws()
    }
}
