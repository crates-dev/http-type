use crate::*;

/// Implements the `std::error::Error` trait for `HttpUrlError`.
impl std::error::Error for HttpUrlError {}

/// Implements the `Display` trait for `HttpUrlError`, allowing it to be formatted as a string.
impl Display for HttpUrlError {
    /// Formats the `HttpUrlError` variant into a human-readable string.
    ///
    /// # Arguments
    ///
    /// - `f` - The formatter to write the string into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpUrlError::InvalidUrl => write!(f, "Invalid URL"),
            HttpUrlError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl HttpUrlComponents {
    /// Parses a URL string into its components.
    ///
    /// Extracts protocol, host, port, path, query and fragment from the URL string.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The URL string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<HttpUrlComponents, HttpUrlError>` - Either the parsed components or an error.
    #[inline]
    pub fn parse<U>(url_str: U) -> Result<Self, HttpUrlError>
    where
        U: AsRef<str>,
    {
        let parsed_url: Url = Url::parse(url_str.as_ref()).map_err(|_| HttpUrlError::InvalidUrl)?;
        let res: Self = Self {
            protocol: parsed_url
                .scheme()
                .to_string()
                .parse::<Protocol>()
                .unwrap_or_default(),
            host: parsed_url.host_str().map(|h| h.to_string()),
            port: parsed_url.port(),
            path: Some(parsed_url.path().to_string()),
            query: parsed_url.query().map(|q| q.to_string()),
            fragment: parsed_url.fragment().map(|f| f.to_string()),
        };
        Ok(res)
    }
}
