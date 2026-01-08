use crate::*;

/// Implementation for `CookieBuilder`.
impl<'a> CookieBuilder<'a> {
    /// Parses a `Set-Cookie` header string into a `CookieBuilder`.
    ///
    /// This method takes a `Set-Cookie` header string and extracts the various
    /// attributes of a cookie, populating a `CookieBuilder` instance.
    ///
    /// # Arguments
    ///
    /// - `&'a str` - The `Set-Cookie` header string to parse.
    ///
    /// # Returns
    ///
    /// A `CookieBuilder<'a>` instance populated with the parsed cookie attributes.
    pub fn parse(cookie: &'a str) -> Self {
        let parts: Vec<&str> = cookie.split(SEMICOLON).collect();
        if parts.is_empty() {
            return Self::default();
        }
        let mut name: &str = "";
        let mut value: &str = "";
        if let Some(name_value_pair) = parts.first() {
            let name_value_pair: &str = name_value_pair.trim();
            if let Some((n, v)) = name_value_pair.split_once(EQUAL) {
                name = n.trim();
                value = v.trim();
            } else if !name_value_pair.is_empty() {
                name = name_value_pair;
                value = "";
            }
        }
        let mut expires: &str = "";
        let mut max_age: i64 = 0i64;
        let mut domain: &str = "";
        let mut path: &str = "";
        let mut secure: bool = false;
        let mut http_only: bool = false;
        let mut same_site: &str = "";
        for part in parts.iter().skip(1) {
            let part: &str = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((key, val)) = part.split_once(EQUAL) {
                let key_lowercase: String = key.trim().to_lowercase();
                let val: &str = val.trim();
                match key_lowercase.as_str() {
                    COOKIE_EXPIRES_LOWERCASE => {
                        expires = val;
                    }
                    COOKIE_MAX_AGE_LOWERCASE => {
                        if let Ok(max_age_value) = val.parse::<i64>() {
                            max_age = max_age_value;
                        }
                    }
                    COOKIE_DOMAIN_LOWERCASE => {
                        domain = val;
                    }
                    COOKIE_PATH_LOWERCASE => {
                        path = val;
                    }
                    COOKIE_SAME_SITE_LOWERCASE => {
                        same_site = val;
                    }
                    _ => {}
                }
            } else {
                let attribute_lowercase: String = part.to_lowercase();
                match attribute_lowercase.as_str() {
                    COOKIE_SECURE_LOWERCASE => {
                        secure = true;
                    }
                    COOKIE_HTTP_ONLY_LOWERCASE => {
                        http_only = true;
                    }
                    _ => {}
                }
            }
        }

        Self {
            name,
            value,
            expires,
            max_age,
            domain,
            path,
            secure,
            http_only,
            same_site,
        }
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
            cookie_string.push_str(self.expires);
        }
        if self.max_age > 0 {
            cookie_string.push_str(COOKIE_MAX_AGE_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(&self.max_age.to_string());
        }
        if !self.domain.is_empty() {
            cookie_string.push_str(COOKIE_DOMAIN_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(self.domain);
        }
        if !self.path.is_empty() {
            cookie_string.push_str(COOKIE_PATH_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(self.path);
        }
        if self.secure {
            cookie_string.push_str(COOKIE_SECURE_ATTRIBUTE_LOWERCASE);
        }
        if self.http_only {
            cookie_string.push_str(COOKIE_HTTP_ONLY_ATTRIBUTE_LOWERCASE);
        }
        if !self.same_site.is_empty() {
            cookie_string.push_str(COOKIE_SAME_SITE_ATTRIBUTE_LOWERCASE);
            cookie_string.push_str(self.same_site);
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
    /// - `&str` - The `Cookie` header string to parse.
    ///
    /// # Returns
    ///
    /// A `Cookies` collection (a hash map) containing all parsed cookie key-value pairs.
    pub fn parse(cookie: &str) -> Cookies<'_> {
        let mut cookies: Cookies<'_> = hash_map_xx_hash3_64();
        if cookie.trim().is_empty() {
            return cookies;
        }
        let parts: Vec<&str> = cookie.split(SEMICOLON).collect();
        for part in parts {
            let part: &str = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((name, value)) = part.split_once(EQUAL) {
                let name: &str = name.trim();
                let value: &str = value.trim();
                if !name.is_empty() {
                    cookies.insert(name, value);
                }
            } else if !part.is_empty() {
                cookies.insert(part, EMPTY_STR);
            }
        }
        cookies
    }
}
