use super::{error::Error, r#type::Response};
use crate::StatusCode;
use http_constant::*;
use std::{borrow::Cow, collections::HashMap, io::Write, net::TcpStream};

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Response<'a> {
    /// Creates a new instance of `Response`.
    ///
    /// # Returns
    /// - An initialized `Response` with default values.
    pub fn new() -> Self {
        Response {
            version: Cow::Borrowed(HTTP_VERSION_1_1),
            status_code: 200,
            reason_phrase: Cow::Borrowed(OK),
            headers: HashMap::new(),
            body: Vec::new(),
            response: Vec::new(),
        }
    }

    /// Sets the HTTP version for the response.
    ///
    /// # Parameters
    /// - `version`: The HTTP version to set (e.g., HTTP/1.1).
    ///
    /// # Returns
    /// - A mutable reference to the `Response` for chaining.
    pub fn version<S: Into<Cow<'a, str>>>(&mut self, version: S) -> &mut Self {
        self.version = version.into();
        self
    }

    /// Sets the HTTP status code for the response.
    ///
    /// # Parameters
    /// - `code`: The status code to set (e.g., 404).
    ///
    /// # Returns
    /// - A mutable reference to the `Response` for chaining.
    pub fn status_code(&mut self, code: usize) -> &mut Self {
        self.status_code = code;
        self.reason_phrase(StatusCode::phrase(code));
        self
    }

    /// Sets the reason phrase for the response.
    ///
    /// # Parameters
    /// - `phrase`: The reason phrase to set (e.g., Not Found).
    ///
    /// # Returns
    /// - A mutable reference to the `Response` for chaining.
    pub fn reason_phrase<S: Into<Cow<'a, str>>>(&mut self, phrase: S) -> &mut Self {
        self.reason_phrase = phrase.into();
        self
    }

    /// Adds a header to the response.
    ///
    /// # Parameters
    /// - `key`: The name of the header.
    /// - `value`: The value of the header.
    ///
    /// # Returns
    /// - A mutable reference to the `Response` for chaining.
    pub fn header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Sets the body of the response.
    ///
    /// # Parameters
    /// - `body`: The body content as a byte vector.
    ///
    /// # Returns
    /// - A mutable reference to the `Response` for chaining.
    pub fn body<B: Into<Vec<u8>>>(&mut self, body: B) -> &mut Self {
        self.body = body.into();
        self
    }

    /// Builds the full HTTP response as a byte vector.
    ///
    /// # Returns
    /// - The serialized HTTP response including headers and body.
    pub fn build(&mut self) -> Vec<u8> {
        let mut response_str: String = String::new();
        response_str.push_str(&format!(
            "{}{}{}{}{}{}",
            self.version, SPACE, self.status_code, SPACE, self.reason_phrase, HTTP_BR
        ));
        for (key, value) in &self.headers {
            response_str.push_str(&format!("{}{}{}{}", key, COLON_SPACE, value, HTTP_BR));
        }
        response_str.push_str(HTTP_BR);
        let mut response_bytes: Vec<u8> = response_str.into_bytes();
        response_bytes.extend_from_slice(&self.body);
        self.response = response_bytes.clone();
        response_bytes
    }

    /// Sends the HTTP response over a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A mutable reference to the `TcpStream` to send the response.
    ///
    /// # Returns
    /// - `Ok`: If the response is successfully sent.
    /// - `Err`: If an error occurs during sending.
    pub fn send(&mut self, mut stream: &TcpStream) -> Result<(), Error> {
        if self.response.is_empty() {
            self.response = self.build();
        }
        stream
            .write_all(&self.response)
            .map_err(|err| Error::ResponseError(err.to_string()))
            .and_then(|_| Ok(()))
    }
}
