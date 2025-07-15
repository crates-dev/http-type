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
            cookie_string.push_str("; Expires=");
            cookie_string.push_str(expires_value);
        }
        if let Some(max_age_value) = self.max_age {
            cookie_string.push_str("; Max-Age=");
            cookie_string.push_str(&max_age_value.to_string());
        }
        if let Some(ref domain_value) = self.domain {
            cookie_string.push_str("; Domain=");
            cookie_string.push_str(domain_value);
        }
        if let Some(ref path_value) = self.path {
            cookie_string.push_str("; Path=");
            cookie_string.push_str(path_value);
        }
        if self.secure {
            cookie_string.push_str("; Secure");
        }
        if self.http_only {
            cookie_string.push_str("; HttpOnly");
        }
        if let Some(ref same_site_value) = self.same_site {
            cookie_string.push_str("; SameSite=");
            cookie_string.push_str(same_site_value);
        }
        cookie_string
    }
}
