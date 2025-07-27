use crate::*;

impl ContentType {
    /// Serializes the provided data into a JSON string.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be serialized into JSON.
    ///
    /// # Returns
    ///
    /// A string containing the serialized JSON representation of the provided data.
    fn get_application_json<T>(data: &T) -> String
    where
        T: Serialize + Display,
    {
        json_to_string(data).unwrap_or_else(|_| "{}".to_string())
    }

    /// Serializes the provided data into an XML string.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be serialized into XML.
    ///
    /// # Returns
    ///
    /// A string containing the serialized XML representation of the provided data.
    fn get_application_xml<T>(data: &T) -> String
    where
        T: Serialize + Display,
    {
        serde_xml_rs::to_string(data).unwrap_or_else(|_| "<root></root>".to_string())
    }

    /// Formats the provided data into a plain text string.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be formatted into plain text.
    ///
    /// # Returns
    ///
    /// A plain text string representing the provided data.
    fn get_text_plain<T>(data: &T) -> String
    where
        T: Serialize + Debug + Clone + Default + Display,
    {
        data.to_string()
    }

    /// Formats the provided data into an HTML string.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be formatted into HTML.
    ///
    /// # Returns
    ///
    /// A string containing the HTML representation of the provided data.
    fn get_text_html<T>(data: &T) -> String
    where
        T: Serialize + Debug + Clone + Default,
    {
        let mut html: String = String::with_capacity(64);
        html.push_str("<table><tr><td>");
        html.push_str(&format!("{:?}", data));
        html.push_str("</td></tr></table>");
        html
    }

    /// Serializes the provided data into a URL-encoded string.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be serialized into URL-encoded format.
    ///
    /// # Returns
    ///
    /// A string containing the URL-encoded representation of the provided data.
    fn get_form_url_encoded<T>(data: &T) -> String
    where
        T: Serialize + Display,
    {
        serde_urlencoded::to_string(data).unwrap_or_else(|_| String::new())
    }

    /// Formats the provided data as a hexadecimal string.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be formatted into binary representation.
    ///
    /// # Returns
    ///
    /// A string containing the hexadecimal encoding of the provided data.
    fn get_binary<T>(data: &T) -> String
    where
        T: Serialize + Debug + Clone + Default + Display,
    {
        hex::encode(data.to_string())
    }

    /// Gets a formatted body string based on the `ContentType`.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be formatted into the body string.
    ///
    /// # Returns
    ///
    /// A string containing the formatted body based on the content type.
    pub fn get_body_string<T>(&self, data: &T) -> String
    where
        T: Serialize + Debug + Clone + Default + Display,
    {
        match self {
            Self::ApplicationJson => Self::get_application_json(data),
            Self::ApplicationXml => Self::get_application_xml(data),
            Self::TextPlain => Self::get_text_plain(data),
            Self::TextHtml => Self::get_text_html(data),
            Self::FormUrlEncoded => Self::get_form_url_encoded(data),
            Self::Unknown => Self::get_binary(data),
        }
    }

    /// Formats a content type with a charset value.
    ///
    /// # Arguments
    ///
    /// * `content_type` - The content type.
    /// * `charset` - The character set.
    ///
    /// # Returns
    ///
    /// A formatted string.
    pub fn format_content_type_with_charset(content_type: &str, charset: &str) -> String {
        let mut result: String = String::with_capacity(
            content_type.len() + SEMICOLON_SPACE.len() + CHARSET_EQUAL.len() + charset.len(),
        );
        result.push_str(content_type);
        result.push_str(SEMICOLON_SPACE);
        result.push_str(CHARSET_EQUAL);
        result.push_str(charset);
        result
    }

    /// Formats a content type with a full charset declaration.
    ///
    /// # Arguments
    ///
    /// * `content_type` - The content type.
    /// * `charset_with_key` - The charset declaration.
    ///
    /// # Returns
    ///
    /// A formatted string.
    pub fn format_content_type_with_charset_declaration(
        content_type: &str,
        charset_with_key: &str,
    ) -> String {
        let mut result: String = String::with_capacity(
            content_type.len() + SEMICOLON_SPACE.len() + charset_with_key.len(),
        );
        result.push_str(content_type);
        result.push_str(SEMICOLON_SPACE);
        result.push_str(charset_with_key);
        result
    }
}

impl FromStr for ContentType {
    type Err = ();

    /// Parses a string slice into a `ContentType`.
    ///
    /// # Arguments
    ///
    /// * `data` - The string slice to parse.
    ///
    /// # Returns
    ///
    /// A `Result` indicating either the parsed `ContentType` or an empty error.
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_ascii_lowercase().as_str() {
            APPLICATION_JSON => Ok(Self::ApplicationJson),
            APPLICATION_XML => Ok(Self::ApplicationXml),
            TEXT_PLAIN => Ok(Self::TextPlain),
            TEXT_HTML => Ok(Self::TextHtml),
            FORM_URLENCODED => Ok(Self::FormUrlEncoded),
            _ => Ok(Self::Unknown),
        }
    }
}

impl Default for ContentType {
    /// Returns the default `ContentType`.
    ///
    /// # Returns
    ///
    /// The `Unknown` variant of `ContentType`.
    fn default() -> Self {
        Self::Unknown
    }
}
