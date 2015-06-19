use std::collections::{hash_map, HashMap, HashSet};
use std::fmt::{Display, Formatter, Error};
use std::hash::Hash;
use std::iter::FromIterator;

pub struct Sep<S>(pub &'static str, pub S);

impl<'a,S:Display> Display for Sep<&'a Vec<S>> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let &Sep(sep, vec) = self;
        let mut elems = vec.iter();
        if let Some(elem) = elems.next() {
            try!(write!(fmt, "{}", elem));
            while let Some(elem) = elems.next() {
                try!(write!(fmt, "{}{}", sep, elem));
            }
        }
        Ok(())
    }
}

pub struct Prefix<S>(pub &'static str, pub S);

impl<'a,S:Display> Display for Prefix<&'a [S]> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let &Prefix(prefix, vec) = self;
        let mut elems = vec.iter();
        while let Some(elem) = elems.next() {
            try!(write!(fmt, "{}{}", prefix, elem));
        }
        Ok(())
    }
}

pub struct Multimap<K,V> {
    map: HashMap<K,Vec<V>>
}

impl<K:Hash+Eq,V> Multimap<K,V> {
    pub fn new() -> Multimap<K,V> {
        Multimap { map: map() }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.map.entry(key).or_insert(vec![]).push(value);
    }

    pub fn get(&self, key: &K) -> &[V] {
        match self.map.get(key) {
            Some(v) => v,
            None => &[]
        }
    }

    pub fn into_iter(self) -> hash_map::IntoIter<K, Vec<V>> {
        self.map.into_iter()
    }
}

impl<K:Hash+Eq,V> FromIterator<(K,V)> for Multimap<K,V> {
    fn from_iter<T>(iterator: T) -> Self where T: IntoIterator<Item=(K,V)> {
        let mut map = Multimap::new();
        for (key, value) in iterator {
            map.push(key, value);
        }
        map
    }
}

pub type Map<K,V> = HashMap<K,V>;

pub fn map<K:Hash+Eq,V>() -> HashMap<K,V> {
    HashMap::new()
}

pub type Set<K> = HashSet<K>;

pub fn set<K:Hash+Eq>() -> HashSet<K> {
    HashSet::new()
}
