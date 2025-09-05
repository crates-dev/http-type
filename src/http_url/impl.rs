use crate::*;

impl HttpUrlComponents {
    /// Parses a URL string into its components.
    ///
    /// Extracts protocol, host, port, path, query and fragment from the URL string.
    ///
    /// # Arguments
    ///
    /// - `&str` - The URL string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<HttpUrlComponents, HttpUrlError>` - Either the parsed components or an error.
    pub fn parse(url_str: &str) -> Result<Self, HttpUrlError> {
        let parsed_url: UrlParser =
            UrlParser::parse(url_str).map_err(|_| HttpUrlError::InvalidUrl)?;
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
