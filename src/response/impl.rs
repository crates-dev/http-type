use crate::*;

/// Provides a default value for `Response`.
///
/// Returns a new `Response` instance with all fields initialized to their default values.
impl Default for Response {
    #[inline(always)]
    fn default() -> Self {
        let http_status: HttpStatus = HttpStatus::default();
        Self {
            version: HttpVersion::HTTP1_1,
            status_code: http_status.code(),
            reason_phrase: http_status.to_string(),
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
    /// - `Response` - A new response instance with default values.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Tries to retrieve the value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValue` - The optional header values.
    #[inline(always)]
    pub fn try_get_header<K>(&self, key: K) -> OptionResponseHeadersValue
    where
        K: AsRef<str>,
    {
        self.headers.get(key.as_ref()).cloned()
    }

    /// Retrieves the value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValue` - The optional header values.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header<K>(&self, key: K) -> ResponseHeadersValue
    where
        K: AsRef<str>,
    {
        self.try_get_header(key).unwrap()
    }

    /// Tries to retrieve the first value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValueItem` - The first header value if exists.
    #[inline(always)]
    pub fn try_get_header_front<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.headers
            .get(key.as_ref())
            .and_then(|values| values.front().cloned())
    }

    /// Retrieves the first value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValueItem` - The first header value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_front<K>(&self, key: K) -> ResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_header_front(key).unwrap()
    }

