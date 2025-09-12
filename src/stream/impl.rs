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
    /// - `Into<ResponseData>` - The response data to send (must implement Into<ResponseData>).
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result indicating success or failure.
    pub async fn send<D>(&self, data: D) -> ResponseResult
    where
        D: Into<ResponseData>,
    {
        self.write()
            .await
            .write_all(&data.into())
            .await
            .map_err(|err| ResponseError::Response(err.to_string()))?;
        Ok(())
    }

    /// Sends HTTP response body.
    ///
    /// # Arguments
    ///
    /// - `Into<ResponseBody>` - The response body data (must implement Into<ResponseBody>).
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result indicating success or failure.
    pub async fn send_body<D>(&self, data: D) -> ResponseResult
    where
        D: Into<ResponseBody>,
    {
        self.write()
            .await
            .write_all(&data.into())
            .await
            .map_err(|err| ResponseError::Response(err.to_string()))?;
        Ok(())
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
