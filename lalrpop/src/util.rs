use std::collections::{btree_map, BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter, Error};
use std::iter::FromIterator;

pub use std::collections::btree_map as map;
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

impl<K:Ord,V> Multimap<K,V> {
    pub fn new() -> Multimap<K,V> {
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

impl<K:Ord,V> IntoIterator for Multimap<K,V> {
    type Item = (K, Vec<V>);
    type IntoIter = btree_map::IntoIter<K, Vec<V>>;
    fn into_iter(self) -> btree_map::IntoIter<K, Vec<V>> {
        self.into_iter()
    }
}

impl<K:Ord,V> FromIterator<(K,V)> for Multimap<K,V> {
    fn from_iter<T>(iterator: T) -> Self where T: IntoIterator<Item=(K,V)> {
        let mut map = Multimap::new();
        for (key, value) in iterator {
            map.push(key, value);
        }
        map
    }
}

/// In general, we avoid coding directly against any particular map,
/// but rather build against `util::Map` (and `util::map` to construct
/// an instance). This should be a deterministic map, such that two
/// runs of LALRPOP produce the same output, but otherwise it doesn't
/// matter much. I'd probably prefer to use `HashMap` with an
/// alternative hasher, but that's not stable.
pub type Map<K,V> = BTreeMap<K,V>;

pub fn map<K:Ord,V>() -> Map<K,V> {
    Map::<K,V>::default()
}

/// As `Map`, but for sets.
pub type Set<K> = BTreeSet<K>;

pub fn set<K:Ord>() -> Set<K> {
    Set::<K>::default()
}

/// Strip leading and trailing whitespace.
pub fn strip(s: &str) -> &str {
    s.trim_matches(char::is_whitespace)
}
