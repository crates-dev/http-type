use crate::*;

pub fn dash_map<K: Eq + Hash, V>() -> DashMapXxHash3_64<K, V> {
    DashMap::with_hasher(BuildHasherDefault::default())
}
