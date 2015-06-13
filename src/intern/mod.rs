use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Error, Formatter};

#[cfg(test)]
mod test;

thread_local! {
    static INTERNER_TLS: RefCell<Interner> =
        RefCell::new(Interner::new())
}

struct Interner {
    map: HashMap<String, InternedString>,
    strings: Vec<String>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct InternedString {
    index: u32
}

pub fn intern(s: &str) -> InternedString {
    INTERNER_TLS.with(|interner| {
        let mut interner = interner.borrow_mut();

        match interner.map.get(s) {
            Some(&v) => { return v; }
            None => { }
        }

        let index = interner.strings.len() as u32;
        let result = InternedString { index: index };
        interner.map.insert(s.to_string(), result);
        interner.strings.push(s.to_string());
        return result;
    })
}

impl Interner {
    fn new() -> Interner {
        Interner { map: HashMap::new(), strings: vec![] }
    }

    fn data(&self, i: InternedString) -> &str {
        &self.strings[i.index()]
    }
}

impl InternedString {
    fn index(&self) -> usize {
        self.index as usize
    }

    pub fn to_string(&self) -> String {
        INTERNER_TLS.with(
            |interner| interner.borrow().data(*self).to_string())
    }
}

impl Debug for InternedString {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        INTERNER_TLS.with(
            |interner| Debug::fmt(&interner.borrow().data(*self), fmt))
    }
}

impl Display for InternedString {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        INTERNER_TLS.with(
            |interner| Display::fmt(&interner.borrow().strings[self.index()], fmt))
    }
}
