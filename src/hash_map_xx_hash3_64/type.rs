use crate::*;

/// A type alias for `HashMap` using `XxHash3_64` as the hasher.
pub type HashMapXxHash3_64<K, V> = HashMap<K, V, BuildHasherDefault<XxHash3_64>>;
