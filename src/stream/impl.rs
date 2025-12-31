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
    #[inline(always)]
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
    #[inline(always)]
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
    pub async fn read(&'_ self) -> RwLockReadGuardTcpStream<'_> {
        self.get_0().read().await
    }

    /// Gets a write lock on the inner TcpStream.
    ///
    /// Allows exclusive write access to the stream.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuardTcpStream` - The write guard for the stream.
    pub(crate) async fn write(&'_ self) -> RwLockWriteGuardTcpStream<'_> {
        self.get_0().write().await
    }

    /// Sends HTTP response data over the stream.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The response data to send (must implement AsRef<[u8]>).
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send<D>(&self, data: D) -> Result<(), ResponseError>
    where
        D: AsRef<[u8]>,
    {
        self.write()
            .await
            .write_all(data.as_ref())
            .await
            .map_err(|err| ResponseError::Response(err.to_string()))?;
        Ok(())
    }

    /// Sends HTTP response data over the stream.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The response data to send (must implement AsRef<[u8]>).
    ///
    /// # Panics
    ///
    /// Panics if the write operation fails.
    pub async fn send<D>(&self, data: D)
    where
        D: AsRef<[u8]>,
    {
        self.try_send(data).await.unwrap();
    }

    /// Sends HTTP response body.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The response body data (must implement AsRef<[u8]>).
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send_body<D>(&self, data: D) -> Result<(), ResponseError>
    where
        D: AsRef<[u8]>,
    {
        self.write()
            .await
            .write_all(data.as_ref())
            .await
            .map_err(|err| ResponseError::Response(err.to_string()))?;
        Ok(())
    }

    /// Sends HTTP response body.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The response body data (must implement AsRef<[u8]>).
    ///
    /// # Panics
    ///
    /// Panics if the write operation fails.
    pub async fn send_body<D>(&self, data: D)
    where
        D: AsRef<[u8]>,
    {
        self.try_send_body(data).await.unwrap();
    }

    /// Sends multiple HTTP response bodies sequentially.
    ///
    /// # Arguments
    ///
    /// - `I: IntoIterator<Item = D>, D: AsRef<[u8]>` - The response body data list to send.
    ///
    /// # Returns
    ///
    /// - `Result<(), ResponseError>` - Result indicating success or failure.
    pub async fn try_send_body_list<I, D>(&self, data_iter: I) -> Result<(), ResponseError>
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        let mut stream: RwLockWriteGuardTcpStream = self.write().await;
        for data in data_iter {
            stream
                .write_all(data.as_ref())
                .await
                .map_err(|err| ResponseError::Response(err.to_string()))?;
        }
        Ok(())
    }

    /// Sends multiple HTTP response bodies sequentially.
    ///
    /// # Arguments
    ///
    /// - `I: IntoIterator<Item = D>, D: AsRef<[u8]>` - The response body data list to send.
    ///
    /// # Panics
    ///
    /// Panics if any write operation fails.
    pub async fn send_body_list<I, D>(&self, data_iter: I)
    where
        I: IntoIterator<Item = D>,
        D: AsRef<[u8]>,
    {
        let mut stream: RwLockWriteGuardTcpStream = self.write().await;
        for data in data_iter {
            stream.write_all(data.as_ref()).await.unwrap();
        }
    }

    /// Flushes all buffered data to the stream.
    ///
    /// # Returns
    ///
    /// - `Result<(), std::io::Error>` - Result indicating success or failure.
    pub async fn try_flush(&self) -> Result<(), std::io::Error> {
        self.write().await.flush().await
    }

    /// Flushes all buffered data to the stream.
    ///
    /// # Panics
    ///
    /// Panics if the flush operation fails.
    pub async fn flush(&self) {
        self.try_flush().await.unwrap();
    }
}
