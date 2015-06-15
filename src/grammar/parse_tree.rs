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
  Comma<$A>: Vec<$A> = {
      ~v:(~$A ",")* ~e:(~$A ,?)?> => {
          let mut v = v; v.push(e); v
      }
  };

  // Example 2: conditional patterns
  Expr<$M>: Expr = {

      ~Expr "(" ~Comma<Expr> ")" => Expr::CallExpr(~~~);

      ID if $M != "NO_ID" => {
      };

  };
}
```

*/

use intern::InternedString;
use grammar::ty::TypeName;

#[derive(Clone, Debug)]
pub struct Grammar {
    pub type_name: TypeName,
    pub items: Vec<GrammarItem>,
}

#[derive(Clone, Debug)]
pub enum GrammarItem {
    TokenType(TokenTypeData),
    Nonterminal(NonterminalData),
}

#[derive(Clone, Debug)]
pub struct TokenTypeData {
    pub type_name: TypeName,
    pub conversions: Vec<(InternedString, InternedString)>,
}

#[derive(Clone, Debug)]
pub struct NonterminalData {
    pub name: InternedString,
    pub type_decl: Option<String>,
    pub alternatives: Vec<Alternative>
}

#[derive(Clone, Debug)]
pub struct Alternative {
    pub expr: Vec<Symbol>,

    // => { code }
    pub action: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Symbol {
    // (<X> <Y>) etc
    Expr(Vec<Symbol>),

    // "foo"
    Terminal(InternedString),

    // foo
    Nonterminal(InternedString),

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
