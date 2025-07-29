use crate::*;

impl ArcRwLockStream {
    /// Creates a new ArcRwLockStream from an Arc<RwLock<TcpStream>>.
    ///
    /// # Arguments
    ///
    /// - `Arc<RwLock<TcpStream>>` - The stream to wrap.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockStream` - The new stream wrapper.
    pub fn from(arc_rw_lock_stream: ArcRwLock<TcpStream>) -> Self {
        Self(arc_rw_lock_stream)
    }

    /// Creates a new ArcRwLockStream from a TcpStream.
    ///
    /// Wraps the stream in an Arc<RwLock<_>>.
    ///
    /// # Arguments
    ///
    /// - `TcpStream` - The raw stream to wrap.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockStream` - The new thread-safe stream wrapper.
    pub fn from_stream(stream: TcpStream) -> Self {
        Self(arc_rwlock(stream))
    }

    /// Gets a read lock on the inner TcpStream.
    ///
    /// Allows shared read access to the stream.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuardTcpStream` - The read guard for the stream.
    pub async fn read(&self) -> RwLockReadGuardTcpStream {
        self.get_0().read().await
    }

    /// Gets a write lock on the inner TcpStream.
    ///
    /// Allows exclusive write access to the stream.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuardTcpStream` - The write guard for the stream.
    pub(crate) async fn write(&self) -> RwLockWriteGuardTcpStream {
        self.get_0().write().await
    }

    /// Sends HTTP response data over the stream.
    ///
    /// # Arguments
    ///
    /// - `&ResponseData` - The response data to send.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result indicating success or failure.
    pub async fn send(&self, data: &ResponseData) -> ResponseResult {
        self.write()
            .await
            .write_all(&data)
            .await
            .map_err(|err| ResponseError::Response(err.to_string()))?;
        Ok(())
    }

    /// Sends response body with WebSocket framing condition.
    ///
    /// Handles both HTTP and WebSocket response formats.
    ///
    /// # Arguments
    ///
    /// - `&ResponseBody` - The response body data.
    /// - `bool` - Whether to use WebSocket framing.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result indicating success or failure.
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

    /// Sends HTTP response body (non-WebSocket).
    ///
    /// Convenience wrapper for send_body_conditional with WebSocket disabled.
    ///
    /// # Arguments
    ///
    /// - `&ResponseBody` - The response body data.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result indicating success or failure.
    pub async fn send_body(&self, body: &ResponseBody) -> ResponseResult {
        self.send_body_conditional(body, false).await
    }

    /// Sends WebSocket response body.
    ///
    /// Convenience wrapper for send_body_conditional with WebSocket enabled.
    ///
    /// # Arguments
    ///
    /// - `&ResponseBody` - The WebSocket frame data.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result indicating success or failure.
    pub async fn send_ws_body(&self, body: &ResponseBody) -> ResponseResult {
        self.send_body_conditional(body, true).await
    }

    /// Flushes all buffered data to the stream.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn flush(&self) -> &Self {
        let _ = self.write().await.flush();
        self
    }
}
