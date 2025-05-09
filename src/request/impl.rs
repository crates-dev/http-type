use crate::*;

impl Default for Request {
    fn default() -> Self {
        Self {
            method: Methods::default(),
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
    /// Creates a new `Request` object from a TCP stream.
    ///
    /// # Parameters
    /// - `reader`: A mut reference to a `&mut BufReader<&mut TcpStream>`
    /// - `buffer_size`: Request buffer size
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
        if parts.len() < 3 {
            return Err(RequestError::InvalidHttpRequest);
        }
        let method: RequestMethod = parts[0]
            .to_string()
            .parse::<RequestMethod>()
            .unwrap_or_default();
        let full_path: RequestPath = parts[1].to_string();
        let version: RequestVersion = parts[2]
            .to_string()
            .parse::<RequestVersion>()
            .unwrap_or_default();
        let hash_index: OptionUsize = full_path.find(HASH_SYMBOL);
        let query_index: OptionUsize = full_path.find(QUERY_SYMBOL);
        let query_string: String = query_index.map_or(EMPTY_STR.to_owned(), |i| {
            let temp: String = full_path[i + 1..].to_string();
            if hash_index.is_none() || hash_index.unwrap() <= i {
                return temp.into();
            }
            let data: String = temp
                .split(HASH_SYMBOL)
                .next()
                .unwrap_or_default()
                .to_string();
            data.into()
        });
        let querys: RequestQuerys = Self::parse_querys(&query_string);
        let path: RequestPath = if let Some(i) = query_index.or(hash_index) {
            full_path[..i].to_string()
        } else {
            full_path
        };
        let mut headers: RequestHeaders = hash_map_xx_hash3_64();
        let mut host: RequestHost = EMPTY_STR.to_owned();
        let mut content_length: usize = 0;
        loop {
            let mut header_line: String = String::with_capacity(buffer_size);
            let _ = AsyncBufReadExt::read_line(reader, &mut header_line).await;
            let header_line: &str = header_line.trim();
            if header_line.is_empty() {
                break;
            }
            let parts: Vec<&str> = header_line.splitn(2, COLON_SPACE_SYMBOL).collect();
            if parts.len() != 2 {
                continue;
            }
            let key: String = parts[0].trim().to_ascii_lowercase();
            let value: String = parts[1].trim().to_string();
            if key == HOST {
                host = value.clone();
            } else if key == CONTENT_LENGTH {
                content_length = value.parse().unwrap_or(0);
            }
            headers.insert(key, value);
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
    /// - `buffer_size`: Request buffer size
    ///
    /// # Returns
    /// - `Ok`: A `Request` object populated with the HTTP request data.
    /// - `Err`: An `RequestError` if the request is invalid or cannot be read.
    pub async fn http_request_from_stream(
        stream: &ArcRwLockStream,
        buffer_size: usize,
    ) -> RequestReaderHandleResult {
        let mut buf_stream: RwLockWriteGuard<'_, TcpStream> = stream.get_write_lock().await;
        let mut reader: BufReader<&mut TcpStream> = BufReader::new(&mut buf_stream);
        Self::http_from_reader(&mut reader, buffer_size).await
    }

    /// Creates a new `Request` object from a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A reference to a `&ArcRwLockStream` representing the incoming connection.
    /// - `buffer_size`: Request buffer size
    ///
    /// # Returns
    /// - `Ok`: A `Request` object populated with the HTTP request data.
    /// - `Err`: An `RequestError` if the request is invalid or cannot be read.
    pub async fn websocket_request_from_stream(
        stream: &ArcRwLockStream,
        buffer_size: usize,
    ) -> RequestReaderHandleResult {
        let mut buf_stream: RwLockWriteGuard<'_, TcpStream> = stream.get_write_lock().await;
        let mut reader: BufReader<&mut TcpStream> = BufReader::new(&mut buf_stream);
        Self::websocket_from_reader(&mut reader, buffer_size).await
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
    /// - `buffer_size`: - Request buffer size
    ///
    /// # Returns
    /// - `Ok(Request)` - A `Request` object constructed from the received data.
    ///   - If no data is read (`Ok(0)`), an empty `Request` object is returned.
    ///   - If data is successfully read, the request body is set with the received bytes.
    /// - `Err(RequestError::InvalidWebSocketRequest)` - If an error occurs while reading from the stream.
    pub async fn websocket_from_reader(
        reader: &mut BufReader<&mut TcpStream>,
        buffer_size: usize,
    ) -> RequestReaderHandleResult {
        let mut dynamic_buffer: Vec<u8> = Vec::with_capacity(buffer_size);
        let mut temp_buffer: Vec<u8> = vec![0; buffer_size];
        let mut full_frame: Vec<u8> = Vec::with_capacity(buffer_size);
        loop {
            let len: usize = match reader.read(&mut temp_buffer).await {
                Ok(len) => len,
                Err(_) => return Err(RequestError::InvalidWebSocketRequest),
            };
            if len == 0 {
                break;
            }
            dynamic_buffer.extend_from_slice(&temp_buffer[..len]);
            if let Some((frame, consumed)) =
                WebSocketFrame::decode_websocket_frame_with_length(&dynamic_buffer)
            {
                dynamic_buffer.drain(0..consumed);
                full_frame.extend_from_slice(frame.get_payload_data());
                if *frame.get_fin() {
                    let mut request: Request = Request::default();
                    request.set_body(full_frame);
                    return Ok(request);
                }
            }
        }
        Err(RequestError::InvalidWebSocketRequest)
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
            let mut parts: SplitN<'_, &str> = pair.splitn(2, EQUAL);
            let key: String = parts.next().unwrap_or_default().to_string();
            if key.is_empty() {
                continue;
            }
            let value: String = parts.next().unwrap_or_default().to_string();
            query_map.insert(key, value);
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
        self.querys
            .get(&key.into())
            .and_then(|data| Some(data.clone()))
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
        self.headers
            .get(&key.into())
            .and_then(|data| Some(data.clone()))
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
    /// It then attempts to deserialize the string into the specified type `T` using `serde_json::from_str`.
    ///
    /// # Type Parameters
    /// - `T`: The target type to deserialize into. It must implement the `DeserializeOwned` trait.
    ///
    /// # Returns
    /// - `Ok(T)`: The deserialized object of type `T` if the deserialization is successful.
    /// - `Err(serde_json::Error)`: An error if the deserialization fails (e.g., invalid JSON format or type mismatch).
    pub fn get_body_json<T>(&self) -> ResultSerdeJsonError<T>
    where
        T: DeserializeOwned,
    {
        json_parse_from_slice(self.get_body())
    }

    /// Adds a header to the request.
    ///
    /// This function inserts a key-value pair into the request headers.
    /// The key and value are converted into `String`, allowing for efficient handling of both owned and borrowed string data.
    ///
    /// # Parameters
    /// - `key`: The header key, which will be converted into a `String`.
    /// - `value`: The value of the header, which will be converted into a `String`.
    ///
    /// # Returns
    /// - Returns a mutable reference to the current instance (`&mut Self`), allowing for method chaining.
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set the body of the response.
    ///
    /// This method allows you to set the body of the response by converting the provided
    /// value into a `RequestBody` type. The `body` is updated with the converted value,
    /// and the method returns a mutable reference to the current instance for method chaining.
    ///
    /// # Parameters
    /// - `body`: The body of the response to be set. It can be any type that can be converted
    ///   into a `RequestBody` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    /// Set the body of the response.
    ///
    /// This method allows you to set the body of the response by converting the provided
    /// value into a `RequestBody` type. The `body` is updated with the converted value,
    /// and the method returns a mutable reference to the current instance for method chaining.
    ///
    /// # Parameters
    /// - `body`: The body of the response to be set. It can be any type that can be converted
    ///   into a `RequestBody` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    pub fn set_body<T: Into<RequestBody>>(&mut self, body: T) -> &mut Self {
        self.body = body.into();
        self
    }

    /// Set the HTTP method of the request.
    ///
    /// This method allows you to set the HTTP method (e.g., GET, POST) of the request
    /// by converting the provided value into a `RequestMethod` type. The `method` is updated
    /// with the converted value, and the method returns a mutable reference to the current
    /// instance for method chaining.
    ///
    /// # Parameters
    /// - `method`: The HTTP method to be set for the request. It can be any type that can
    ///   be converted into a `RequestMethod` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    pub fn set_method<T: Into<RequestMethod>>(&mut self, method: T) -> &mut Self {
        self.method = method.into();
        self
    }

    /// Set the host of the request.
    ///
    /// This method allows you to set the host (e.g., www.example.com) for the request
    /// by converting the provided value into a `RequestHost` type. The `host` is updated
    /// with the converted value, and the method returns a mutable reference to the current
    /// instance for method chaining.
    ///
    /// # Parameters
    /// - `host`: The host to be set for the request. It can be any type that can be converted
    ///   into a `RequestHost` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    pub fn set_host<T: Into<RequestHost>>(&mut self, host: T) -> &mut Self {
        self.host = host.into();
        self
    }

    /// Set the path of the request.
    ///
    /// This method allows you to set the path (e.g., /api/v1/resource) for the request
    /// by converting the provided value into a `RequestPath` type. The `path` is updated
    /// with the converted value, and the method returns a mutable reference to the current
    /// instance for method chaining.
    ///
    /// # Parameters
    /// - `path`: The path to be set for the request. It can be any type that can be converted
    ///   into a `RequestPath` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    pub fn set_path<T: Into<RequestPath>>(&mut self, path: T) -> &mut Self {
        self.path = path.into();
        self
    }

    /// Sets a query parameter for the request.
    ///
    /// # Parameters
    /// - `key`: The query parameter's key, which can be of any type that implements `Into<RequestQuerysKey>`.
    /// - `value`: The query parameter's value, which can be of any type that implements `Into<RequestQuerysValue>`.
    ///
    /// # Returns
    /// - Returns a mutable reference to the current instance (`Self`), allowing for method chaining.
    pub fn set_query<K: Into<RequestQuerysKey>, V: Into<RequestQuerysValue>>(
        &mut self,
        key: K,
        value: V,
    ) -> &mut Self {
        self.querys.insert(key.into(), value.into());
        self
    }

    /// Converts the request to a formatted string representation.
    ///
    /// - Returns: A `String` containing formatted request details.
    pub fn get_string(&self) -> String {
        let body: &Vec<u8> = self.get_body();
        format!(
            "[Request] => [Method]: {}; [Host]: {}; [Version]: {}; [Path]: {}; [Querys]: {:?}; [Headers]: {:?}; [Body]: {};",
            self.get_method(),
            self.get_host(),
            self.get_version(),
            self.get_path(),
            self.get_querys(),
            self.get_headers(),
            body_to_string(body),
        )
    }

    /// Retrieves the upgrade type from the request headers.
    ///
    /// - Returns: The `UpgradeType` extracted from the `UPGRADE` header.
    ///            If the header is missing or invalid, returns the default `UpgradeType`.
    pub fn get_upgrade_type(&self) -> UpgradeType {
        let upgrade_type: UpgradeType = self
            .get_header(UPGRADE)
            .and_then(|data| data.parse::<UpgradeType>().ok())
            .unwrap_or_default();
        upgrade_type
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
        if let Some(connection_value) = self.get_header(CONNECTION) {
            let connection_value_lowercase: String = connection_value.to_ascii_lowercase();
            if connection_value_lowercase == CONNECTION_KEEP_ALIVE {
                return true;
            } else if connection_value_lowercase == CONNECTION_CLOSE {
                return false;
            }
        }
        self.get_version().is_http1_1_or_higher()
    }

    /// Determines if keep-alive connection should be disabled for this request.
    ///
    /// This is the inverse of `is_enable_keep_alive()`. It returns true when
    /// keep-alive should be disabled, and false when it should be enabled.
    ///
    /// # Returns
    /// - `bool`: true if keep-alive should be disabled, false otherwise
    pub fn is_disable_keep_alive(&self) -> bool {
        !self.is_enable_keep_alive()
    }
}
