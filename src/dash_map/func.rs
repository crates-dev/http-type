use crate::*;

#[inline]
pub fn dash_map<K: Eq + Hash, V>() -> DashMap<K, V, RandomState> {
    let thread_count: usize = get_thread_count();
    DashMap::with_capacity_and_hasher(thread_count, RandomState::new())
}
