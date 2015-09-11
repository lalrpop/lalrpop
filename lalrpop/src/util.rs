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

pub struct Escape<S>(pub S);

impl<S:Display> Display for Escape<S> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let tmp = format!("{}", self.0);
        for c in tmp.chars() {
            match c {
                'a' ... 'z' | '0' ... '9' | 'A' ... 'Z' => try!(write!(fmt, "{}", c)),
                '_' => try!(write!(fmt, "__")),
                _ => try!(write!(fmt, "_{:x}", c as usize)),
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
    map: Map<K,Vec<V>>
}

impl<K:Hash+Eq,V> Multimap<K,V> {
    pub fn new() -> Multimap<K,V> {
        Multimap { map: map() }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn push(&mut self, key: K, value: V) {
        self.map.entry(key).or_insert(vec![]).push(value);
    }

    pub fn into_iter(self) -> hash_map::IntoIter<K, Vec<V>> {
        self.map.into_iter()
    }
}

impl<K:Hash+Eq,V> IntoIterator for Multimap<K,V> {
    type Item = (K, Vec<V>);
    type IntoIter = hash_map::IntoIter<K, Vec<V>>;
    fn into_iter(self) -> hash_map::IntoIter<K, Vec<V>> {
        self.into_iter()
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

pub fn map<K:Hash+Eq,V>() -> Map<K,V> {
    Map::<K,V>::default()
}

pub type Set<K> = HashSet<K>;

#[allow(dead_code)] // we don't happen to use this yet
pub fn set<K:Hash+Eq>() -> Set<K> {
    Set::<K>::default()
}

/// Strip leading and trailing whitespace.
pub fn strip(s: &str) -> &str {
    s.trim_matches(char::is_whitespace)
}
