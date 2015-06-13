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
  Expr = Alt => { };

  // Declare a "choice" nonterminal.
  Expr: Type = {
    "class" "Id" "{" Foo+ Foo* => {
        // action code
    };
    "foo" "bar" => {
    };
  };

}
```

*/

use intern::InternedString;
use grammar::TypeName;

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
    pub expr: SymbolExpr,

    // => { code }
    pub action: Option<String>,
}

// "foo" <z:"bar"> etc
#[derive(Clone, Debug)]
pub struct SymbolExpr {
    pub args: Vec<SymbolArg>
}

#[derive(Clone, Debug)]
pub struct SymbolArg {
    pub symbol: Symbol,
    pub highlight: SymbolHighlight,
}

#[derive(Copy, Clone, Debug)]
pub enum SymbolHighlight {
    // X
    None,

    // <X>
    Anon,

    // <name:X>
    Named(InternedString)
}

#[derive(Clone, Debug)]
pub enum Symbol {
    // "foo"
    Terminal(InternedString),

    // foo
    Nonterminal(InternedString),

    // X+ or (X Y Z)+
    Plus(SymbolExpr),

    // X? or (X Y Z)?
    Question(SymbolExpr),

    // X* or (X Y Z)*
    Star(SymbolExpr),
}
