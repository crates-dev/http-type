use crate::*;

/// Combines a content type and a charset value into a single formatted string.
///
/// - `content_type`: The content type (e.g., `"text/html"`).
/// - `charset`: The character set (e.g., `"utf-8"`).
/// - Returns: A format string.
#[inline]
pub fn content_type_charset(content_type: &str, charset: &str) -> String {
    format!(
        "{}{}{}{}",
        content_type, SEMICOLON_SPACE, CHARSET_EQUAL, charset
    )
}

/// Combines a content type and a charset into a single formatted string.
///
/// - `content_type`: The content type (e.g., `"text/html"`).
/// - `charset`: The character set (e.g., `"charset=utf-8"`).
/// - Returns: A format string.
#[inline]
pub fn content_type_charset_with_key(content_type: &str, charset_with_key: &str) -> String {
    format!("{}{}{}", content_type, SEMICOLON_SPACE, charset_with_key)
}
