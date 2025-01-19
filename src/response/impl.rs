use super::{error::Error, r#type::Response};
use crate::{CloseStreamResult, ResponseData, ResponseResult, StatusCode};
use http_constant::*;
use std::{collections::HashMap, io::Write, net::TcpStream};
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

    /// Builds the full HTTP response as a byte vector.
    ///
    /// # Returns
    /// - The serialized HTTP response including headers and body.
    #[inline]
    pub fn build(&mut self) -> ResponseData {
        if self.reason_phrase.is_empty() {
            self.set_reason_phrase(StatusCode::phrase(*self.get_status_code()).into());
        }
        let mut response_str: String = String::new();
        response_str.push_str(&format!(
            "{}{}{}{}{}{}",
            self.get_version(),
            SPACE,
            self.get_status_code(),
            SPACE,
            self.get_reason_phrase(),
            HTTP_BR
        ));
        for (key, value) in self.get_headers() {
            response_str.push_str(&format!("{}{}{}{}", key, COLON_SPACE, value, HTTP_BR));
        }
        response_str.push_str(HTTP_BR);
        let mut response_bytes: Vec<u8> = response_str.into_bytes();
        response_bytes.extend_from_slice(self.get_body());
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
            .write_all(&self.get_body())
            .and_then(|_| stream.flush())
            .map_err(|err| Error::ResponseError(err.to_string()))
            .and_then(|_| Ok(self.get_body()))
            .cloned();
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
        let response: ResponseData = self.build();
        self.set_response(response);
        let send_res: ResponseResult = stream
            .write_all(&self.response)
            .and_then(|_| stream.flush())
            .map_err(|err| Error::ResponseError(err.to_string()))
            .and_then(|_| Ok(self.get_response()))
            .cloned();
        send_res
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
            .write_all(&self.get_body())
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        Ok(self.get_response().clone())
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
        let response: ResponseData = self.build();
        self.set_response(response);
        stream
            .write_all(&self.response)
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        Ok(self.get_response().clone())
    }
}
