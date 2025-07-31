use crate::*;

/// Creates a new `ArcMutex` from the given data.
///
/// # Arguments
///
/// - `T` - The data to be wrapped in an `Arc<Mutex>`.
///
/// # Returns
///
/// An `ArcMutex<T>` containing the provided data.
pub fn arc_mutex<T>(data: T) -> ArcMutex<T> {
    Arc::new(Mutex::new(data))
}
