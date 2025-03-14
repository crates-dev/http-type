use crate::*;

#[inline]
pub fn arc_mutex<T>(data: T) -> ArcMutex<T> {
    Arc::new(Mutex::new(data))
}
