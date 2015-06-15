/*!

The "parse-tree" is what is produced by the parser. We use it do
some pre-expansion and so forth before creating the proper AST.

Here is an example file to give you the idea:

```
grammar Type<'input, T> {

  // External token type; "xxx" is assumed to map
  // to a variant name, but we can do some substitutions
  // for you. Input will be an `Iterator<Item=T>`.
  //
  // Eventually this should become optional, but not
  // for the first version because I am lazy.
  token parser::Token<T> where {
    "(" => LParen;
    ")" => RParen;
  };

  // Declare an "aliasing" nonterminal.
  Expr = Alt;

  // ...which can optionally map.
  Expr = Alt => code;

  // Declare a "match" nonterminal.
  Expr: Type = {
    "class" "Id" "{" Foo+ Foo* => {
        // action code
    }
    "foo" "bar" if $X ~ "[COMMA]" => {
    }
  };

  // Macro nonterminals. Macro arguments may be either any
  // symbol expressions and may be used in types, definitions,
  // or guard expressions.

  // Example 1: comma-separated list with optional trailing comma.
  Comma<E>: Vec<E> = {
      ~v:(~E ",")* ~e:E? => {
          let mut v = v;
          if let Some(e) = e { v.push(e); }
          v
      };
  };

  // Example 2: conditional patterns
  Expr<M>: Expr = {
      ~Expr "(" ~Comma<Expr> ")" => Expr::CallExpr(~~);

      ID if M !~ "NO_ID" => {
      };
  };
}
```

*/

use intern::InternedString;

#[derive(Clone, Debug)]
pub struct Grammar {
    pub type_name: TypeRef,
    pub items: Vec<GrammarItem>,
}

#[derive(Clone, Debug)]
pub enum GrammarItem {
    TokenType(TokenTypeData),
    Nonterminal(NonterminalData),
}

#[derive(Clone, Debug)]
pub struct TokenTypeData {
    pub type_name: TypeRef,
    pub conversions: Vec<(InternedString, InternedString)>,
}

#[derive(Clone, Debug)]
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
    Nonterminal(InternedString),
}

#[derive(Clone, Debug)]
pub struct NonterminalData {
    pub name: InternedString,
    pub args: Vec<InternedString>, // macro arguments
    pub type_decl: Option<TypeRef>,
    pub alternatives: Vec<Alternative>
}

#[derive(Clone, Debug)]
pub struct Alternative {
    pub expr: Vec<Symbol>,

    // if C, only legal in macros
    pub condition: Option<Condition>,

    // => { code }
    pub action: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Condition {
    // X == "Foo", equality
    Equals(InternedString, InternedString),

    // X != "Foo", inequality
    NotEquals(InternedString, InternedString),

    // X ~~ "Foo", regexp match
    Match(InternedString, InternedString),

    // X !~ "Foo", regexp non-match
    NotMatch(InternedString, InternedString),
}

#[derive(Clone, Debug)]
pub enum Symbol {
    // (<X> <Y>) etc
    Expr(Vec<Symbol>),

    // "foo"
    Terminal(InternedString),

    // foo
    Nonterminal(InternedString),

    // foo<..>
    Macro(InternedString, Vec<Symbol>),

    // X+
    Plus(Box<Symbol>),

    // X?
    Question(Box<Symbol>),

    // X*
    Star(Box<Symbol>),

    // ~X
    Choose(Box<Symbol>),

    // ~x:X
    Name(InternedString, Box<Symbol>),
}
