/*!

The "parse-tree" is what is produced by the parser. We use it do
some pre-expansion and so forth before creating the proper AST.

*/

use intern::{intern, InternedString};
use lexer::dfa::DFA;
use grammar::consts::{LALR, RECURSIVE_ASCENT, TABLE_DRIVEN, TEST_ALL};
use grammar::repr::{self as r, NominalTypeRepr, TypeRepr};
use grammar::pattern::Pattern;
use message::Content;
use message::builder::InlineBuilder;
use std::fmt::{Debug, Display, Formatter, Error};
use tls::Tls;
use util::Sep;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grammar {
    // see field `prefix` in `grammar::repr::Grammar`
    pub prefix: String,
    pub span: Span,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub where_clauses: Vec<String>,
    pub items: Vec<GrammarItem>,
    pub annotations: Vec<Annotation>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span(pub usize, pub usize);

impl Into<Box<Content>> for Span {
    fn into(self) -> Box<Content> {
        let file_text = Tls::file_text();
        let string = file_text.span_str(self);

        // Insert an Adjacent block to prevent wrapping inside this
        // string:
        InlineBuilder::new().begin_adjacent().text(string).end().end()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarItem {
    MatchToken(MatchToken),
    ExternToken(ExternToken),
    InternToken(InternToken),
    Nonterminal(NonterminalData),
    Use(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchToken {
    pub contents: Vec<MatchContents>,
    pub span: Span,
}

impl MatchToken {
    pub fn new(contents: MatchContents, span: Span) -> MatchToken {
        MatchToken {
            contents: vec![contents],
            span: span
        }
    }

    // Not really sure if this is the best way to do it
    pub fn add(self, contents: MatchContents) -> MatchToken {
        let mut new_contents = self.contents.clone();
        new_contents.push(contents);
        MatchToken {
            contents: new_contents,
            span: self.span
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchContents {
    pub items: Vec<MatchItem>
}

// FIXME: Validate that MatchSymbol is actually a TerminalString::Literal
//          and that MatchMapping is an Id or String
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MatchItem {
    CatchAll(Span),
    Unmapped(MatchSymbol, Span),
    Mapped(MatchSymbol, MatchMapping, Span)
}

impl MatchItem {
    pub fn is_catch_all(&self) -> bool {
        match *self {
            MatchItem::CatchAll(_) => true,
            _ => false
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            MatchItem::CatchAll(span)     => span,
            MatchItem::Unmapped(_, span)  => span,
            MatchItem::Mapped(_, _, span) => span
        }
    }
}

pub type MatchSymbol = TerminalLiteral;
pub type MatchMapping = TerminalString;

/// Intern tokens are not typed by the user: they are synthesized in
/// the absence of an "extern" declaration with information about the
/// string literals etc that appear in the grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InternToken {
    /// Set of `r"foo"` and `"foo"` literals extracted from the
    /// grammar. Sorted by order of increasing precedence.
    pub match_entries: Vec<MatchEntry>,
    pub dfa: DFA
}

/// In `token_check`, as we prepare to generate a tokenizer, we
/// combine any `match` declaration the user may have given with the
/// set of literals (e.g. `"foo"` or `r"[a-z]"`) that appear elsewhere
/// in their in the grammar to produce a series of `MatchEntry`. Each
/// `MatchEntry` roughly corresponds to one line in a `match` declaration.
///
/// So e.g. if you had
///
/// ```
/// match {
///    r"(?i)BEGIN" => "BEGIN",
///    "+" => "+",
/// } else {
///    _
/// }
///
/// ID = r"[a-zA-Z]+"
/// ```
///
/// This would correspond to three match entries:
/// - `MatchEntry { match_literal: r"(?i)BEGIN", user_name: "BEGIN", precedence: 2 }`
/// - `MatchEntry { match_literal: "+", user_name: "+", precedence: 3 }`
/// - `MatchEntry { match_literal: "r[a-zA-Z]+"", user_name: r"[a-zA-Z]+", precedence: 0 }`
///
/// A couple of things to note:
///
/// - Literals appearing in the grammar are converting into an "identity" mapping
/// - Each match group G is combined with the implicit priority IP of 1 for literals and 0 for
///   regex to yield the final precedence; the formula is `G*2 + IP`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatchEntry {
    /// The precedence of this match entry.
    ///
    /// NB: This field must go first, so that `PartialOrd` sorts by precedence first!
    pub precedence: usize,
    pub match_literal: TerminalLiteral,
    pub user_name: TerminalString,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternToken {
    pub span: Span,
    pub associated_types: Vec<AssociatedType>,
    pub enum_token: Option<EnumToken>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssociatedType {
    pub type_span: Span,
    pub type_name: InternedString,
    pub type_ref: TypeRef,
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
pub struct Path {
    pub absolute: bool,
    pub ids: Vec<InternedString>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRef {
    // (T1, T2)
    Tuple(Vec<TypeRef>),

    // Foo<'a, 'b, T1, T2>, Foo::Bar, etc
    Nominal {
        path: Path,
        types: Vec<TypeRef>
    },

    Ref {
        lifetime: Option<InternedString>,
        mutable: bool,
        referent: Box<TypeRef>,
    },

    // 'x ==> only should appear within nominal types, but what do we care
    Lifetime(InternedString),

    // Foo or Bar ==> treated specially since macros may care
    Id(InternedString),

    // <N> ==> type of a nonterminal, emitted by macro expansion
    OfSymbol(SymbolKind),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeParameter {
    Lifetime(InternedString),
    Id(InternedString),
}

impl TypeParameter {
    pub fn is_lifetime(&self) -> bool {
        match *self {
            TypeParameter::Lifetime(_) => true,
            _ => false
        }
    }
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
    pub annotations: Vec<Annotation>,
    pub span: Span,
    pub args: Vec<NonterminalString>, // macro arguments
    pub type_decl: Option<TypeRef>,
    pub alternatives: Vec<Alternative>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Annotation {
    pub id_span: Span,
    pub id: InternedString,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alternative {
    pub span: Span,

    pub expr: ExprSymbol,

    // if C, only legal in macros
    pub condition: Option<Condition>,

    // => { code }
    pub action: Option<ActionKind>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionKind {
    User(String),
    Fallible(String),
    Lookahead,
    Lookbehind,
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
    pub kind: SymbolKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    // (X Y)
    Expr(ExprSymbol),

    // foo, before name resolution
    AmbiguousId(InternedString),

    // "foo" and foo (after name resolution)
    Terminal(TerminalString),

    // foo, after name resolution
    Nonterminal(NonterminalString),

    // foo<..>
    Macro(MacroSymbol),

    // X+, X?, X*
    Repeat(Box<RepeatSymbol>),

    // <X>
    Choose(Box<Symbol>),

    // x:X
    Name(InternedString, Box<Symbol>),

    // @L
    Lookahead,

    // @R
    Lookbehind,
    
    Error
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerminalString {
    Literal(TerminalLiteral),
    Bare(InternedString),
    Error,
}

impl TerminalString {
    pub fn as_literal(&self) -> Option<TerminalLiteral> {
        match *self {
            TerminalString::Literal(l) => Some(l),
            _ => None
        }
    }

    pub fn display_len(&self) -> usize {
        match *self {
            TerminalString::Literal(x) => x.display_len(),
            TerminalString::Bare(x) => x.len(),
            TerminalString::Error => "error".len()
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerminalLiteral {
    Quoted(InternedString),
    Regex(InternedString),
}

impl TerminalLiteral {
    /// The *base precedence* is the precedence within a `match { }`
    /// block level. It indicates that quoted things like `"foo"` get
    /// precedence over regex matches.
    pub fn base_precedence(&self) -> usize {
        match *self {
            TerminalLiteral::Quoted(_) => 1,
            TerminalLiteral::Regex(_) => 0,
        }
    }

    pub fn display_len(&self) -> usize {
        match *self {
            TerminalLiteral::Quoted(x) => x.len(),
            TerminalLiteral::Regex(x) => x.len() + "####r".len(),
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonterminalString(pub InternedString);

impl NonterminalString
{
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Into<Box<Content>> for NonterminalString {
    fn into(self) -> Box<Content> {
        let session = Tls::session();

        InlineBuilder::new().text(self).styled(session.nonterminal_symbol).end()
    }
}

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

impl TerminalString {
    pub fn quoted(i: InternedString) -> TerminalString {
        TerminalString::Literal(TerminalLiteral::Quoted(i))
    }

    pub fn regex(i: InternedString) -> TerminalString {
        TerminalString::Literal(TerminalLiteral::Regex(i))
    }
}

impl Into<Box<Content>> for TerminalString {
    fn into(self) -> Box<Content> {
        let session = Tls::session();
        InlineBuilder::new()
            .text(self)
            .styled(session.terminal_symbol)
            .end()
    }
}

impl Grammar {
    pub fn extern_token(&self) -> Option<&ExternToken> {
        self.items.iter()
                  .flat_map(|i| i.as_extern_token())
                  .next()
    }

    pub fn enum_token(&self) -> Option<&EnumToken> {
        self.items.iter()
                  .flat_map(|i| i.as_extern_token())
                  .flat_map(|et| et.enum_token.as_ref())
                  .next()
    }

    pub fn intern_token(&self) -> Option<&InternToken> {
        self.items.iter()
                  .flat_map(|i| i.as_intern_token())
                  .next()
    }

    pub fn match_token(&self) -> Option<&MatchToken> {
        self.items.iter()
                  .flat_map(|i| i.as_match_token())
                  .next()
    }
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
            GrammarItem::MatchToken(..) => None,
            GrammarItem::ExternToken(..) => None,
            GrammarItem::InternToken(..) => None,
        }
    }

    pub fn as_match_token(&self) -> Option<&MatchToken> {
        match *self {
            GrammarItem::Nonterminal(..) => None,
            GrammarItem::Use(..) => None,
            GrammarItem::MatchToken(ref d) => Some(d),
            GrammarItem::ExternToken(..) => None,
            GrammarItem::InternToken(..) => None,
        }
    }

    pub fn as_extern_token(&self) -> Option<&ExternToken> {
        match *self {
            GrammarItem::Nonterminal(..) => None,
            GrammarItem::Use(..) => None,
            GrammarItem::MatchToken(..) => None,
            GrammarItem::ExternToken(ref d) => Some(d),
            GrammarItem::InternToken(..) => None,
        }
    }

    pub fn as_intern_token(&self) -> Option<&InternToken> {
        match *self {
            GrammarItem::Nonterminal(..) => None,
            GrammarItem::Use(..) => None,
            GrammarItem::MatchToken(..) => None,
            GrammarItem::ExternToken(..) => None,
            GrammarItem::InternToken(ref d) => Some(d),
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
        match *self {
            TerminalString::Literal(s) =>
                write!(fmt, "{}", s),
            TerminalString::Bare(s) =>
                write!(fmt, "{}", s),
            TerminalString::Error => 
                write!(fmt, "error"),
        }
    }
}

impl Debug for TerminalString {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

impl Display for TerminalLiteral {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            TerminalLiteral::Quoted(s) =>
                write!(fmt, "{:?}", s), // the Debug impl adds the `"` and escaping
            TerminalLiteral::Regex(s) =>
                write!(fmt, "r#{:?}#", s), // FIXME -- need to determine proper number of #
        }
    }
}

impl Debug for TerminalLiteral {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self)
    }
}

impl Display for Path {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}{}",
               if self.absolute {"::"} else {""},
               Sep("::", &self.ids))
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
            SymbolKind::AmbiguousId(ref s) =>
                write!(fmt, "{}", s),
            SymbolKind::Macro(ref m) =>
                write!(fmt, "{}", m),
            SymbolKind::Repeat(ref r) =>
                write!(fmt, "{}", r),
            SymbolKind::Choose(ref s) =>
                write!(fmt, "<{}>", s),
            SymbolKind::Name(n, ref s) =>
                write!(fmt, "{}:{}", n, s),
            SymbolKind::Lookahead =>
                write!(fmt, "@L"),
            SymbolKind::Lookbehind =>
                write!(fmt, "@R"),
            SymbolKind::Error =>
                write!(fmt, "error"),
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

impl ExternToken {
    pub fn associated_type(&self, name: InternedString) -> Option<&AssociatedType> {
        self.associated_types.iter()
                             .filter(|a| a.type_name == name)
                             .next()
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

impl Display for TypeParameter {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            TypeParameter::Lifetime(s) => write!(fmt, "{}", s),
            TypeParameter::Id(s) => write!(fmt, "{}", s),
        }
    }
}

impl Display for TypeRef {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            TypeRef::Tuple(ref types) =>
                write!(fmt, "({})", Sep(", ", types)),
            TypeRef::Nominal { ref path, ref types } if types.len() == 0 =>
                write!(fmt, "{}", path),
            TypeRef::Nominal { ref path, ref types } =>
                write!(fmt, "{}<{}>", path, Sep(", ", types)),
            TypeRef::Lifetime(ref s) =>
                write!(fmt, "{}", s),
            TypeRef::Id(ref s) =>
                write!(fmt, "{}", s),
            TypeRef::OfSymbol(ref s) =>
                write!(fmt, "`{}`", s),
            TypeRef::Ref { lifetime: None, mutable: false, ref referent } =>
                write!(fmt, "&{}", referent),
            TypeRef::Ref { lifetime: Some(l), mutable: false, ref referent } =>
                write!(fmt, "&{} {}", l, referent),
            TypeRef::Ref { lifetime: None, mutable: true, ref referent } =>
                write!(fmt, "&mut {}", referent),
            TypeRef::Ref { lifetime: Some(l), mutable: true, ref referent } =>
                write!(fmt, "&{} mut {}", l, referent),
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
                    path: Path::from_id(id),
                    types: vec![]
                }),
            TypeRef::OfSymbol(_) =>
                unreachable!("OfSymbol produced by parser"),
            TypeRef::Ref { lifetime, mutable, ref referent } =>
                TypeRepr::Ref { lifetime: lifetime,
                                mutable: mutable,
                                referent: Box::new(referent.type_repr()) },
        }
    }
}

impl Path {
    pub fn from_id(id: InternedString) -> Path {
        Path {
            absolute: false,
            ids: vec![id]
        }
    }

    pub fn usize() -> Path {
        Path {
            absolute: false,
            ids: vec![intern("usize")]
        }
    }

    pub fn str() -> Path {
        Path {
            absolute: false,
            ids: vec![intern("str")]
        }
    }

    pub fn vec() -> Path {
        Path {
            absolute: true,
            ids: vec![intern("std"), intern("vec"), intern("Vec")]
        }
    }

    pub fn option() -> Path {
        Path {
            absolute: true,
            ids: vec![intern("std"), intern("option"), intern("Option")]
        }
    }

    pub fn as_id(&self) -> Option<InternedString> {
        if !self.absolute && self.ids.len() == 1 {
            Some(self.ids[0])
        } else {
            None
        }
    }
}

pub fn read_algorithm(annotations: &[Annotation], algorithm: &mut r::Algorithm) {
    for annotation in annotations {
        if annotation.id == intern(LALR) {
            algorithm.lalr = true;
        } else if annotation.id == intern(TABLE_DRIVEN) {
            algorithm.codegen = r::LrCodeGeneration::TableDriven;
        } else if annotation.id == intern(RECURSIVE_ASCENT) {
            algorithm.codegen = r::LrCodeGeneration::RecursiveAscent;
        } else if annotation.id == intern(TEST_ALL) {
            algorithm.codegen = r::LrCodeGeneration::TestAll;
        } else {
            panic!("validation permitted unknown annotation: {:?}",
                    annotation.id);
        }
    }
}
