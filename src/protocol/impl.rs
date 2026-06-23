use crate::*;

/// Implementation of protocol identification and port resolution methods.
///
/// This implementation block provides utility functions for working with
/// HTTP protocol strings, enabling identification of HTTP/HTTPS variants
/// and retrieval of their standard port numbers.
impl Protocol {
    /// Checks if the given protocol string represents HTTP.
    ///
    /// Performs a case-insensitive comparison against the HTTP protocol identifier.
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to check.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the protocol is HTTP (case-insensitive), `false` otherwise.
    #[inline(always)]
    pub fn is_http(protocol: &str) -> bool {
        matches!(protocol.to_lowercase().as_str(), HTTP_LOWERCASE)
    }

    /// Checks if the given protocol string represents HTTPS.
    ///
    /// Performs a case-insensitive comparison against the HTTPS protocol identifier.
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to check.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the protocol is HTTPS (case-insensitive), `false` otherwise.
    #[inline(always)]
    pub fn is_https(protocol: &str) -> bool {
        matches!(protocol.to_lowercase().as_str(), HTTPS_LOWERCASE)
    }

    /// Checks if the given protocol string represents HTTP/2.
    ///
    /// Accepts "h2" (HTTP/2 over TLS) and "h2c" (HTTP/2 cleartext).
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to check.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the protocol is HTTP/2, `false` otherwise.
    #[inline(always)]
    pub fn is_h2(protocol: &str) -> bool {
        matches!(protocol.to_lowercase().as_str(), "h2" | "h2c")
    }

    /// Checks if the given protocol string represents HTTP/2 cleartext (h2c).
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to check.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the protocol is h2c, `false` otherwise.
    #[inline(always)]
    pub fn is_h2c(protocol: &str) -> bool {
        matches!(protocol.to_lowercase().as_str(), "h2c")
    }

    /// Checks if the given protocol string represents HTTP/3.
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to check.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the protocol is HTTP/3, `false` otherwise.
    #[inline(always)]
    pub fn is_h3(protocol: &str) -> bool {
        matches!(protocol.to_lowercase().as_str(), "h3")
    }

    /// Checks if the given protocol string represents HTTP, HTTPS, HTTP/2, or HTTP/3.
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to check.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the protocol is an HTTP family protocol, `false` otherwise.
    #[inline(always)]
    pub fn is_http_family(protocol: &str) -> bool {
        Self::is_http(protocol)
            || Self::is_https(protocol)
            || Self::is_h2(protocol)
            || Self::is_h3(protocol)
    }

    /// Returns the default port number for the given protocol.
    ///
    /// Performs a case-insensitive comparison to determine the protocol type
    /// and returns the corresponding standard port number.
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to lookup.
    ///
    /// # Returns
    /// - `u16`: The default port number for the protocol.
    #[inline(always)]
    pub fn get_port(protocol: &str) -> u16 {
        match protocol.to_lowercase().as_str() {
            HTTP_LOWERCASE => 80,
            HTTPS_LOWERCASE => 443,
            "h2" => 443,
            "h2c" => 80,
            "h3" => 443,
            FTP_LOWERCASE => 21,
            FTPS_LOWERCASE => 990,
            SFTP_LOWERCASE => 22,
            SSH_LOWERCASE => 22,
            TELNET_LOWERCASE => 23,
            SMTP_LOWERCASE => 25,
            SMTPS_LOWERCASE => 465,
            POP3_LOWERCASE => 110,
            POP3S_LOWERCASE => 995,
            IMAP_LOWERCASE => 143,
            IMAPS_LOWERCASE => 993,
            DNS_LOWERCASE => 53,
            WS_LOWERCASE => 80,
            WSS_LOWERCASE => 443,
            _ => 80,
        }
    }

    /// Returns the default port for HTTP/2.
    ///
    /// # Arguments
    /// - `bool`:3 for TLS, 80 for cleartext.
    ///
    /// # Returns
    /// - `u16`: 443 for TLS, 80 for cleartext.
    #[inline(always)]
    pub fn get_h2_port(is_tls: bool) -> u16 {
        if is_tls { 443 } else { 80 }
    }

    /// Returns the default port for HTTP/3.
    ///
    /// # Returns
    /// - `u16`: 443.
    #[inline(always)]
    pub fn get_h3_port() -> u16 {
        443
    }
}
