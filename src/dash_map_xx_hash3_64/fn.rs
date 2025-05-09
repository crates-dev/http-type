use crate::*;

pub fn dash_map_xx_hash3_64<K: Eq + Hash, V>() -> DashMapXxHash3_64<K, V> {
    DashMap::with_hasher(BuildHasherDefault::default())
}
