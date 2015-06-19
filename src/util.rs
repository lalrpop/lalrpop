use std::fmt::{Display, Formatter, Error};

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
