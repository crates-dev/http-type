use crate::*;

impl Default for Response {
    fn default() -> Self {
        Self {
            version: HttpVersion::default(),
            status_code: ResponseStatusCode::default(),
            reason_phrase: ResponseReasonPhrase::default(),
            headers: hash_map_xx_hash3_64(),
            body: Vec::new(),
        }
    }
}

impl Response {
    /// Creates a new instance of `Response`.
    ///
    /// # Returns
    ///
    /// An initialized `Response` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieves the value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `key`: The header's key. It can be any type that can be converted into `ResponseHeadersKey`.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValue`: Returns `Some(value)` if the key exists in the response headers,
    ///   or `None` if the key does not exist.
    pub fn get_header<K>(&self, key: K) -> OptionResponseHeadersValue
    where
        K: Into<ResponseHeadersKey>,
    {
        self.headers
            .get(&key.into())
            .and_then(|data| Some(data.clone()))
    }

    /// Retrieves the first value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `key`: The header's key. It can be any type that can be converted into `ResponseHeadersKey`.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValueItem`: Returns `Some(value)` if the key exists and has at least one value,
    ///   or `None` if the key does not exist or has no values.
    pub fn get_header_front<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: Into<ResponseHeadersKey>,
    {
        self.headers
            .get(&key.into())
            .and_then(|values| values.front().cloned())
    }

