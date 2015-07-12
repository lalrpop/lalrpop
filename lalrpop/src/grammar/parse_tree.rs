/*!

The "parse-tree" is what is produced by the parser. We use it do
some pre-expansion and so forth before creating the proper AST.

*/

use intern::{InternedString};
use grammar::repr::{NominalTypeRepr, TypeRepr};
use grammar::pattern::Pattern;
use std::fmt::{Debug, Display, Formatter, Error};
use util::Sep;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grammar {
    pub span: Span,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub where_clauses: Vec<String>,
    pub items: Vec<GrammarItem>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span(pub usize, pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarItem {
    ExternToken(ExternToken),
    Nonterminal(NonterminalData),
    Use(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternToken {
    pub enum_token: EnumToken,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumToken {
    pub type_name: TypeRef,
    pub type_span: Span,
    pub conversions: Vec<Conversion>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conversion {
    pub span: Span,
    pub from: TerminalString,
    pub to: Pattern<TypeRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRef {
    // (T1, T2)
    Tuple(Vec<TypeRef>),

    // Foo<'a, 'b, T1, T2>, Foo::Bar, etc
    Nominal {
        path: Vec<InternedString>,
        types: Vec<TypeRef>
    },

    // 'x ==> only should appear within nominal types, but what do we care
    Lifetime(InternedString),

    // Foo or Bar ==> treated specially since macros may care
    Id(InternedString),

    // <N> ==> type of a nonterminal, emitted by macro expansion
    OfSymbol(SymbolKind),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeParameter {
    Lifetime(InternedString),
    Id(InternedString),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: InternedString,
    pub ty: TypeRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonterminalData {
    // a "public" nonterminal is one that we will use as a start symbol
    pub public: bool,
    pub name: NonterminalString,
    pub span: Span,
    pub args: Vec<NonterminalString>, // macro arguments
    pub type_decl: Option<TypeRef>,
    pub alternatives: Vec<Alternative>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alternative {
    pub span: Span,

    pub expr: ExprSymbol,

    // if C, only legal in macros
    pub condition: Option<Condition>,

    // => { code }
    pub action: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Condition {
    pub span: Span,
    pub lhs: NonterminalString, // X
    pub rhs: InternedString, // "Foo"
    pub op: ConditionOp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConditionOp {
    // X == "Foo", equality
    Equals,

    // X != "Foo", inequality
    NotEquals,

    // X ~~ "Foo", regexp match
    Match,

    // X !~ "Foo", regexp non-match
    NotMatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub span: Span,
    pub kind: SymbolKind
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    // (X Y)
    Expr(ExprSymbol),

    // "foo"
    Terminal(TerminalString),

    // foo
    Nonterminal(NonterminalString),

    // foo<..>
    Macro(MacroSymbol),

    // X+, X?, X*
    Repeat(Box<RepeatSymbol>),

    // <X>
    Choose(Box<Symbol>),

    // x:X
    Name(InternedString, Box<Symbol>),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TerminalString(pub InternedString);

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonterminalString(pub InternedString);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RepeatOp {
    Star, Plus, Question
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepeatSymbol {
    pub op: RepeatOp,
    pub symbol: Symbol
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprSymbol {
    pub symbols: Vec<Symbol>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MacroSymbol {
    pub name: NonterminalString,
    pub args: Vec<Symbol>,
}

impl GrammarItem {
    pub fn is_macro_def(&self) -> bool {
        match *self {
            GrammarItem::Nonterminal(ref d) => d.is_macro_def(),
            _ => false,
        }
    }

    pub fn as_nonterminal(&self) -> Option<&NonterminalData> {
        match *self {
            GrammarItem::Nonterminal(ref d) => Some(d),
            GrammarItem::Use(..) => None,
            GrammarItem::ExternToken(..) => None,
        }
    }

    pub fn as_extern_token(&self) -> Option<&ExternToken> {
        match *self {
            GrammarItem::Nonterminal(..) => None,
            GrammarItem::Use(..) => None,
            GrammarItem::ExternToken(ref d) => Some(d),
        }
    }
}

impl NonterminalData {
    pub fn is_macro_def(&self) -> bool {
        !self.args.is_empty()
    }
}

impl Symbol {
    pub fn new(span: Span, kind: SymbolKind) -> Symbol {
        Symbol { span: span, kind: kind }
    }

    pub fn canonical_form(&self) -> String {
        format!("{}", self)
    }
}

impl Display for TerminalString {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "\"{}\"", self.0)
    }
}

impl Debug for TerminalString {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

impl Display for NonterminalString {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.0)
    }
}

impl Debug for NonterminalString {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(&self.kind, fmt)
    }
}

impl Display for SymbolKind {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            SymbolKind::Expr(ref expr) =>
                write!(fmt, "{}", expr),
            SymbolKind::Terminal(ref s) =>
                write!(fmt, "{}", s),
            SymbolKind::Nonterminal(ref s) =>
                write!(fmt, "{}", s),
            SymbolKind::Macro(ref m) =>
                write!(fmt, "{}", m),
            SymbolKind::Repeat(ref r) =>
                write!(fmt, "{}", r),
            SymbolKind::Choose(ref s) =>
                write!(fmt, "<{}>", s),
            SymbolKind::Name(n, ref s) =>
                write!(fmt, "{}:{}", n, s),
        }
    }
}

impl Display for RepeatSymbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}{}", self.symbol, self.op)
    }
}

impl Display for RepeatOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            RepeatOp::Plus => write!(fmt, "+"),
            RepeatOp::Star => write!(fmt, "*"),
            RepeatOp::Question => write!(fmt, "?"),
        }
    }
}

impl Display for ExprSymbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "({})", Sep(" ", &self.symbols))
    }
}

impl ExprSymbol {
    pub fn canonical_form(&self) -> String {
        format!("{}", self)
    }
}

impl MacroSymbol {
    pub fn canonical_form(&self) -> String {
        format!("{}", self)
    }
}

impl RepeatSymbol {
    pub fn canonical_form(&self) -> String {
        format!("{}", self)
    }
}

impl Display for MacroSymbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}<{}>", self.name, Sep(", ", &self.args))
    }
}

