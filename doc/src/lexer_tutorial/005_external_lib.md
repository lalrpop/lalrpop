# Using an external library

Writing a lexer yourself can be tricky. Fortunately, you can find on
[crates.io](https://crates.io) many different libraries to generate a lexer for
you.

In this tutorial, we will use [Logos](https://docs.rs/logos/latest/logos/) to
build a simple lexer for a toy programming language. Here is an example of what
we will be able to parse:

```
var a = 42;
var b = 23;

# a comment
print (a - b);
```

## Setup

In your `Cargo.toml`, add the following dependency:

```toml
logos = "0.14"
```

This will provide the `logos` crate and the `Logos` trait.

## The AST

We will use the following abstract syntax tree (AST) as a representation of our expressions:

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  Variable { name: String, value: Box<Expression> },
  Print { value: Box<Expression> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
  Integer(i64),
  Variable(String),
  BinaryOperation { lhs: Box<Expression>, operator: Operator, rhs: Box<Expression> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
}
```

## Implement the tokenizer

In a file named `tokens.rs` (or any other name you want), create an enumeration
for your tokens, as well as a type for lexing errors:

```rust
use std::fmt;  // to implement the Display trait later
use std::num::ParseIntError;
use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<Infallible> for LexicalError {
    fn from(_: Infallible) -> Self {
        unreachable!();
    }
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"#.*\n?", error = LexicalError)]
pub enum Token {
  #[token("var")]
  KeywordVar,
  #[token("print")]
  KeywordPrint,

  #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
  Identifier(String),
  #[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
  Integer(i64),

  #[token("(")]
  LParen,
  #[token(")")]
  RParen,
  #[token("=")]
  Assign,
  #[token(";")]
  Semicolon,

  #[token("+")]
  OperatorAdd,
  #[token("-")]
  OperatorSub,
  #[token("*")]
  OperatorMul,
  #[token("/")]
  OperatorDiv,
}
```

An exact match is specified using the `#[token(...)]` attribute.

For example, `#[token("+")]` makes the `OperatorAdd` token to be emitted only
when a literal `"+"` appears in the input (unless it is part of another match,
see below).

On the other hand, `#[regex(...)]` will match a regular expression.

The `#[logos(...)]` attribute around the enum defines regexes to skip when
parsing the input. We've chosen to skip common whitespace characters, and
single-line comments of the form `# ...`. It also allows us to specify our
custom error type, `LexicalError` if an unexpected token was encountered or if
parsing an integer fails.

A few things to note about how **Logos** works:

When several sequences of tokens can match the same input, Logos uses precise
rules to make a choice. Rule of thumb is:

- Longer beats shorter.
- Specific beats generic.

This means the `"printa"` input string will generate the following token:

- `Token::Identifier(String::new("printa"))`

And not:

- `Token::KeywordPrint`
- `Token::Identifier(String::new("a"))`

This is because `printa` is longer than `print`, therefore the `Identifier` rule
has priority.

Finally, we implement the `Display` trait:

```rust
impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
```

This is required because the token is included in the error message that
LALRPOP generates if it fails at parsing.

## Implement the lexer

This part is very similar to the previous tutorials. In a file `lexer.rs` (or
any other name), we will implement the Lexer as required by LALRPOP.

First, we define our types and structures:

```rust
use logos::{Logos, SpannedIter};

use crate::tokens::{Token, LexicalError}; // your Token enum, as above

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
  // instead of an iterator over characters, we have a token iterator
  token_stream: SpannedIter<'input, Token>,
}
```

Then, we create the constructor for our Lexer:

```rust
impl<'input> Lexer<'input> {
  pub fn new(input: &'input str) -> Self {
    // the Token::lexer() method is provided by the Logos trait
    Self { token_stream: Token::lexer(input).spanned() }
  }
}
```

Finally, we implement the `Iterator` trait:

```rust
impl<'input> Iterator for Lexer<'input> {
  type Item = Spanned<Token, usize, LexicalError>;

  fn next(&mut self) -> Option<Self::Item> {
    self.token_stream
      .next()
      .map(|(token, span)| Ok((span.start, token?, span.end)))
  }
}
```

## Update the grammar

Next, in our `grammar.lalrpop` file (or any other name), we can integrate our
lexer as follows:

```lalrpop
use crate::tokens::{Token, LexicalError};
use crate::ast;

grammar;

// ...

extern {
  type Location = usize;
  type Error = LexicalError;

  enum Token {
    "var" => Token::KeywordVar,
    "print" => Token::KeywordPrint,
    "identifier" => Token::Identifier(<String>),
    "int" => Token::Integer(<i64>),
    "(" => Token::LParen,
    ")" => Token::RParen,
    "=" => Token::Assign,
    ";" => Token::Semicolon,
    "+" => Token::OperatorAdd,
    "-" => Token::OperatorSub,
    "*" => Token::OperatorMul,
    "/" => Token::OperatorDiv,
  }
}
```

**NB:** This part allows us to give a precise name to the tokens emitted by our
Lexer. We can then use those names (`"identifier"`, `"var"`, ...) in our grammar
rules to reference the desired token.

Finally, we can build our rules:

```rust
pub Script: Vec<Box<ast::Statement>> = {
  <stmts:Statement*> => stmts
}

pub Statement: Box<ast::Statement> = {
  "var" <name:"identifier"> "=" <value:Expression> ";" => {
    Box::new(ast::Statement::Variable { name, value })
  },
  "print" <value:Expression> ";" => {
    Box::new(ast::Statement::Print { value })
  },
}

pub Expression: Box<ast::Expression> = {
  #[precedence(lvl="1")]
  Term,

  #[precedence(lvl="2")] #[assoc(side="left")]
  <lhs:Expression> "*" <rhs:Expression> => {
    Box::new(ast::Expression::BinaryOperation {
      lhs,
      operator: ast::Operator::Mul,
      rhs
    })
  },
  <lhs:Expression> "/" <rhs:Expression> => {
    Box::new(ast::Expression::BinaryOperation {
      lhs,
      operator: ast::Operator::Div,
      rhs
    })
  },

  #[precedence(lvl="3")] #[assoc(side="left")]
  <lhs:Expression> "+" <rhs:Expression> => {
    Box::new(ast::Expression::BinaryOperation {
      lhs,
      operator: ast::Operator::Add,
      rhs
    })
  },
  <lhs:Expression> "-" <rhs:Expression> => {
    Box::new(ast::Expression::BinaryOperation {
      lhs,
      operator: ast::Operator::Sub,
      rhs
    })
  },
}

pub Term: Box<ast::Expression> => {
  <val:"int"> => {
    Box::new(ast::Expression::Integer(val))
  },
  <name:"identifier"> => {
    Box::new(ast::Expression::Variable(name))
  },
  "(" Expression ")",
}
```

Our grammar is now complete.

## Running your parser

The last step is to run our parser:

```rust
let source_code = std::fs::read_to_string("myscript.toy")?;
let lexer = Lexer::new(&source_code[..]);
let parser = ScriptParser::new();
let ast = parser.parse(lexer)?;

println!("{:?}", ast);
```
