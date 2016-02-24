use std::collections::btree_map;
use std::default::Default;
use std::iter::FromIterator;

use super::map::{map, Map};
use super::set::Set;

pub struct Multimap<K, C: Collection> {
    map: Map<K, C>,
}

pub trait Collection: Default {
    type Item;

    fn push(&mut self, item: Self::Item);
}

impl<K: Ord, C: Collection> Multimap<K, C> {
    pub fn new() -> Multimap<K, C> {
        Multimap { map: map() }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn push(&mut self, key: K, value: C::Item) {
        self.map.entry(key).or_insert_with(|| C::default()).push(value);
    }

    pub fn into_iter(self) -> btree_map::IntoIter<K, C> {
        self.map.into_iter()
    }
}

impl<K: Ord, C: Collection> IntoIterator for Multimap<K, C> {
    type Item = (K, C);
    type IntoIter = btree_map::IntoIter<K, C>;
    fn into_iter(self) -> btree_map::IntoIter<K, C> {
        self.into_iter()
    }
}

impl<K: Ord, C: Collection> FromIterator<(K, C::Item)> for Multimap<K, C> {
    fn from_iter<T>(iterator: T) -> Self
        where T: IntoIterator<Item = (K, C::Item)>
    {
        let mut map = Multimap::new();
        for (key, value) in iterator {
            map.push(key, value);
        }
        map
    }
}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn push(&mut self, item: T) {
        self.push(item);
    }
}

impl<T: Ord> Collection for Set<T> {
    type Item = T;

    fn push(&mut self, item: T) {
        self.insert(item);
    }
}
