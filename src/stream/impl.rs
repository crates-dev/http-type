use crate::*;

impl ArcRwLockStream {
    /// Creates a new `ArcRwLockStream` from an `Arc<RwLock<TcpStream>>`.
    ///
    /// # Arguments
    ///
    /// - `arc_rw_lock_stream`: An `Arc<RwLock<TcpStream>>` that will be wrapped in the new `ArcRwLockStream`.
    ///
    /// # Returns
    ///
    /// A new `ArcRwLockStream` instance containing the provided stream.
    pub fn from(arc_rw_lock_stream: ArcRwLock<TcpStream>) -> Self {
        Self(arc_rw_lock_stream)
    }

    /// Creates a new `ArcRwLockStream` from a `TcpStream`.
    ///
    /// This method wraps the provided `TcpStream` in an `Arc<RwLock<_>>` to create the `ArcRwLockStream`.
    ///
    /// # Arguments
    ///
    /// - `stream`: A `TcpStream` that will be wrapped in the new `ArcRwLockStream`.
    ///
    /// # Returns
    ///
    /// A new `ArcRwLockStream` instance containing the provided stream.
    pub fn from_stream(stream: TcpStream) -> Self {
        Self(arc_rwlock(stream))
    }

    /// Returns a reference to the inner `TcpStream`.
    ///
    /// This method acquires a read lock on the underlying stream, allowing shared access
    /// to the TCP stream while preventing concurrent writes.
    ///
    /// # Returns
    ///
    /// A read guard that provides shared access to the TCP stream.
    pub async fn read(&self) -> RwLockReadGuardTcpStream {
        self.get_0().read().await
    }

    /// Returns a mutable reference to the inner `TcpStream`.
    ///
    /// This method acquires a write lock on the underlying stream, allowing exclusive access
    /// for writing operations while preventing any concurrent access.
    ///
    /// # Returns
    ///
    /// A write guard that provides exclusive access to the TCP stream.
    pub(crate) async fn write(&self) -> RwLockWriteGuardTcpStream {
        self.get_0().write().await
    }

    /// Sends the HTTP response over a TCP stream.
    ///
    /// # Arguments
    ///
    /// - `data`: The response data to be sent.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the response is successfully sent.
    /// - `Err(ResponseError)`: If an error occurs during sending.
    pub async fn send(&self, data: &ResponseData) -> ResponseResult {
        self.write()
            .await
            .write_all(&data)
            .await
            .map_err(|err| ResponseError::Response(err.to_string()))?;
        Ok(())
    }

    /// Sends the HTTP or WebSocket response body over a TCP stream.
    ///
    /// This method handles the conditional framing of the body data based on whether
    /// it's for a WebSocket connection.
    ///
    /// # Arguments
    ///
    /// - `body`: The response body to be sent.
    /// - `is_websocket`: A boolean indicating if the connection is a WebSocket connection.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the response body is successfully sent.
    /// - `Err(ResponseError)`: If an error occurs during sending.
    pub async fn send_body_conditional(
        &self,
        body: &ResponseBody,
        is_websocket: bool,
    ) -> ResponseResult {
        let body_list: Vec<ResponseBody> = if is_websocket {
            WebSocketFrame::create_response_frame_list(body)
        } else {
            vec![body.clone()]
        };
        let mut stream: RwLockWriteGuardTcpStream = self.write().await;
        for tmp_body in body_list {
            stream
                .write_all(&tmp_body)
                .await
                .map_err(|err| ResponseError::Response(err.to_string()))?;
        }
        Ok(())
    }

    /// Sends the HTTP response body over a TCP stream.
    ///
    /// This is a convenience method that calls `send_body_conditional` with `is_websocket` set to `false`.
    ///
    /// # Arguments
    ///
    /// - `body`: The response body to be sent.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the response body is successfully sent.
    /// - `Err(ResponseError)`: If an error occurs during sending.
    pub async fn send_body(&self, body: &ResponseBody) -> ResponseResult {
        self.send_body_conditional(body, false).await
    }

    /// Sends the HTTP WebSocket response body over a TCP stream.
    ///
    /// This is a convenience method that calls `send_body_conditional` with `is_websocket` set to `true`.
    ///
    /// # Arguments
    ///
    /// - `body`: The response body to be sent.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the response body is successfully sent.
    /// - `Err(ResponseError)`: If an error occurs during sending.
    pub async fn send_ws_body(&self, body: &ResponseBody) -> ResponseResult {
        self.send_body_conditional(body, true).await
    }

    /// Flushes the TCP stream.
    ///
    /// This ensures that all buffered data is written to the underlying stream.
    ///
    /// # Returns
    ///
    /// - `&Self`: A reference to the `ArcRwLockStream` instance, allowing for method chaining.
    pub async fn flush(&self) -> &Self {
        let _ = self.write().await.flush();
        self
    }
}
