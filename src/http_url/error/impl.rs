use super::*;
use crate::*;

/// Implements the `StdError` trait for `HttpUrlError`.
impl StdError for HttpUrlError {}

/// Implements the `Display` trait for `HttpUrlError`, allowing it to be formatted as a string.
impl Display for HttpUrlError {
    /// Formats the `HttpUrlError` variant into a human-readable string.
    ///
    /// # Arguments
    ///
    /// - `f` - The formatter to write the string into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpUrlError::InvalidUrl => write!(f, "Invalid URL"),
        }
    }
}
