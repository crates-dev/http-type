use crate::*;

impl ArcRwLockStream {
    /// Creates a new `ArcRwLockStream` from an `Arc<RwLock<TcpStream>>`.
    ///
    /// # Parameters
    /// - `arc_rw_lock_stream`: An `Arc<RwLock<TcpStream>>` that will be wrapped in the new `ArcRwLockStream`
    ///
    /// # Returns
    /// Returns a new `ArcRwLockStream` instance containing the provided stream
    pub fn from(arc_rw_lock_stream: ArcRwLock<TcpStream>) -> Self {
        Self(arc_rw_lock_stream)
    }

    /// Creates a new `ArcRwLockStream` from a `TcpStream`.
    ///
    /// # Parameters
    /// - `stream`: A `TcpStream` that will be wrapped in the new `ArcRwLockStream`
    ///
    /// # Returns
    /// Returns a new `ArcRwLockStream` instance containing the provided stream wrapped in an `Arc<RwLock<_>>`
    pub fn from_stream(stream: TcpStream) -> Self {
        Self(arc_rwlock(stream))
    }

    /// Returns a reference to the inner `TcpStream`.
    ///
    /// This method acquires a read lock on the underlying stream, allowing shared access
    /// to the TCP stream while preventing concurrent writes.
    ///
    /// # Returns
    /// Returns a read guard that provides shared access to the TCP stream
    pub async fn get_read_lock(&self) -> RwLockReadGuardTcpStream {
        self.0.read().await
    }

    /// Returns a mutable reference to the inner `TcpStream`.
    ///
    /// This method acquires a write lock on the underlying stream, allowing exclusive access
    /// for writing operations while preventing any concurrent access.
    ///
    /// # Returns
    /// Returns a write guard that provides exclusive access to the TCP stream
    pub async fn get_write_lock(&self) -> RwLockWriteGuardTcpStream {
        self.0.write().await
    }

    /// Sends the HTTP response over a TCP stream.
    ///
    /// # Parameters
    /// - `data`: Response data
    ///
    /// # Returns
    /// - `Ok`: If the response is successfully sent.
    /// - `Err`: If an error occurs during sending.
    pub async fn send(&self, data: &ResponseData) -> ResponseResult {
        self.get_write_lock()
            .await
            .write_all(&data)
            .await
            .map_err(|err| ResponseError::ResponseError(err.to_string()))?;
        Ok(())
    }

    /// Sends the HTTP response body over a TCP stream.
    ///
    /// # Parameters
    /// - `body`: Response body.
    /// - `is_websocket`: Is websocket
    ///
    /// # Returns
    /// - `Ok`: If the response body is successfully sent.
    /// - `Err`: If an error occurs during sending.
    pub async fn send_body(&self, body: &ResponseBody, is_websocket: bool) -> ResponseResult {
        let body_list: Vec<ResponseBody> = if is_websocket {
            WebSocketFrame::create_response_frame_list(body)
        } else {
            vec![body.clone()]
        };
        let mut stream: RwLockWriteGuardTcpStream = self.get_write_lock().await;
        for tmp_body in body_list {
            stream
                .write_all(&tmp_body)
                .await
                .map_err(|err| ResponseError::ResponseError(err.to_string()))?;
        }
        Ok(())
    }

    /// Flush the TCP stream.
    ///
    /// - Returns: A `ResponseResult` indicating success or failure.
    pub async fn flush(&self) -> ResponseResult {
        self.get_write_lock()
            .await
            .flush()
            .await
            .map_err(|err| ResponseError::ResponseError(err.to_string()))?;
        Ok(())
    }

    /// Closes the stream after sending the response.
    ///
    /// This function is responsible for:
    /// - Building the response using the `build()` method.
    /// - Setting the response using the `set_response()` method.
    /// - Shutting down the write half of the TCP stream to indicate no more data will be sent.
    ///
    /// # Returns
    /// - `ResponseResult`: The result of the operation, indicating whether the closure was successful or if an error occurred.
    pub async fn close(&self) -> ResponseResult {
        self.get_write_lock()
            .await
            .shutdown()
            .await
            .map_err(|err| ResponseError::CloseError(err.to_string()))
    }
}
