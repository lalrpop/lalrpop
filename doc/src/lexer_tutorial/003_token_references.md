# Using tokens with references

When using a custom lexer, you might want tokens to hold references to the original input.
This allows to use references to the input when the grammar can have arbitrary symbols such as variable names.
Using references instead of copying the symbols can improve performance and memory usage of the parser.

## The Lexer

We can now create a new calculator parser that can deal with symbols the same way an interpreter would deal with variables.
First we need the corresponding AST :

``` rust
pub enum ExprSymbol<'input>{
    NumSymbol(&'input str),
    Op(Box<ExprSymbol<'input>>, Opcode, Box<ExprSymbol<'input>>),
    Error,
}
```

Then, we need to build the tokens:


``` rust
#[derive(Copy, Clone, Debug)]
pub enum Tok<'input> {
    NumSymbol(&'input str),
    FactorOp(Opcode),
    ExprOp(Opcode),
    ParenOpen,
    ParenClose,
}
```

Then, we can build the lexer itself.
It's  quite simple, it returns any operator, and if it detects any other character, stores the beginning then continues to the next operator and sends the symbol it just parsed.

``` rust
use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, ' '))  | Some((_, '\n')) | Some((_, '\t')) => continue,
                Some((i, ')')) => return Some(Ok((i, Tok::ParenClose, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Tok::ParenOpen, i + 1))),
                Some((i, '+')) => return Some(Ok((i, Tok::ExprOp(Opcode::Add), i + 1))),
                Some((i, '-')) => return Some(Ok((i, Tok::ExprOp(Opcode::Sub), i + 1))),
                Some((i, '*')) => return Some(Ok((i, Tok::FactorOp(Opcode::Mul), i + 1))),
                Some((i, '/')) => return Some(Ok((i, Tok::FactorOp(Opcode::Div), i + 1))),

                None => return None, // End of file
                Some((i,_)) => {
                    loop {
                        match self.chars.peek() {
                            Some((j, ')'))|Some((j, '('))|Some((j, '+'))|Some((j, '-'))|Some((j, '*'))|Some((j, '/'))|Some((j,' '))
                            => return Some(Ok((i, Tok::NumSymbol(&self.input[i..*j]), *j))),
                            None => return Some(Ok((i, Tok::NumSymbol(&self.input[i..]),self.input.len()))),
                            _ => {self.chars.next();},
                        }
                    }
                }
            }
        }
    }
}
```

## The parser

We can then take a look at the corresponding parser with a new grammar:

``` rust
Term: Box<ExprSymbol<'input>> = {
    "num" => Box::new(ExprSymbol::NumSymbol(<>)),
    "(" <Expr> ")"
};
```

We need to pass the input to the parser so that the input's lifetime is known to the borrow checker when compiling the generated parser.
``` rust
grammar<'input>(input: &'input str);
```

Then we just need to define the tokens the same as before :

``` rust
extern {
    type Location = usize;
    type Error = ();
    
    enum Tok<'input> {
        "num" => Tok::NumSymbol(<&'input str>),
        "FactorOp" => Tok::FactorOp(<Opcode>),
        "ExprOp" => Tok::ExprOp(<Opcode>),
        "(" => Tok::ParenOpen,
        ")" => Tok::ParenClose,
    }
}
```

# Calling the parser
We can finally run the parser we built:

``` rust
let input = "22 * pi + 66";
let lexer = Lexer::new(input);
let expr = calculator8::ExprParser::new()
    .parse(input,lexer)
    .unwrap();
assert_eq!(&format!("{:?}", expr), "((\"22\" * \"pi\") + \"66\")");
```
