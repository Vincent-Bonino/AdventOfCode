use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default)]
pub struct CollectionHashMap<K, T>
where
    K: Eq + Hash,
{
    pub hash_map: HashMap<K, Vec<T>>,
}

impl<K: Eq + Hash, T> CollectionHashMap<K, T> {
    pub fn new() -> Self {
        CollectionHashMap {
            hash_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: T) {
        self.hash_map.entry(key).or_default().push(value)
    }
}
