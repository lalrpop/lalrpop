/*!
 * Compiled representation of a grammar. Simplified, normalized
 * version of `parse_tree`. The normalization passes produce this
 * representation incrementally.
 */

use intern::InternedString;
use std::fmt::{Debug, Display, Formatter, Error};
use util::{map, Map, Sep};

// These concepts we re-use wholesale
pub use grammar::parse_tree::{NonterminalString, Span, TerminalString};

#[derive(Clone, Debug)]
pub struct Grammar {
    // a unique prefix that can be appended to identifiers to ensure
    // that they do not conflict with any action strings
    pub prefix: String,
    pub action_fn_defns: Vec<ActionFnDefn>,
    pub productions: Map<NonterminalString, Vec<Production>>,
    pub conversions: Map<TerminalString, TerminalString>,
    pub types: Types,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Production {
    // this overlaps with the key in the hashmap, obviously, but it's
    // handy to have it
    pub nonterminal: NonterminalString,
    pub symbols: Vec<Symbol>,
    pub action_fn: ActionFn,
    pub span: Span,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    Nonterminal(NonterminalString),
    Terminal(TerminalString),
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

#[derive(Clone, Debug)]
pub struct Types {
    terminal_type: TypeRepr,
    nonterminal_types: Map<NonterminalString, TypeRepr>
}

impl Types {
    pub fn new(terminal_type: TypeRepr) -> Types {
        Types { terminal_type: terminal_type,
                nonterminal_types: map() }
    }

    pub fn add_type(&mut self, nt_id: NonterminalString, ty: TypeRepr) {
        assert!(self.nonterminal_types.insert(nt_id, ty).is_none());
    }

    pub fn terminal_type(&self) -> &TypeRepr {
        &self.terminal_type
    }

    pub fn lookup_nonterminal_type(&self, id: NonterminalString) -> Option<&TypeRepr> {
        self.nonterminal_types.get(&id)
    }

    pub fn nonterminal_type(&self, id: NonterminalString) -> &TypeRepr {
        &self.nonterminal_types[&id]
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

#[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
            Symbol::Terminal(id) => write!(fmt, "{}", id),
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

impl Grammar {
    pub fn new(prefix: String,
               action_fn_defns: Vec<ActionFnDefn>,
               flat_productions: Vec<Production>,
               conversions: Vec<(TerminalString, TerminalString)>,
               types: Types)
               -> Grammar
    {
        let mut productions = map();

        for production in flat_productions {
            let mut vec = productions.entry(production.nonterminal).or_insert(vec![]);
            vec.push(production);
        }

        Grammar {
            prefix: prefix,
            action_fn_defns: action_fn_defns,
            productions: productions,
            conversions: conversions.into_iter().collect(),
            types: types,
        }
    }

    pub fn productions_for(&self, nonterminal: NonterminalString) -> &[Production] {
        match self.productions.get(&nonterminal) {
            Some(v) => &v[..],
            None => &[], // this...probably shouldn't happen actually?
        }
    }
}