impl Display for TypeRef {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            TypeRef::Tuple(ref types) =>
                write!(fmt, "({})", Sep(", ", types)),
            TypeRef::Nominal { ref path, ref types } if types.len() == 0 =>
                write!(fmt, "{}", Sep("::", path)),
            TypeRef::Nominal { ref path, ref types } =>
                write!(fmt, "{}<{}>", Sep("::", path), Sep(", ", types)),
            TypeRef::Lifetime(ref s) =>
                write!(fmt, "{}", s),
            TypeRef::Id(ref s) =>
                write!(fmt, "{}", s),
            TypeRef::OfSymbol(ref s) =>
                write!(fmt, "`{}`", s),
        }
    }
}

impl TypeRef {
    // Converts a TypeRef to a TypeRepr, assuming no inference is
    // required etc. This is safe for all types a user can directly
    // type, but not safe for the result of expanding macros.
    pub fn type_repr(&self) -> TypeRepr {
        match *self {
            TypeRef::Tuple(ref types) =>
                TypeRepr::Tuple(types.iter().map(TypeRef::type_repr).collect()),
            TypeRef::Nominal { ref path, ref types } =>
                TypeRepr::Nominal(NominalTypeRepr {
                    path: path.clone(),
                    types: types.iter().map(TypeRef::type_repr).collect()
                }),
            TypeRef::Lifetime(id) =>
                TypeRepr::Lifetime(id),
            TypeRef::Id(id) =>
                TypeRepr::Nominal(NominalTypeRepr {
                    path: vec![id],
                    types: vec![]
                }),
            TypeRef::OfSymbol(_) =>
                unreachable!("OfSymbol produced by parser")
        }
    }
}
