use crate::*;

/// Implementation for `CookieBuilder`.
impl CookieBuilder {
    /// Creates a new cookie builder instance.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The cookie name type.
    /// - `AsRef<str>` - The cookie value type.
    ///
    /// # Returns
    ///
    /// - `CookieBuilder` - A new builder instance.
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        Self {
            name: name.as_ref().to_owned(),
            value: value.as_ref().to_owned(),
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    /// Parses a `Set-Cookie` header string into a `CookieBuilder`.
    ///
    /// This method takes a `Set-Cookie` header string and extracts the various
    /// attributes of a cookie, populating a `CookieBuilder` instance.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The `Set-Cookie` header string to parse.
    ///
    /// # Returns
    ///
    /// A `CookieBuilder` instance populated with the parsed cookie attributes.
    pub fn parse<C>(cookie: C) -> Self
    where
        C: AsRef<str>,
    {
        let mut cookie_builder: Self = Self::default();
        let parts: Vec<&str> = cookie.as_ref().split(SEMICOLON).collect();
        if parts.is_empty() {
            return cookie_builder;
        }
        if let Some(name_value_pair) = parts.first() {
            let name_value_pair: &str = name_value_pair.trim();
            if let Some((name, value)) = name_value_pair.split_once(EQUAL) {
                cookie_builder.name = name.trim().to_string();
                cookie_builder.value = value.trim().to_string();
            } else if !name_value_pair.is_empty() {
                cookie_builder.name = name_value_pair.to_string();
                cookie_builder.value = String::new();
            }
        }
        for part in parts.iter().skip(1) {
            let part: &str = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((key, value)) = part.split_once(EQUAL) {
                let key_lowercase: String = key.trim().to_lowercase();
                let value: String = value.trim().to_string();
                match key_lowercase.as_str() {
                    COOKIE_EXPIRES_LOWERCASE => {
                        cookie_builder.expires = Some(value);
                    }
                    COOKIE_MAX_AGE_LOWERCASE => {
                        if let Ok(max_age_value) = value.parse::<i64>() {
                            cookie_builder.max_age = Some(max_age_value);
                        }
                    }
                    COOKIE_DOMAIN_LOWERCASE => {
                        cookie_builder.domain = Some(value);
                    }
                    COOKIE_PATH_LOWERCASE => {
                        cookie_builder.path = Some(value);
                    }
                    COOKIE_SAME_SITE_LOWERCASE => {
                        cookie_builder.same_site = Some(value);
                    }
                    _ => {}
                }
            } else {
                let attribute_lowercase: String = part.to_lowercase();
                match attribute_lowercase.as_str() {
                    COOKIE_SECURE_LOWERCASE => {
                        cookie_builder.secure = true;
                    }
                    COOKIE_HTTP_ONLY_LOWERCASE => {
                        cookie_builder.http_only = true;
                    }
                    _ => {}
                }
            }
        }
        cookie_builder
    }

    /// Sets the expiration date for the cookie.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The expiration date string.
    ///
    /// # Returns
    ///
    /// The `CookieBuilder` instance for method chaining.
    pub fn expires<E>(&mut self, expires: E) -> &mut Self
    where
        E: AsRef<str>,
    {
        self.expires = Some(expires.as_ref().to_owned());
        self
    }

    /// Sets the maximum age for the cookie in seconds.
    ///
    /// # Arguments
    ///
    /// - `i64` - The maximum age in seconds.
    ///
    /// # Returns
    ///
    /// The `CookieBuilder` instance for method chaining.
    pub fn max_age(&mut self, max_age: i64) -> &mut Self {
        self.max_age = Some(max_age);
        self
    }

    /// Sets the domain for the cookie.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The domain for the cookie.
    ///
    /// # Returns
    ///
    /// The `CookieBuilder` instance for method chaining.
    pub fn domain<D>(&mut self, domain: D) -> &mut Self
    where
        D: AsRef<str>,
    {
        self.domain = Some(domain.as_ref().to_owned());
        self
    }

    /// Sets the path for the cookie.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The path for the cookie.
    ///
    /// # Returns
    ///
    /// The `CookieBuilder` instance for method chaining.
    pub fn path<T>(&mut self, path: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.path = Some(path.as_ref().to_owned());
        self
    }

    /// Sets the `Secure` flag for the cookie.
    ///
    /// This flag indicates that the cookie should only be transmitted over secure (HTTPS) connections.
    ///
    /// # Returns
    ///
    /// The `CookieBuilder` instance for method chaining.
    pub fn secure(&mut self) -> &mut Self {
        self.secure = true;
        self
    }

    /// Sets the `HttpOnly` flag for the cookie.
    ///
    /// This flag prevents client-side JavaScript from accessing the cookie.
    ///
    /// # Returns
    ///
    /// The `CookieBuilder` instance for method chaining.
    pub fn http_only(&mut self) -> &mut Self {
        self.http_only = true;
        self
    }

    /// Sets the `SameSite` policy for the cookie.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The `SameSite` policy.
    ///
    /// # Returns
    ///
    /// The `CookieBuilder` instance for method chaining.
    pub fn same_site<T>(&mut self, same_site: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.same_site = Some(same_site.as_ref().to_owned());
        self
    }

    /// Builds the cookie string according to the `Set-Cookie` header format.
    ///
    /// # Returns
    ///
    /// - `String` - A formatted cookie string ready to be sent in a `Set-Cookie` header.
    pub fn build(&self) -> String {
        if self.name.is_empty() {
            return String::new();
        }
        let mut cookie_string: String = format!("{}={}", self.name, self.value);
        if let Some(ref expires_value) = self.expires {
            cookie_string.push_str(COOKIE_EXPIRES_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(expires_value);
        }
        if let Some(max_age_value) = self.max_age {
            cookie_string.push_str(COOKIE_MAX_AGE_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(&max_age_value.to_string());
        }
        if let Some(ref domain_value) = self.domain {
            cookie_string.push_str(COOKIE_DOMAIN_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(domain_value);
        }
        if let Some(ref path_value) = self.path {
            cookie_string.push_str(COOKIE_PATH_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(path_value);
        }
        if self.secure {
            cookie_string.push_str(COOKIE_SECURE_ATTRIBUTE_LOWERCASE);
        }
        if self.http_only {
            cookie_string.push_str(COOKIE_HTTP_ONLY_ATTRIBUTE_LOWERCASE);
        }
        if let Some(ref same_site_value) = self.same_site {
            cookie_string.push_str(COOKIE_SAME_SITE_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(same_site_value);
        }
        cookie_string
    }
}

/// Implementation for `Cookie`.
impl Cookie {
    /// Parses a `Cookie` header string into a collection of key-value pairs.
    ///
    /// This method takes a `Cookie` header string (typically from a `Cookie` request header)
    /// and parses it into a map of cookie names to their values.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The `Cookie` header string to parse.
    ///
    /// # Returns
    ///
    /// A `Cookies` collection (a hash map) containing all parsed cookie key-value pairs.
    pub fn parse<C>(cookie: C) -> Cookies
    where
        C: AsRef<str>,
    {
        let cookie_ref: &str = cookie.as_ref();
        let mut cookies: Cookies = hash_map_xx_hash3_64();
        if cookie_ref.trim().is_empty() {
            return cookies;
        }
        let parts: Vec<&str> = cookie_ref.split(SEMICOLON).collect();
        for part in parts {
            let part: &str = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((name, value)) = part.split_once(EQUAL) {
                let name: String = name.trim().to_string();
                let value: String = value.trim().to_string();
                if !name.is_empty() {
                    cookies.insert(name, value);
                }
            } else if !part.is_empty() {
                cookies.insert(part.to_string(), String::new());
            }
        }
        cookies
    }
}
