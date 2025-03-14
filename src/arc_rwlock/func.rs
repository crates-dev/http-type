use crate::*;

#[inline]
pub fn arc_rwlock<T>(data: T) -> ArcRwLock<T> {
    Arc::new(RwLock::new(data))
}
