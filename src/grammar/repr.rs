/*!
 * Compiled representation of a grammar. Simplified, normalized
 * version of `parse_tree`. The normalization passes produce this
 * representation incrementally.
 */

use intern::InternedString;
use grammar::parse_tree::Span;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use util::Sep;

#[derive(Clone, Debug)]
pub struct Grammar {
    pub action_fn_defns: Vec<ActionFnDefn>,
    pub productions: Vec<Production>,
    pub conversions: HashMap<InternedString, InternedString>,
    pub types: Types,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Production {
    pub span: Span,
    pub nonterminal: InternedString,
    pub symbols: Vec<Symbol>,
    pub action_fn: ActionFn,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    Nonterminal(InternedString),
    Terminal(InternedString),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionFnDefn {
    pub arg_patterns: Vec<InternedString>,
    pub arg_types: Vec<TypeRepr>,
    pub ret_type: TypeRepr,
    pub code: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRepr {
    Tuple(Vec<TypeRepr>),
    Nominal { path: Vec<InternedString>, types: Vec<TypeRepr> },
    Lifetime(InternedString),
}

#[derive(Clone, Debug)]
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

    pub fn lookup_nonterminal_type(&self, nt_id: InternedString) -> Option<&TypeRepr> {
        self.nonterminal_types.get(&nt_id)
    }

    pub fn nonterminal_type(&self, nt_id: InternedString) -> &TypeRepr {
        &self.nonterminal_types[&nt_id]
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
pub struct ActionFn(u32);

impl ActionFn {
    pub fn new(x: usize) -> ActionFn {
        ActionFn(x as u32)
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl Symbol {
    pub fn ty<'ty>(&self, t: &'ty Types) -> &'ty TypeRepr {
        match *self {
            Symbol::Nonterminal(id) => t.terminal_type(),
            Symbol::Terminal(id) => t.nonterminal_type(id),
        }
    }
}