    /// Tries to retrieve the last value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `OptionResponseHeadersValueItem` - The last header value if exists.
    #[inline(always)]
    pub fn try_get_header_back<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.headers
            .get(key.as_ref())
            .and_then(|values| values.back().cloned())
    }

    /// Retrieves the last value of a response header by its key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header's key (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `ResponseHeadersValueItem` - The last header value if exists.
    ///
    /// # Panics
    ///
    /// This function will panic if the header key is not found.
    #[inline(always)]
    pub fn get_header_back<K>(&self, key: K) -> ResponseHeadersValueItem
    where
        K: AsRef<str>,
    {
        self.try_get_header_back(key).unwrap()
    }

    /// Checks if a header exists in the response.
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

    /// Gets the number of headers in the response.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of unique header keys.
    #[inline(always)]
    pub fn get_headers_length(&self) -> usize {
        self.headers.len()
    }

    /// Tries to get the number of values for a specific header key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key to count (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `OptionUsize` - The count of values for the header.
    #[inline(always)]
    pub fn try_get_header_length<K>(&self, key: K) -> OptionUsize
    where
        K: AsRef<str>,
    {
        self.headers.get(key.as_ref()).map(|values| values.len())
    }

    /// Gets the number of values for a specific header key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key to count (must implement AsRef<str>).
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

    /// Gets the total number of header values in the response.
    ///
    /// This counts all values across all headers, so a header with multiple values
    /// will contribute more than one to the total count.
    ///
    /// # Returns
    ///
    /// - `usize` - The total count of all header values.
    #[inline(always)]
    pub fn get_headers_values_length(&self) -> usize {
        self.headers.values().map(|values| values.len()).sum()
    }

    /// Retrieves the body content of the response as a UTF-8 encoded string.
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

    /// Deserializes the body content of the response as_ref a specified type `T`.
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
    /// - `ResultJsonError<T>` - The deserialization result.
    pub fn try_get_body_json<T>(&self) -> ResultJsonError<T>
    where
        T: DeserializeOwned,
    {
        json_from_slice(self.get_body())
    }

    /// Deserializes the body content of the response as_ref a specified type `T`.
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

    /// Determines whether the header should be skipped during setting.
    ///
    /// - Returns `true` if the header is empty or not allowed.
    /// - Returns `false` if the header can be set.
    #[inline(always)]
    fn should_skip_header(&self, key: &ResponseHeadersKey) -> bool {
        key.trim().is_empty() || key == CONTENT_LENGTH
    }

    /// Sets a header in the response, replacing any existing values.
    ///
    /// This function replaces all existing values for a header with a single new value.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key (must implement AsRef<str>).
    /// - `AsRef<str>` - The header value (must implement AsRef<String>).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    fn set_header_without_check<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut deque: VecDeque<String> = VecDeque::with_capacity(1);
        deque.push_back(value.as_ref().to_owned());
        self.headers.insert(key.as_ref().to_owned(), deque);
        self
    }

    /// Sets a header in the response, replacing any existing values.
    ///
    /// This function replaces all existing values for a header with a single new value.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key (must implement AsRef<str>).
    /// - `AsRef<str>` - The header value (must implement AsRef<String>).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key: ResponseHeadersKey = key.as_ref().to_owned();
        if self.should_skip_header(&key) {
            return self;
        }
        let mut deque: VecDeque<String> = VecDeque::with_capacity(1);
        deque.push_back(value.as_ref().to_owned());
        self.headers.insert(key, deque);
        self
    }

    /// Adds a header to the response.
    ///
    /// This function appends a value to the response headers.
    /// If the header already exists, the new value will be added to the existing values.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key (must implement AsRef<str>).
    /// - `AsRef<str>` - The header value (must implement AsRef<String>).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn add_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key: ResponseHeadersKey = key.as_ref().to_owned();
        if self.should_skip_header(&key) {
            return self;
        }
        self.headers
            .entry(key)
            .or_default()
            .push_back(value.as_ref().to_owned());
        self
    }

    /// Removes a header from the response.
    ///
    /// This function removes all values for the specified header key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The header key to remove (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn remove_header<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        let _ = self.headers.remove(key.as_ref()).is_some();
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
    /// - `AsRef<str>` - The header key (must implement AsRef<str>).
    /// - `AsRef<str>` - The value to remove (must implement AsRef<String>).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn remove_header_value<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key: ResponseHeadersKey = key.as_ref().to_owned();
        if let Some(values) = self.headers.get_mut(&key) {
            values.retain(|v| v != &value.as_ref().to_owned());
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
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn clear_headers(&mut self) -> &mut Self {
        self.headers.clear();
        self
    }

    /// Sets the body of the response.
    ///
    /// This method allows you to set the body of the response by converting the provided
    /// value as_ref a `ResponseBody` type. The `body` is updated with the converted value.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The body content (must implement AsRef<[u8]>).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn set_body<T>(&mut self, body: T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.body = body.as_ref().to_owned();
        self
    }

    /// Sets the reason phrase of the response.
    ///
    /// This method allows you to set the reason phrase of the response by converting the
    /// provided value as_ref a `ResponseReasonPhrase` type. The `reason_phrase` is updated
    /// with the converted value.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The reason phrase (must implement AsRef<str>).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    #[inline(always)]
    pub fn set_reason_phrase<T>(&mut self, reason_phrase: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.reason_phrase = reason_phrase.as_ref().to_owned();
        self
    }

    /// Pushes a header with a key and value as_ref the response string.
    ///
    /// # Arguments
    ///
    /// - `&mut String`: A mutable reference to the string where the header will be added.
    /// - `&str`: The header key as a string slice (`&str`).
    /// - `&str`: The header value as a string slice (`&str`).
    #[inline(always)]
    fn push_header(response_string: &mut String, key: &str, value: &str) {
        response_string.push_str(key);
        response_string.push_str(COLON);
        response_string.push_str(value);
        response_string.push_str(HTTP_BR);
    }

    /// Pushes the first line of an HTTP response (version, status code, and reason phrase) as_ref the response string.
    /// This corresponds to the status line of the HTTP response.
    ///
    /// # Arguments
    ///
    /// - `response_string`: A mutable reference to the string where the first line will be added.
    #[inline(always)]
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
    /// - `ResponseData` - The complete HTTP response bytes.
    pub fn build(&mut self) -> ResponseData {
        if self.reason_phrase.is_empty() {
            self.set_reason_phrase(HttpStatus::phrase(*self.get_status_code()));
        }
        let mut response_string: String = String::with_capacity(DEFAULT_BUFFER_SIZE);
        self.push_http_first_line(&mut response_string);
        let compress_type_opt: OptionCompress = self
            .try_get_header_back(CONTENT_ENCODING)
            .map(|value| value.parse::<Compress>().unwrap_or_default());
        if self.try_get_header_back(CONNECTION).is_none() {
            self.set_header_without_check(CONNECTION, KEEP_ALIVE);
        }
        let content_type: ResponseHeadersValueItem =
            self.try_get_header_back(CONTENT_TYPE).unwrap_or_else(|| {
                let mut content_type: String = String::with_capacity(
                    TEXT_HTML.len() + SEMICOLON_SPACE.len() + CHARSET_UTF_8.len(),
                );
                content_type.push_str(TEXT_HTML);
                content_type.push_str(SEMICOLON_SPACE);
                content_type.push_str(CHARSET_UTF_8);
                self.set_header_without_check(CONTENT_TYPE, &content_type);
                content_type
            });
        let mut body: ResponseBody = self.get_body().clone();
        if let Some(compress_type) = compress_type_opt
            && !compress_type.is_unknown()
        {
            body = compress_type
                .encode(&body, DEFAULT_BUFFER_SIZE)
                .into_owned();
        }
        if !content_type.eq_ignore_ascii_case(TEXT_EVENT_STREAM) {
            self.set_header_without_check(CONTENT_LENGTH, body.len().to_string());
        }
        self.get_headers().iter().for_each(|(key, values)| {
            for value in values.iter() {
                Self::push_header(&mut response_string, key, value);
            }
        });
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
    #[inline(always)]
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
