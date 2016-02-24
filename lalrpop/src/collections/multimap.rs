use std::collections::btree_map;
use std::iter::FromIterator;

use super::map::{map, Map};

pub struct Multimap<K, V> {
    map: Map<K, Vec<V>>,
}

impl<K: Ord, V> Multimap<K, V> {
    pub fn new() -> Multimap<K, V> {
        Multimap { map: map() }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn push(&mut self, key: K, value: V) {
        self.map.entry(key).or_insert(vec![]).push(value);
    }

    pub fn into_iter(self) -> btree_map::IntoIter<K, Vec<V>> {
        self.map.into_iter()
    }
}

impl<K: Ord, V> IntoIterator for Multimap<K, V> {
    type Item = (K, Vec<V>);
    type IntoIter = btree_map::IntoIter<K, Vec<V>>;
    fn into_iter(self) -> btree_map::IntoIter<K, Vec<V>> {
        self.into_iter()
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for Multimap<K, V> {
    fn from_iter<T>(iterator: T) -> Self
        where T: IntoIterator<Item = (K, V)>
    {
        let mut map = Multimap::new();
        for (key, value) in iterator {
            map.push(key, value);
        }
        map
    }
}
