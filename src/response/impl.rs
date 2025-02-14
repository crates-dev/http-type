use super::error::Error;
use crate::*;
use http_compress::*;
use std::{borrow::Cow, collections::HashMap, io::Write, net::TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream as TokioTcpStream;

impl Default for Response {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    /// Creates a new instance of `Response`.
    ///
    /// # Returns
    /// - An initialized `Response` with default values.
    #[inline]
    pub fn new() -> Self {
        Response {
            version: HTTP_VERSION_1_1.to_owned(),
            status_code: 200,
            reason_phrase: EMPTY_STR.to_owned(),
            headers: HashMap::new(),
            body: Vec::new(),
            response: Vec::new(),
        }
    }

    /// Adds a header to the response.
    ///
    /// This function inserts a key-value pair into the response headers.
    /// The key and value are converted into `String`, allowing for efficient handling of both owned and borrowed string data.
    ///
    /// # Parameters
    /// - `key`: The header key, which will be converted into a `String`.
    /// - `value`: The value of the header, which will be converted into a `String`.
    ///
    /// # Returns
    /// - Returns a mutable reference to the current instance (`&mut Self`), allowing for method chaining.
    #[inline]
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
    /// value into a `ResponseBody` type. The `body` is updated with the converted value,
    /// and the method returns a mutable reference to the current instance for method chaining.
    ///
    /// # Parameters
    /// - `body`: The body of the response to be set. It can be any type that can be converted
    ///   into a `ResponseBody` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    /// Set the body of the response.
    ///
    /// This method allows you to set the body of the response by converting the provided
    /// value into a `ResponseBody` type. The `body` is updated with the converted value,
    /// and the method returns a mutable reference to the current instance for method chaining.
    ///
    /// # Parameters
    /// - `body`: The body of the response to be set. It can be any type that can be converted
    ///   into a `ResponseBody` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    #[inline]
    pub fn set_body<T: Into<ResponseBody>>(&mut self, body: T) -> &mut Self {
        self.body = body.into();
        self
    }

    /// Set the reason phrase of the response.
    ///
    /// This method allows you to set the reason phrase of the response by converting the
    /// provided value into a `ResponseReasonPhrase` type. The `reason_phrase` is updated
    /// with the converted value, and the method returns a mutable reference to the current
    /// instance for method chaining.
    ///
    /// # Parameters
    /// - `reason_phrase`: The reason phrase to be set for the response. It can be any type
    ///   that can be converted into a `ResponseReasonPhrase` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    #[inline]
    pub fn set_reason_phrase<T: Into<ResponseReasonPhrase>>(
        &mut self,
        reason_phrase: T,
    ) -> &mut Self {
        self.reason_phrase = reason_phrase.into();
        self
    }

    /// Pushes a header with a key and value into the response string.
    ///
    /// # Parameters
    /// - `response_string`: A mutable reference to the string where the header will be added.
    /// - `key`: The header key as a string slice (`&str`).
    /// - `value`: The header value as a string slice (`&str`).
    #[inline]
    pub(super) fn push_header(response_string: &mut String, key: &str, value: &str) {
        response_string.push_str(&format!("{}{}{}{}", key, COLON_SPACE, value, HTTP_BR));
    }

    /// Pushes the first line of an HTTP response (version, status code, and reason phrase) into the response string.
    /// This corresponds to the status line of the HTTP response.
    ///
    /// # Parameters
    /// - `response_string`: A mutable reference to the string where the first line will be added.
    #[inline]
    pub(super) fn push_http_response_first_line(&self, response_string: &mut String) {
        response_string.push_str(&format!(
            "{}{}{}{}{}{}",
            self.get_version(),
            SPACE,
            self.get_status_code(),
            SPACE,
            self.get_reason_phrase(),
            HTTP_BR
        ));
    }

    /// Builds the full HTTP response as a byte vector.
    ///
    /// # Returns
    /// - The serialized HTTP response including headers and body.
    #[inline]
    pub fn build(&mut self) -> ResponseData {
        if self.reason_phrase.is_empty() {
            self.set_reason_phrase(StatusCode::phrase(*self.get_status_code()));
        }
        let mut response_string: String = String::new();
        self.push_http_response_first_line(&mut response_string);
        let mut compress_type_opt: Option<Compress> = None;
        let mut connection_opt: Option<&str> = None;
        let mut content_type_opt: Option<&str> = None;
        for (key, value) in self.get_headers() {
            if key == CONTENT_LENGTH {
                continue;
            } else if key == CONTENT_ENCODING {
                compress_type_opt = Some(value.parse::<Compress>().unwrap_or_default());
            } else if key == CONNECTION {
                connection_opt = Some(value);
            } else if key == CONTENT_TYPE {
                content_type_opt = Some(value);
            }
            Self::push_header(&mut response_string, key, &value);
        }
        if connection_opt.is_none() {
            Self::push_header(&mut response_string, CONNECTION, CONNECTION_KEEP_ALIVE);
        }
        if content_type_opt.is_none() {
            Self::push_header(
                &mut response_string,
                CONTENT_TYPE,
                &format!("{}{}{}", TEXT_HTML, SEMICOLON_SPACE, CHARSET_UTF_8),
            );
        }
        let mut body: Cow<Vec<u8>> = Cow::Borrowed(self.get_body());
        if let Some(compress_type) = compress_type_opt {
            if !compress_type.is_unknown() {
                let tmp_body: Cow<'_, Vec<u8>> = compress_type.encode(&body, DEFAULT_BUFFER_SIZE);
                body = Cow::Owned(tmp_body.into_owned());
            }
        }
        let len_string: String = body.len().to_string();
        let len_str: &str = len_string.as_str();
        Self::push_header(&mut response_string, CONTENT_LENGTH, len_str);
        response_string.push_str(HTTP_BR);
        let mut response_bytes: Vec<u8> = response_string.into_bytes();
        response_bytes.extend_from_slice(&body);
        self.set_response(response_bytes.clone());
        response_bytes
    }

    /// Sends the HTTP response body over a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A mutable reference to the `TcpStream` to send the response.
    ///
    /// # Returns
    /// - `Ok`: If the response body is successfully sent.
    /// - `Err`: If an error occurs during sending.
    #[inline]
    pub fn send_body(&mut self, mut stream: &TcpStream) -> ResponseResult {
        let send_res: ResponseResult = stream
            .write_all(self.get_body())
            .and_then(|_| stream.flush())
            .map_err(|err| Error::ResponseError(err.to_string()))
            .and_then(|_| Ok(()));
        send_res
    }

    /// Closes the stream after sending the response.
    ///
    /// This function is responsible for:
    /// - Building the response using the `build()` method.
    /// - Setting the response using the `set_response()` method.
    /// - Shutting down the write half of the TCP stream to indicate no more data will be sent.
    ///
    /// # Parameters
    /// - `stream`: A reference to the `TcpStream` that will be closed after sending the response.
    ///
    /// # Returns
    /// - `CloseStreamResult`: The result of the operation, indicating whether the closure was successful or if an error occurred.
    #[inline]
    pub fn close(&mut self, stream: &TcpStream) -> CloseStreamResult {
        let _ = stream
            .shutdown(std::net::Shutdown::Both)
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        Ok(())
    }

    /// Sends the HTTP response over a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A mutable reference to the `TcpStream` to send the response.
    ///
    /// # Returns
    /// - `Ok`: If the response is successfully sent.
    /// - `Err`: If an error occurs during sending.
    #[inline]
    pub fn send(&mut self, mut stream: &TcpStream) -> ResponseResult {
        self.build();
        stream
            .write_all(self.get_response())
            .and_then(|_| stream.flush())
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        Ok(())
    }

    /// Sends the HTTP response body over a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A mutable reference to the `TcpStream` to send the response.
    ///
    /// # Returns
    /// - `Ok`: If the response body is successfully sent.
    /// - `Err`: If an error occurs during sending.
    #[inline]
    pub async fn async_send_body(&mut self, stream: &mut TokioTcpStream) -> ResponseResult {
        stream
            .write_all(self.get_body())
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        Ok(())
    }

    /// Closes the stream after sending the response.
    ///
    /// This function is responsible for:
    /// - Building the response using the `build()` method.
    /// - Setting the response using the `set_response()` method.
    /// - Shutting down the write half of the TCP stream to indicate no more data will be sent.
    ///
    /// # Parameters
    /// - `stream`: A reference to the `TcpStream` that will be closed after sending the response.
    ///
    /// # Returns
    /// - `CloseStreamResult`: The result of the operation, indicating whether the closure was successful or if an error occurred.
    #[inline]
    pub fn async_close(&mut self, stream: &mut TokioTcpStream) -> CloseStreamResult {
        let _ = stream.shutdown();
        Ok(())
    }

    /// Sends the HTTP response over a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A mutable reference to the `TcpStream` to send the response.
    ///
    /// # Returns
    /// - `Ok`: If the response is successfully sent.
    /// - `Err`: If an error occurs during sending.
    #[inline]
    pub async fn async_send(&mut self, stream: &mut TokioTcpStream) -> ResponseResult {
        self.build();
        stream
            .write_all(&self.get_response())
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        Ok(())
    }
}
