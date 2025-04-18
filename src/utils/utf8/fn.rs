use crate::*;

/// Checks if the given byte slice is valid UTF-8.
///
/// - `data`: The byte slice to validate.
/// - Returns: `true` if the byte slice is valid UTF-8, otherwise `false`.
pub fn is_valid_utf8(data: &[u8]) -> bool {
    std::str::from_utf8(data).is_ok()
}

/// Converts a byte slice to a UTF-8 string or provides a fallback message.
///
/// - `body`: The byte slice to convert.
/// - Returns: A `Cow<'_, str>` containing the UTF-8 string if valid,
///            otherwise an owned string indicating the binary data length.
pub fn body_to_string(body: &[u8]) -> Cow<'_, str> {
    match std::str::from_utf8(body) {
        Ok(string_data) => Cow::Borrowed(string_data),
        Err(_) => Cow::Owned(format!("binary data len: {}", body.len())),
    }
}
