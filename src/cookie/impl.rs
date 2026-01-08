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
    #[inline(always)]
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        Self {
            name: name.as_ref().to_owned(),
            value: value.as_ref().to_owned(),
            ..Default::default()
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
                        cookie_builder.expires = value;
                    }
                    COOKIE_MAX_AGE_LOWERCASE => {
                        if let Ok(max_age_value) = value.parse::<i64>() {
                            cookie_builder.max_age = max_age_value;
                        }
                    }
                    COOKIE_DOMAIN_LOWERCASE => {
                        cookie_builder.domain = value;
                    }
                    COOKIE_PATH_LOWERCASE => {
                        cookie_builder.path = value;
                    }
                    COOKIE_SAME_SITE_LOWERCASE => {
                        cookie_builder.same_site = value;
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

    /// Builds the cookie string according to the `Set-Cookie` header format.
    ///
    /// Only includes attributes that have been set with non-empty values.
    ///
    /// # Returns
    ///
    /// - `String` - A formatted cookie string ready to be sent in a `Set-Cookie` header.
    pub fn build(&self) -> String {
        if self.name.is_empty() {
            return String::new();
        }
        let mut cookie_string: String = format!("{}={}", self.name, self.value);

        if !self.expires.is_empty() {
            cookie_string.push_str(COOKIE_EXPIRES_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(&self.expires);
        }
        if self.max_age > 0 {
            cookie_string.push_str(COOKIE_MAX_AGE_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(&self.max_age.to_string());
        }
        if !self.domain.is_empty() {
            cookie_string.push_str(COOKIE_DOMAIN_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(&self.domain);
        }
        if !self.path.is_empty() {
            cookie_string.push_str(COOKIE_PATH_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(&self.path);
        }
        if self.secure {
            cookie_string.push_str(COOKIE_SECURE_ATTRIBUTE_LOWERCASE);
        }
        if self.http_only {
            cookie_string.push_str(COOKIE_HTTP_ONLY_ATTRIBUTE_LOWERCASE);
        }
        if !self.same_site.is_empty() {
            cookie_string.push_str(COOKIE_SAME_SITE_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(&self.same_site);
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
