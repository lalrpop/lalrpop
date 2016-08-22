# Writing a custom lexer

Let's say we want to parse the Whitespace language, so we've put together a grammar like the following:

```rust
pub Program = <Statement*>;

Statement: ast::Stmt = {
    " " <StackOp>,
    "\t" " " <MathOp>,
    "\t" "\t" <HeapOp>,
    "\n" <FlowCtrl>,
    "\t" "\n" <Io>,
};

StackOp: ast::Stmt = {
    " " <Number> => ast::Stmt::Push(<>),
    "\n" " " => ast::Stmt::Dup,
    "\n" "\t" => ast::Stmt::Swap,
    "\n" "\n" => ast::Stmt::Discard,
};

MathOp: ast::Stmt = {
    " " " " => ast::Stmt::Add,
    " " "\t" => ast::Stmt::Sub,
    " " "\n" => ast::Stmt::Mul,
    "\t" " " => ast::Stmt::Div,
    "\t" "\t" => ast::Stmt::Mod,
};

// Remainder omitted
```

Naturally, it doesn't work. By default, LALRPOP generates a tokenizer that skips all whitespace -- including newlines. What we *want* is to capture whitespace characters and ignore the rest as comments, and LALRPOP does the opposite of that.

At the moment, LALRPOP doesn't allow you to configure the default tokenizer. In the future it will become quite flexible, but for now we have to write our own.

Let's start by defining the stream format. The parser will accept an iterator where each item in the stream has the following structure:

```rust
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
```

`Loc` is typically just a `usize`, representing a byte offset into the input string. Each token is accompanied by two of them, marking the start and end positions where it was found. `Error` can be pretty much anything you choose. And of course `Tok` is the meat of the stream, defining what possible values the tokens themselves can have. Following the conventions of Rust iterators, we'll signal a valid token with `Some(Ok(...))`, an error with `Some(Err(...))`, and EOF with `None`.

(Note that the term "tokenizer" normally refers to a piece of code that simply splits up the stream, whereas a "lexer" also tags each token with its lexical category. What we're writing is the latter.)

Whitespace is a simple language from a lexical standpoint, with only three valid tokens:

```rust
pub enum Tok {
    Space,
    Tab,
    Linefeed,
}
```

Everything else is a comment. There are no invalid lexes, so we'll define our own error type, a void enum:

```rust
pub enum LexicalError {
    // Not possible
}
```

Now for the lexer itself. We'll take a string slice as its input. For each token we process, we'll want to know the character value, and the byte offset in the string where it begins. We can do that by wrapping the `CharIndices` iterator, which yields tuples of `(usize, char)` representing exactly that information.

```rust
use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { chars: input.char_indices() }
    }
}
```

(The lifetime parameter `'input` indicates that the Lexer cannot outlive the string it's trying to parse.)

Let's review our rules:

- For a space character, we output `Tok::Space`.
- For a tab character, we output `Tok::Tab`.
- For a linefeed (newline) character, we output `Tok::Linefeed`.
- We skip all other characters.
- If we've reached the end of the string, we'll return `None` to signal EOF.

Writing a lexer for a language with multi-character tokens can get very complicated, but this is so straightforward, we can translate it directly into code without thinking very hard. Here's our `Iterator` implementation:

```rust
impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, ' ')) => return Some(Ok((i, Tok::Space, i+1))),
                Some((i, '\t')) => return Some(Ok((i, Tok::Tab, i+1))),
                Some((i, '\n')) => return Some(Ok((i, Tok::Linefeed, i+1))),

                None => return None, // End of file
                _ => continue, // Comment; skip this character
            }
        }
    }
}
```

That's it. That's all we need.

## Updating the parser

To use this with LALRPOP, we need to expose its API to the parser. It's pretty easy to do, but also somewhat magical, so pay close attention. At the bottom of the grammar file, we need an `extern` block:

```rust
extern {
    // ...
}
```

Now we tell LALRPOP about the `Location` and `Error` types, as if we're writing a trait:

```rust
extern {
    type Location = usize;
    type Error = lexer::LexicalError;
    
    // ...
}
```

We expose the `Tok` type by kinda sorta redeclaring it:

```rust
extern {
    type Location = usize;
    type Error = lexer::LexicalError;

    enum lexer::Tok {
        // ...
    }
}
```

Now we have to declare each of our terminals. For each variant of `Tok`, we pick what name the parser will see, and write a pattern of the form `name => lexer::Tok::Variant`, similar to how action code works in grammar rules. The name can be an identifier, or a string literal. We'll use the latter.

Here's the whole thing:

```rust
extern {
    type Location = usize;
    type Error = lexer::LexicalError;

    enum lexer::Tok {
        " " => lexer::Tok::Space,
        "\t" => lexer::Tok::Tab,
        "\n" => lexer::Tok::Linefeed,
    }
}
```

From now on, the parser will take a `Lexer` as its input instead of a string slice. And any time we write a string literal in the grammar, it'll substitute a variant of our `Tok` enum. This means **we don't have to change any of the rules we already wrote!** This will work as-is:

```rust
FlowCtrl: ast::Stmt = {
    " " " " <Label> => ast::Stmt::Mark(<>),
    " " "\t" <Label> => ast::Stmt::Call(<>),
    " " "\n" <Label> => ast::Stmt::Jump(<>),
    "\t" " " <Label> => ast::Stmt::Jz(<>),
    "\t" "\t" <Label> => ast::Stmt::Js(<>),
    "\t" "\n" => ast::Stmt::Return,
    "\n" "\n" => ast::Stmt::Exit,
};
```

The complete grammar is available in `whitespace/src/parser.lalrpop`.

## Where to go from here

Things to try that apply to lexers in general:

- Longer tokens
- Tokens that require tracking internal lexer state

Things to try that are LALRPOP-specific:

- Persuade a lexer generator to output the `Spanned` format
- Make this tutorial better
