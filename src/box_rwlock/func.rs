use crate::*;

pub fn box_rwlock<T>(data: T) -> BoxRwLock<T> {
    Box::new(RwLock::new(data))
}
