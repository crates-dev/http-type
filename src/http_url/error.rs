use crate::*;

/// Represents different types of errors that can occur in the application.
///
/// The `Error` enum defines various error types related to HTTP requests, network connections, and TLS operations.
/// Each variant corresponds to a specific error that can occur during the execution of the application.
///
/// # Variants
/// - `InvalidUrl`: Indicates that the provided URL is invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidUrl,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
        }
    }
}
