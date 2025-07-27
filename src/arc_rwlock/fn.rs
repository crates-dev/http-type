use crate::*;

/// Creates a new `ArcRwLock` from the given data.
///
/// # Arguments
///
/// * `data` - The data to be wrapped in an `Arc<RwLock>`.
///
/// # Returns
///
/// An `ArcRwLock` containing the provided data.
pub fn arc_rwlock<T>(data: T) -> ArcRwLock<T> {
    Arc::new(RwLock::new(data))
}
