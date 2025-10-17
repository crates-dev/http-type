use crate::*;

/// Creates a new `RcRwLock` instance.
///
/// This function wraps a given data of type T in an `Rc` and `RwLock` to provide shared mutable access.
///
/// # Arguments
///
/// - `data` - The data of type T to be wrapped.
///
/// # Returns
///
/// A new `RcRwLock<T>` instance.
#[inline]
pub fn rc_rwlock<T>(data: T) -> RcRwLock<T> {
    Rc::new(RwLock::new(data))
}
