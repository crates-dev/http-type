use crate::*;

/// Creates a new `BoxRwLock` from the given data.
///
/// # Arguments
///
/// - `data` - The data to be wrapped in a `Box<RwLock>`.
///
/// # Returns
///
/// A `BoxRwLock` containing the provided data.
pub fn box_rwlock<T>(data: T) -> BoxRwLock<T> {
    Box::new(RwLock::new(data))
}
