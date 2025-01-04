use super::error::Error;
use super::r#type::Request;
use crate::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::str::SplitN;

impl Default for Request {
    fn default() -> Self {
        Self {
            method: String::new(),
            host: String::new(),
            path: String::new(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

impl Request {
    /// Creates a new `Request` object from a TCP stream.
    ///
    /// # Parameters
    /// - `stream`: A reference to a `TcpStream` representing the incoming connection.
    ///
    /// # Returns
    /// - `Ok`: A `Request` object populated with the HTTP request data.
    /// - `Err`: An `Error` if the request is invalid or cannot be read.
    pub fn new(stream: &TcpStream) -> RequestNewResult {
        let mut reader: BufReader<&TcpStream> = BufReader::new(stream);
        let mut request_line: String = String::new();
        reader
            .read_line(&mut request_line)
            .map_err(|_| Error::HttpReadError)?;
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(Error::InvalidHttpRequest);
        }
        let method: RequestMethod = parts[0].to_string();
        let full_path: String = parts[1].to_string();
        let hash_index: Option<usize> = full_path.find(HASH_SYMBOL);
        let query_index: Option<usize> = full_path.find(QUERY_SYMBOL);
        let query_string: String = query_index.map_or(EMPTY_STR.to_owned(), |i| {
            let temp: String = full_path[i + 1..].to_string();
            if hash_index.is_none() || hash_index.unwrap() <= i {
                return temp.into();
            }
            let data: String = temp
                .split(HASH_SYMBOL)
                .next()
                .unwrap_or_default()
                .to_string();
            data.into()
        });
        let query: RequestQuery = Self::parse_query(&query_string);
        let path: RequestPath = if let Some(i) = query_index.or(hash_index) {
            full_path[..i].to_string()
        } else {
            full_path
        };
        let mut headers: RequestHeaders = HashMap::new();
        let mut host: RequestHost = EMPTY_STR.to_owned();
        let mut content_length: usize = 0;
        loop {
            let mut header_line: String = String::new();
            reader
                .read_line(&mut header_line)
                .map_err(|_| Error::HttpReadError)?;
            let header_line: &str = header_line.trim();
            if header_line.is_empty() {
                break;
            }
            let parts: Vec<&str> = header_line.splitn(2, COLON_SPACE_SYMBOL).collect();
            if parts.len() != 2 {
                continue;
            }
            let key: String = parts[0].trim().to_string();
            let value: String = parts[1].trim().to_string();
            if key.eq_ignore_ascii_case(HOST) {
                host = value.to_string();
            }
            if key.eq_ignore_ascii_case(CONTENT_LENGTH) {
                content_length = value.parse().unwrap_or(0);
            }
            headers.insert(key, value);
        }
        let mut body: RequestBody = Vec::new();
        if content_length > 0 {
            body.resize(content_length, 0);
            reader
                .read_exact(&mut body)
                .map_err(|_| Error::HttpReadError)?;
        }
        Ok(Request {
            method,
            host,
            path,
            query,
            headers,
            body,
        })
    }

    /// Parse query
    fn parse_query(query: &str) -> RequestQuery {
        let mut query_map: RequestQuery = HashMap::new();
        for pair in query.split(AND) {
            let mut parts: SplitN<'_, &str> = pair.splitn(2, EQUAL);
            let key: String = parts.next().unwrap_or_default().to_string();
            let value: String = parts.next().unwrap_or_default().to_string();
            if !key.is_empty() {
                query_map.insert(key, value);
            }
        }
        query_map
    }

    /// Adds a header to the request.
    ///
    /// This function inserts a key-value pair into the request headers.
    /// The key and value are converted into `String`, allowing for efficient handling of both owned and borrowed string data.
    ///
    /// # Parameters
    /// - `key`: The header key, which will be converted into a `String`.
    /// - `value`: The value of the header, which will be converted into a `String`.
    ///
    /// # Returns
    /// - Returns a mutable reference to the current instance (`&mut Self`), allowing for method chaining.
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }
}
