use crate::*;

/// Represents different types of errors that can occur in the application.
///
/// The `HttpUrlError` enum defines various error types related to HTTP requests, network connections, and TLS operations.
/// Each variant corresponds to a specific error that can occur during the execution of the application.
///
/// # Variants
/// - `InvalidUrl`: Indicates that the provided URL is invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpUrlError {
    InvalidUrl,
}
