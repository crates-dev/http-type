use crate::*;

pub type DashMapRandomState<K, V> = DashMap<K, V, BuildHasherDefault<XxHash3_64>>;
