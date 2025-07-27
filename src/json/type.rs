use crate::*;

/// A type alias for `Result` with `JsonError` as the error type.
pub type ResultJsonError<T> = Result<T, JsonError>;
