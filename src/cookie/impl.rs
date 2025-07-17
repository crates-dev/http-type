use crate::*;

impl CookieBuilder {
    /// Creates a new CookieBuilder with the specified name and value.
    ///
    /// # Parameters
    /// - `name`: The name of the cookie.
    /// - `value`: The value of the cookie.
    ///
    /// # Returns
    /// - A new CookieBuilder instance.
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<CookieKey>,
        V: Into<CookieValue>,
    {
        Self {
            name: name.into(),
            value: value.into(),
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    /// Parses a Set-Cookie header string and returns a CookieBuilder instance.
    ///
    /// # Parameters
    /// - `cookie_string`: The Set-Cookie header string to parse.
    ///
    /// # Returns
    /// - A CookieBuilder instance with parsed attributes.
    pub fn parse(cookie_string: &str) -> Self {
        let mut cookie_builder: Self = Self::default();
        let parts: Vec<&str> = cookie_string.split(SEMICOLON).collect();
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
    /// # Parameters
    /// - `expires`: The expiration date string (e.g., "Wed, 21 Oct 2015 07:28:00 GMT").
    ///
    /// # Returns
    /// - The CookieBuilder instance for method chaining.
    pub fn expires<T>(&mut self, expires: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.expires = Some(expires.into());
        self
    }

    /// Sets the maximum age for the cookie in seconds.
    ///
    /// # Parameters
    /// - `max_age`: The maximum age in seconds.
    ///
    /// # Returns
    /// - The CookieBuilder instance for method chaining.
    pub fn max_age(&mut self, max_age: i64) -> &mut Self {
        self.max_age = Some(max_age);
        self
    }

    /// Sets the domain for the cookie.
    ///
    /// # Parameters
    /// - `domain`: The domain for the cookie.
    ///
    /// # Returns
    /// - The CookieBuilder instance for method chaining.
    pub fn domain<T>(&mut self, domain: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.domain = Some(domain.into());
        self
    }

    /// Sets the path for the cookie.
    ///
    /// # Parameters
    /// - `path`: The path for the cookie.
    ///
    /// # Returns
    /// - The CookieBuilder instance for method chaining.
    pub fn path<T>(&mut self, path: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.path = Some(path.into());
        self
    }

    /// Sets the secure flag for the cookie.
    ///
    /// When set to true, the cookie will only be sent over HTTPS connections.
    ///
    /// # Returns
    /// - The CookieBuilder instance for method chaining.
    pub fn secure(&mut self) -> &mut Self {
        self.secure = true;
        self
    }

    /// Sets the HttpOnly flag for the cookie.
    ///
    /// When set to true, the cookie will be inaccessible to JavaScript.
    ///
    /// # Returns
    /// - The CookieBuilder instance for method chaining.
    pub fn http_only(&mut self) -> &mut Self {
        self.http_only = true;
        self
    }

    /// Sets the SameSite policy for the cookie.
    ///
    /// # Parameters
    /// - `same_site`: The SameSite policy ("Strict", "Lax", or "None").
    ///
    /// # Returns
    /// - The CookieBuilder instance for method chaining.
    pub fn same_site<T>(&mut self, same_site: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.same_site = Some(same_site.into());
        self
    }

    /// Builds the cookie string representation.
    ///
    /// # Returns
    /// - A formatted cookie string ready to be used in Set-Cookie headers.
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

impl Cookie {
    /// Parses a Cookie header string and returns a Cookies collection.
    ///
    /// # Parameters
    /// - `cookie_string`: The Cookie header string to parse (e.g., "name1=value1; name2=value2").
    ///
    /// # Returns
    /// - A Cookies collection containing all parsed cookie key-value pairs.
    pub fn parse(cookie_string: &str) -> Cookies {
        let mut cookies: Cookies = hash_map_xx_hash3_64();
        if cookie_string.trim().is_empty() {
            return cookies;
        }
        let parts: Vec<&str> = cookie_string.split(SEMICOLON).collect();
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
