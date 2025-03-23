use crate::*;

impl ArcRwLockStream {
    pub fn from(arc_rw_lock_stream: ArcRwLock<TcpStream>) -> Self {
        Self(arc_rw_lock_stream)
    }

    pub fn from_stream(stream: TcpStream) -> Self {
        Self(arc_rwlock(stream))
    }

    pub async fn get_read_lock(&self) -> RwLockReadGuardTcpStream {
        self.0.read().await
    }

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
        let mut stream: RwLockWriteGuardTcpStream = self.get_write_lock().await;
        stream
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
        let mut stream: RwLockWriteGuardTcpStream = self.get_write_lock().await;
        let body_list: Vec<ResponseBody> = if is_websocket {
            WebSocketFrame::create_response_frame_list(body)
        } else {
            vec![body.clone()]
        };
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
        let mut stream: RwLockWriteGuardTcpStream = self.get_write_lock().await;
        stream
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
        let mut stream: RwLockWriteGuardTcpStream = self.get_write_lock().await;
        stream
            .shutdown()
            .await
            .map_err(|err| ResponseError::CloseError(err.to_string()))
    }
}
