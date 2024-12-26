use super::{error::Error, r#type::Response};
use crate::StatusCode;
use http_constant::*;
use std::{borrow::Cow, collections::HashMap, io::Write, net::TcpStream};

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    /// Creates a new instance of `Response`.
    ///
    /// # Returns
    /// - An initialized `Response` with default values.
    pub fn new() -> Self {
        Response {
            version: Cow::Borrowed(HTTP_VERSION_1_1),
            status_code: 200,
            reason_phrase: Cow::Borrowed(EMPTY_STR),
            headers: HashMap::new(),
            body: Vec::new(),
            response: Vec::new(),
        }
    }

    /// Adds a header to the response.
    ///
    /// This function inserts a key-value pair into the response headers.
    /// The key and value are converted into `Cow<'a, str>`, allowing for efficient handling of both owned and borrowed string data.
    ///
    /// # Parameters
    /// - `key`: The header key, which will be converted into a `Cow<'a, str>`.
    /// - `value`: The value of the header, which will be converted into a `Cow<'a, str>`.
    ///
    /// # Returns
    /// - Returns a mutable reference to the current instance (`&mut Self`), allowing for method chaining.
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Builds the full HTTP response as a byte vector.
    ///
    /// # Returns
    /// - The serialized HTTP response including headers and body.
    pub fn build(&mut self) -> Vec<u8> {
        if self.reason_phrase.is_empty() {
            self.set_reason_phrase(StatusCode::phrase(*self.get_status_code()).into());
        }
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
