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
logos = "0.12.0"
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
for your tokens:

```rust
use std::fmt;  // to implement the Display trait
use logos::Logos;

#[derive(Logos, clone, Debug, PartialEq)]
pub enum Token {
  #[token("var")]
  KeywordVar,
  #[token("print")]
  KeywordPrint,

  #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
  Identifier(String),
  #[regex("\d+", |lex| lex.slice().parse())]
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

  #[regex(r"#.*\n?", logos::skip)]
  #[regex(r"[ \t\n\f]+", logos::skip)]
  #[error]
  Error,
}
```

An exact match is specified using the `#[token()]` annotation. For example, `#[token("+")]` makes the `OperatorAdd` token to be emitted only when a literal `"+"` appears in the input (unless it is part of another match, see below). On the other hand, `#[regex()]` will match a regular expression. For example, combined with the `logos::skip` annotation, `#[regex(r"#.*\n?", logos::skip)]` causes all matches of a `#` character until a newline character (comments of our language) to be ignored.

A few things to note about how **Logos** works:

When several sequences of tokens can match the same input, Logos uses precise rules to make a choice. Rule of thumb is:
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

## Implement the lexer

This part is very similar to the previous tutorials. In a file `lexer.rs` (or
any other name), we will implement the Lexer as required by LALRPOP.

First, we define our types and structures:

```rust
use logos::{Logos, SpannedIter};

use crate::path::to::tokens::Token; // your enum

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub enum LexicalError {
  InvalidToken,
  ShouldNotHappen,
}

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
self.token_stream.next().map(|(token, span)| {
  match token {
    // an invalid token was met
    Token::Error => Err(LexicalError::InvalidToken),
    _ => Ok((span.start, token, span.end)),
  }
})
    }
  }
}
```

## Update the grammar

Next, in our `grammar.lalrpop` file (or any other name), we can integrate our
lexer as follows:

```lalrpop
use lalrpop_util::ParseError;
use crate::path:to:{
  tokens::Token,
  lexer::LexicalError,
  ast,
};

grammar;

// ...

extern {
  type Location = usize;
  type Error = LexicalError;

  enum Token {
    "var" => Token::KeywordVar,
    "print" => Token::KeywordPrint,
    "identifier" => Token::Identifier(_),
    "int" => Token::Integer(_),
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

Finally, we can build our rules:

```rust
pub Script: Vec<Box<ast::Statement>> = {
  <stmts:Statement*> => stmts
}

pub Statement: Box<ast::Statement> = {
  "var" <id:"identifier"> "=" <value:Expression> ";" =>? {
    match id {
      Token::Identifier(name) => {
        Ok(Box::new(ast::Statement::Variable {
          name,
          value,
        }))
      },
      _ => {
        // this should not happen, but the compiler cannot guarantee it
        // it will make sure that an error is returned if you refactor your lexer
        Err(ParseError::User { error: LexicalError::ShouldNotHappen })
      }
    }
  },
  "print" <value:Expression> ";" => Box::new(ast::Statement::Print { value })
}

pub Expression: Box<ast::Expression> = {
  #[precedence(lvl="1")
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
  
  #[precedence(lvl="3")]
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
  <i:"int"> =>? {
    match i {
      Token::Integer(val) => Ok(Box::new(ast::Expression::Integer(val))),
      _ => Err(ParseError::User { error: LexicalError::ShouldNotHappen }),
    }
  },
  <x:"identifier"> =>? {
    match x {
      Token::Identifier(name) => Ok(Box::new(ast::Expression::Identifier(name))),
      _ => Err(ParseError::User { error: LexicalError::ShouldNotHappen })
    }
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
