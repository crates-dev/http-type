use crate::*;

pub fn hash_set_xx_hash3_64<K: Eq + Hash>() -> HashSetXxHash3_64<K> {
    HashSet::with_hasher(BuildHasherDefault::default())
}
