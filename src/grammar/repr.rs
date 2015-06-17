/*!
 * Compiled representation of a grammar. Simplified, normalized
 * version of `parse_tree`. The normalization passes produce this
 * representation incrementally.
 */

use intern::InternedString;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use util::Sep;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionFn {
    arg_names: Vec<InternedString>,
    arg_types: Vec<TypeRepr>,
    ret_type: Vec<TypeRepr>,
    code: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRepr {
    Tuple(Vec<TypeRepr>),
    Nominal { path: Vec<InternedString>, types: Vec<TypeRepr> },
    Lifetime(InternedString),
}

#[derive(Debug)]
pub struct Types {
    terminal_type: TypeRepr,
    nonterminal_types: HashMap<InternedString, TypeRepr>
}

impl Types {
    pub fn new(terminal_type: TypeRepr) -> Types {
        Types { terminal_type: terminal_type,
                nonterminal_types: HashMap::new() }
    }

    pub fn add_type(&mut self, nt_id: InternedString, ty: TypeRepr) {
        assert!(self.nonterminal_types.insert(nt_id, ty).is_none());
    }

    pub fn terminal_type(&self) -> &TypeRepr {
        &self.terminal_type
    }

    pub fn nt_type(&self, nt_id: InternedString) -> Option<&TypeRepr> {
        self.nonterminal_types.get(&nt_id)
    }
}

impl Display for TypeRepr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            TypeRepr::Tuple(ref types) =>
                write!(fmt, "({})", Sep(", ", types)),
            TypeRepr::Nominal { ref path, ref types } if types.len() == 0 =>
                write!(fmt, "{}", Sep("::", path)),
            TypeRepr::Nominal { ref path, ref types } =>
                write!(fmt, "{}<{}>", Sep("::", path), Sep(", ", types)),
            TypeRepr::Lifetime(id) =>
                write!(fmt, "{}", id),
        }
    }
}