    /// Retrieves the last value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `key`: The header's key. It can be any type that can be converted into `ResponseHeadersKey`.
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValueItem`: Returns `Some(value)` if the key exists and has at least one value,
    ///   or `None` if the key does not exist or has no values.
    pub fn get_header_back<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: Into<ResponseHeadersKey>,
    {
        self.headers
            .get(&key.into())
            .and_then(|values| values.back().cloned())
    }

    /// Checks if a header exists in the response.
    ///
    /// # Arguments
    ///
    /// - `key`: The header key to check. It will be converted into a `ResponseHeadersKey`.
    ///
    /// # Returns
    ///
    /// - `true`: If the header exists.
    /// - `false`: If the header does not exist.
    pub fn has_header<K>(&self, key: K) -> bool
    where
        K: Into<ResponseHeadersKey>,
    {
        let key: ResponseHeadersKey = key.into().to_lowercase();
        self.headers.contains_key(&key)
    }

    /// Checks if a header contains a specific value.
    ///
    /// # Arguments
    ///
    /// - `key`: The header key to check. It will be converted into a `ResponseHeadersKey`.
    /// - `value`: The value to search for within the header's values.
    ///
    /// # Returns
    ///
    /// - `true`: If the header exists and contains the specified value.
    /// - `false`: If the header does not exist or does not contain the value.
    pub fn has_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: Into<ResponseHeadersKey>,
        V: Into<ResponseHeadersValueItem>,
    {
        let key: ResponseHeadersKey = key.into();
        let value: ResponseHeadersValueItem = value.into();
        if let Some(values) = self.headers.get(&key) {
            values.contains(&value)
        } else {
            false
        }
    }

    /// Gets the number of headers in the response.
    ///
    /// # Returns
    ///
    /// - The number of unique header keys in the response.
    pub fn get_headers_length(&self) -> usize {
        self.headers.len()
    }

    /// Gets the number of values for a specific header key.
    ///
    /// # Arguments
    ///
    /// - `key`: The header key to count values for. It will be converted into a `ResponseHeadersKey`.
    ///
    /// # Returns
    ///
    /// - The number of values for the specified header key. Returns 0 if the header does not exist.
    pub fn get_header_length<K>(&self, key: K) -> usize
    where
        K: Into<ResponseHeadersKey>,
    {
        let key: ResponseHeadersKey = key.into().to_lowercase();
        self.headers.get(&key).map_or(0, |values| values.len())
    }

    /// Gets the total number of header values in the response.
    ///
    /// This counts all values across all headers, so a header with multiple values
    /// will contribute more than one to the total count.
    ///
    /// # Returns
    ///
    /// - The total number of header values in the response.
    pub fn get_headers_values_length(&self) -> usize {
        self.headers.values().map(|values| values.len()).sum()
    }

    /// Retrieves the body content of the response as a UTF-8 encoded string.
    ///
    /// This method uses `String::from_utf8_lossy` to convert the byte slice returned by `self.get_body()` into a string.
    /// If the byte slice contains invalid UTF-8 sequences, they will be replaced with the Unicode replacement character ().
    ///
    /// # Returns
    ///
    /// A `String` containing the body content.
    pub fn get_body_string(&self) -> String {
        String::from_utf8_lossy(self.get_body()).into_owned()
    }

    /// Deserializes the body content of the response into a specified type `T`.
    ///
    /// This method first retrieves the body content as a byte slice using `self.get_body()`.
    /// It then attempts to deserialize the byte slice into the specified type `T` using `json_from_slice`.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The target type to deserialize into. It must implement the `DeserializeOwned` trait.
    ///
    /// # Returns
    ///
    /// - `Ok(T)`: The deserialized object of type `T` if the deserialization is successful.
    /// - `Err(ResultJsonError)`: An error if the deserialization fails.
    pub fn get_body_json<T>(&self) -> ResultJsonError<T>
    where
        T: DeserializeOwned,
    {
        json_from_slice(self.get_body())
    }

    /// Adds a header to the response.
    ///
    /// This function appends a value to the response headers.
    /// If the header already exists, the new value will be added to the existing values.
    ///
    /// # Arguments
    ///
    /// - `key`: The header key, which will be converted into a `ResponseHeadersKey`.
    /// - `value`: The value of the header, which will be converted into a String.
    ///
    /// # Returns
    ///
    /// - `&mut Self`: A mutable reference to the current instance for method chaining.
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<ResponseHeadersKey>,
        V: Into<String>,
    {
        let key: ResponseHeadersKey = key.into().to_lowercase();
        if key.trim().is_empty() || key == CONTENT_LENGTH {
            return self;
        }
        let value: String = value.into();
        self.headers
            .entry(key)
            .or_insert_with(VecDeque::new)
            .push_back(value);
        self
    }

    /// Replaces all values for a header in the response.
    ///
    /// This function replaces all existing values for a header with a single new value.
    ///
    /// # Arguments
    ///
    /// - `key`: The header key, which will be converted into a `ResponseHeadersKey`.
    /// - `value`: The value of the header, which will be converted into a String.
    ///
    /// # Returns
    ///
    /// - `&mut Self`: A mutable reference to the current instance for method chaining.
    pub fn replace_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<ResponseHeadersKey>,
        V: Into<String>,
    {
        let key: ResponseHeadersKey = key.into().to_lowercase();
        if key.trim().is_empty() {
            return self;
        }
        let value: String = value.into();
        let mut deque: VecDeque<String> = VecDeque::new();
        deque.push_back(value);
        self.headers.insert(key, deque);
        self
    }

    /// Removes a header from the response.
    ///
    /// This function removes all values for the specified header key.
    ///
    /// # Arguments
    ///
    /// - `key`: The header key to remove, which will be converted into a `ResponseHeadersKey`.
    ///
    /// # Returns
    ///
    /// - `&mut Self`: A mutable reference to the current instance for method chaining.
    pub fn remove_header<K>(&mut self, key: K) -> &mut Self
    where
        K: Into<ResponseHeadersKey>,
    {
        let key: ResponseHeadersKey = key.into().to_lowercase();
        let _ = self.headers.remove(&key).is_some();
        self
    }

    /// Removes a specific value from a header in the response.
    ///
    /// This function removes only the specified value from the header.
    /// If the header has multiple values, only the matching value is removed.
    /// If this was the last value for the header, the entire header is removed.
    ///
    /// # Arguments
    ///
    /// - `key`: The header key, which will be converted into a `ResponseHeadersKey`.
    /// - `value`: The specific value to remove from the header.
    ///
    /// # Returns
    ///
    /// - `&mut Self`: A mutable reference to the current instance for method chaining.
    pub fn remove_header_value<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<ResponseHeadersKey>,
        V: Into<String>,
    {
        let key: ResponseHeadersKey = key.into().to_lowercase();
        let value: String = value.into();
        if let Some(values) = self.headers.get_mut(&key) {
            values.retain(|v| v != &value);
            if values.is_empty() {
                self.headers.remove(&key);
            }
        }
        self
    }

    /// Clears all headers from the response.
    ///
    /// This function removes all headers, leaving the headers map empty.
    ///
    /// # Returns
    ///
    /// - `&mut Self`: A mutable reference to the current instance for method chaining.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.headers.clear();
        self
    }

    /// Sets the body of the response.
    ///
    /// This method allows you to set the body of the response by converting the provided
    /// value into a `ResponseBody` type. The `body` is updated with the converted value.
    ///
    /// # Arguments
    ///
    /// - `body`: The body of the response to be set. It can be any type that can be converted
    ///   into a `ResponseBody` using the `Into` trait.
    ///
    /// # Returns
    ///
    /// - `&mut Self`: A mutable reference to the current instance for method chaining.
    pub fn set_body<T>(&mut self, body: T) -> &mut Self
    where
        T: Into<ResponseBody>,
    {
        self.body = body.into();
        self
    }

    /// Sets the reason phrase of the response.
    ///
    /// This method allows you to set the reason phrase of the response by converting the
    /// provided value into a `ResponseReasonPhrase` type. The `reason_phrase` is updated
    /// with the converted value.
    ///
    /// # Arguments
    ///
    /// - `reason_phrase`: The reason phrase to be set for the response. It can be any type
    ///   that can be converted into a `ResponseReasonPhrase` using the `Into` trait.
    ///
    /// # Returns
    ///
    /// - `&mut Self`: A mutable reference to the current instance for method chaining.
    pub fn set_reason_phrase<T>(&mut self, reason_phrase: T) -> &mut Self
    where
        T: Into<ResponseReasonPhrase>,
    {
        self.reason_phrase = reason_phrase.into();
        self
    }

    /// Pushes a header with a key and value into the response string.
    ///
    /// # Arguments
    ///
    /// - `response_string`: A mutable reference to the string where the header will be added.
    /// - `key`: The header key as a string slice (`&str`).
    /// - `value`: The header value as a string slice (`&str`).
    fn push_header(response_string: &mut String, key: &str, value: &str) {
        response_string.push_str(key);
        response_string.push_str(COLON_SPACE);
        response_string.push_str(value);
        response_string.push_str(HTTP_BR);
    }

    /// Pushes the first line of an HTTP response (version, status code, and reason phrase) into the response string.
    /// This corresponds to the status line of the HTTP response.
    ///
    /// # Arguments
    ///
    /// - `response_string`: A mutable reference to the string where the first line will be added.
    fn push_http_first_line(&self, response_string: &mut String) {
        response_string.push_str(&self.get_version().to_string());
        response_string.push_str(SPACE);
        response_string.push_str(&self.get_status_code().to_string());
        response_string.push_str(SPACE);
        response_string.push_str(self.get_reason_phrase());
        response_string.push_str(HTTP_BR);
    }

    /// Builds the full HTTP response as a byte vector.
    ///
    /// This method constructs the complete HTTP response, including the status line,
    /// headers, and body. It handles content encoding, content type, connection
    /// management, and content length.
    ///
    /// # Returns
    ///
    /// - `ResponseData`: The complete HTTP response as a byte vector.
    pub fn build(&mut self) -> ResponseData {
        if self.reason_phrase.is_empty() {
            self.set_reason_phrase(HttpStatus::phrase(*self.get_status_code()));
        }
        let mut response_string: String = String::new();
        self.push_http_first_line(&mut response_string);
        let mut compress_type_opt: OptionCompress = None;
        let mut connection_opt: OptionString = None;
        let mut content_type_opt: OptionString = None;
        let headers: ResponseHeaders = self
            .get_mut_headers()
            .drain()
            .map(|(key, value)| (key.to_lowercase(), value))
            .collect();
        let mut unset_content_length: bool = false;
        for (key, values) in headers.iter() {
            for value in values.iter() {
                if key == CONTENT_ENCODING {
                    compress_type_opt = Some(value.parse::<Compress>().unwrap_or_default());
                } else if key == CONNECTION {
                    connection_opt = Some(value.to_owned());
                } else if key == CONTENT_TYPE {
                    content_type_opt = Some(value.to_owned());
                    if value.eq_ignore_ascii_case(TEXT_EVENT_STREAM) {
                        unset_content_length = true;
                    }
                }
                Self::push_header(&mut response_string, key, value);
            }
        }
        if connection_opt.is_none() {
            Self::push_header(&mut response_string, CONNECTION, KEEP_ALIVE);
        }
        if content_type_opt.is_none() {
            let mut content_type: String = String::with_capacity(
                TEXT_HTML.len() + SEMICOLON_SPACE.len() + CHARSET_UTF_8.len(),
            );
            content_type.push_str(TEXT_HTML);
            content_type.push_str(SEMICOLON_SPACE);
            content_type.push_str(CHARSET_UTF_8);
            Self::push_header(&mut response_string, CONTENT_TYPE, &content_type);
        }
        let mut body: Cow<Vec<u8>> = Cow::Borrowed(self.get_body());
        if !unset_content_length {
            if let Some(compress_type) = compress_type_opt {
                if !compress_type.is_unknown() {
                    let tmp_body: Cow<'_, Vec<u8>> =
                        compress_type.encode(&body, DEFAULT_BUFFER_SIZE);
                    body = Cow::Owned(tmp_body.into_owned());
                }
            }
            let len_string: String = body.len().to_string();
            Self::push_header(&mut response_string, CONTENT_LENGTH, &len_string);
        }
        response_string.push_str(HTTP_BR);
        let mut response_bytes: Vec<u8> = response_string.into_bytes();
        response_bytes.extend_from_slice(&body);
        response_bytes
    }

    /// Converts the response to a formatted string representation.
    ///
    /// This method provides a human-readable summary of the response, including its version,
    /// status code, reason phrase, headers, and body information.
    ///
    /// # Returns
    ///
    /// A `String` containing formatted response details.
    pub fn get_string(&self) -> String {
        let body: &Vec<u8> = self.get_body();
        let body_type: &'static str = if std::str::from_utf8(body).is_ok() {
            PLAIN
        } else {
            BINARY
        };
        format!(
            "[Response] => [version]: {}; [status code]: {}; [reason]: {}; [headers]: {:?}; [body]: {} bytes {};",
            self.get_version(),
            self.get_status_code(),
            self.get_reason_phrase(),
            self.get_headers(),
            body.len(),
            body_type
        )
    }
}
