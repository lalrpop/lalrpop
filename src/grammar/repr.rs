/*!
 * Compiled representation of a grammar. Simplified, normalized
 * version of `parse_tree`. The normalization passes produce this
 * representation incrementally.
 */

use std::cmp::Ord;
use intern::InternedString;
use grammar::parse_tree::Span;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Error};
use util::Sep;

#[derive(Clone, Debug)]
pub struct Grammar {
    pub action_fn_defns: Vec<ActionFnDefn>,
    pub productions: Vec<Production>,
    pub conversions: HashMap<InternedString, InternedString>,
    pub types: Types,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Production {
    pub span: Span,
    pub nonterminal: InternedString,
    pub symbols: Vec<Symbol>,
    pub action_fn: ActionFn,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Symbol {
    Nonterminal(InternedString),
    Terminal(InternedString),
}

#[derive(Clone, PartialEq, Eq)]
pub struct ActionFnDefn {
    pub arg_patterns: Vec<InternedString>,
    pub arg_types: Vec<TypeRepr>,
    pub ret_type: TypeRepr,
    pub code: String,
}

#[derive(Clone, PartialEq, Eq)]
pub enum TypeRepr {
    Tuple(Vec<TypeRepr>),
    Nominal { path: Vec<InternedString>, types: Vec<TypeRepr> },
    Lifetime(InternedString),
}

#[derive(Clone)]
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

#[derive(Debug)]
struct DummyTypes<'a> {
    terminal_type: &'a TypeRepr,
    nonterminal_types: Vec<(&'a InternedString, &'a TypeRepr)>,
}

impl Debug for Types {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let mut nonterminal_types: Vec<_> = self.nonterminal_types.iter()
                                                                  .collect();
        nonterminal_types.sort_by(|k1, k2| Ord::cmp(&k1.0, &k2.0));
        let dummy = DummyTypes { terminal_type:  &self.terminal_type,
                                 nonterminal_types: nonterminal_types };
        Debug::fmt(&dummy, fmt)
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

impl Debug for TypeRepr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
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
            Symbol::Terminal(_) => t.terminal_type(),
            Symbol::Nonterminal(id) => t.nonterminal_type(id),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Symbol::Nonterminal(id) => write!(fmt, "{}", id),
            Symbol::Terminal(id) => write!(fmt, "\"{}\"", id),
        }
    }
}

impl Debug for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

impl Debug for Production {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt,
               "{} = {} => {:?};",
               self.nonterminal, Sep(", ", &self.symbols), self.action_fn)
    }
}

impl Debug for ActionFnDefn {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.to_fn_string("_"))
    }
}

impl ActionFnDefn {
    fn to_fn_string(&self, name: &str) -> String {
        let arg_strings: Vec<String> =
               self.arg_patterns
                   .iter()
                   .zip(self.arg_types.iter())
                   .map(|(p, t)| format!("{}: {}", p, t))
                   .collect();

        format!("fn {}({}) -> {} {{ {} }}",
                name, Sep(", ", &arg_strings), self.ret_type, self.code)
    }
}
