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
pub struct Grammar {
    pub token: TokenData,
    pub action_fns: Vec<ActionFn>,
    pub productions: Vec<Production>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenData {
    pub token_type: TypeRepr,
    pub conversions: HashMap<InternedString, InternedString>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Production {
    pub nonterminal: InternedString,
    pub symbols: Vec<Symbol>,
    pub action_fn: ActionFnIndex,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    Nonterminal(InternedString),
    Terminal(InternedString),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionFn {
    pub arg_names: Vec<InternedString>,
    pub arg_types: Vec<TypeRepr>,
    pub ret_type: Vec<TypeRepr>,
    pub code: String,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActionFnIndex(u32);

impl ActionFnIndex {
    pub fn new(x: usize) -> ActionFnIndex {
        ActionFnIndex(x as u32)
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

