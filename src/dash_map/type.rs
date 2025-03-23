use crate::*;

pub type DashMapXxHash3_64<K, V> = DashMap<K, V, BuildHasherDefault<XxHash3_64>>;
