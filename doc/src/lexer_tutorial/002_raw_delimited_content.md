# Lexing raw delimited content

Our calculator example operated on numbers and arithmetic operators.
There is no overlap between the characters for numeric digits (`0`, `1`, ...),
the characters representing operators (`+`, `-`, ...) and parentheses
(`(`, `)`), so it was easy to embed those tokens directly in the grammar,
as we saw in the earlier sections.

However, some languages do not have such easy separations at the lexical level.

Consider parsing a language with string literals. We will deliberately choose
one that is very simple: All it can do is bind variables, and the variables are
always single characters, like this:

```
x = "a"
y = "bc"
```

Using what we have learned so far, we might try a grammar like the following one:

```lalrpop
use super::{Var, Lit, Stmt};

grammar;

pub Var: Var = <r"[x-z]"> => <>.chars().next().unwrap().into();

pub Lit: Lit = "\"" <r"[a-c]*"> "\"" => <>.into();

pub Stmt: Stmt = <Var> "=" <Lit> => (<>).into();
```

This grammar deliberately uses regular expressions that are very limited in scope,
ensuring that there is no overlap between variable names and string contents,
so that we can see what kinds of cases that this approach *does* handle.

We can combine the above with some library code and get a system that seems to work.

```rust
lalrpop_mod!(pub nobol1); // syntesized by LALRPOP

#[derive(PartialEq, Debug)]
pub struct Var(char);

#[derive(PartialEq, Debug)]
pub struct Lit(String);

#[derive(PartialEq, Debug)]
pub struct Stmt(Var, Lit);

impl From<char> for Var { fn from(c: char) -> Self { Var(c) } }
impl From<&str> for Lit { fn from(s: &str) -> Self { Lit(s.into()) } }
impl From<String> for Lit { fn from(s: String) -> Self { Lit(s.into()) } }
impl From<(Var, Lit)> for Stmt { fn from((v, l): (Var, Lit)) -> Self { Stmt(v, l) } }
impl From<(char, &str)> for Stmt { fn from((v, l): (char, &str)) -> Self { Stmt(v.into(), l.into()) } }

#[test]
fn nobol1() {
    assert_eq!(nobol1::VarParser::new().parse("x"), Ok('x'.into()));
    assert_eq!(nobol1::LitParser::new().parse(r#""abc""#), Ok("abc".into()));
    assert_eq!(nobol1::StmtParser::new().parse(r#"x = "a""#), Ok(('x', "a").into()));
    assert_eq!(nobol1::StmtParser::new().parse(r#"y = "bc""#), Ok(('y', "bc").into()));
}
```

However, as soon as we try to handle examples where there *is* overlap between
the variable names and the string contents, we run into a problem.

Consider an example like this:

```
z = "xyz"
```

If we encode that as a test,

```rust
#[test]
fn no_ball() {
    assert_eq!(nobol1::StmtParser::new().parse(r#"z = "xyz""#), Ok(('z', "xyz").into()));
}
```

we get the following:

```
thread 'no_ball' panicked at 'assertion failed: `(left == right)`
  left: `Err(UnrecognizedToken { token: (5, Token(1, "x"), 6), expected: ["r#\"[a-c]*\"#"] })`,
 right: `Ok(Stmt(Var('z'), Lit("xyz")))`', doc/nobol/src/main.rs:31:5
```

Okay, this sounds easy to fix: the regular expression for string literal content should
be expanded to handle characters other than `a`, `b` and `c`.

So does this work?

```lalrpop
pub Lit: Lit = "\"" <r"[a-z]*"> "\"" => <>.into();
```

No, not on its own. If you just put that in, you get

```
error: ambiguity detected between the terminal `r#"[x-z]"#` and the terminal `r#"[a-z]*"#`
```

We saw the explanation for why this happens in the previous section: the two
regular expressions overlap, and the generated lexer does not know how to
resolve the ambiguity between them.

#### Cut to the chase?

If you want to know "the right way" to solve this problem, you
can skip straight to [the end][].

[the end]: #The-right-way-to-do-this

But if you want to understand *why* it is the right answer, you may
benefit from taking the detour that starts now.

#### Exploring our options

A `match` declaration here, as suggested in the previous chapter, might seem
like it fixes the problem:

```
use super::{Var, Lit, Stmt};

grammar;

match {
   r"[x-z]"
} else {
   r"[a-z]*",
   _
}

pub Var: Var = <r"[x-z]"> => <>.chars().next().unwrap().into();

pub Lit: Lit = "\"" <r"[a-z]*"> "\"" => <>.into();

pub Stmt: Stmt = <Var> "=" <Lit> => (<>).into();
```

With that `match` declaration in place we can successfully run a test like this
one:

```rust
#[test]
fn fair_ball() {
    assert_eq!(nobol2::StmtParser::new().parse(r#"z = "xyz""#), Ok(('z', "xyz").into()));
}
```

Unfortunately, the `match` is actually only papering over the fundamental problem here.
Consider this variant of the previous test:

```rust
#[test]
fn foul_ball() {
    assert_eq!(nobol2::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
}
```

The above produces:

```
---- foul_ball stdout ----
thread 'foul_ball' panicked at 'assertion failed: `(left == right)`
  left: `Err(UnrecognizedToken { token: (5, Token(3, "x"), 6), expected: ["r#\"[a-z]*\"#"] })`,
 right: `Ok(Stmt(Var('z'), Lit("x")))`', doc/nobol/src/main.rs:43:5
```

What is the problem?

Merely specifying a precedence to favor tokenizing `r"[x-z]"` over `r"[a-z]*"`
does not address the real problem here. That precedence rule causes an input
like `z = "x"` to be split into tokens such that the `x` only matches the
regular expression for the `Var`. It will not match the `r"[a-z]*"` in the `Lit`
rule, even if it intuitively seems like it should; they have already been
lexically categorized as different tokens at this point.

One could add further workarounds to deal with this. For example, one could
change the `Lit` production to explicitly handle the `r"[x-z]"` regular
expression as its own case:

```lalrpop
pub Lit: Lit = {
    "\"" <r"[x-z]"> "\"" => <>.into(),
    "\"" <r"[a-z]*"> "\"" => <>.into(),
};
```

But this is a fragile workaround.

Specifically, this workaround is only applicable because we put artificial
limits on this language.

If we wanted to generalize string literals to be able to contain other
characters (such as whitespace), the technique described so far does not work
out well. Consider this grammar:

```lalrpop
match {
   r"[x-z]"
} else {
   r"[a-z ]*",
   _
}

pub Var: Var = <r"[x-z]"> => <>.chars().next().unwrap().into();

pub Lit: Lit = {
    "\"" <r"[x-z]"> "\"" => <>.into(),
    "\"" <r"[a-z ]*"> "\"" => <>.into(),
};

pub Stmt: Stmt = <Var> "=" <Lit> => (<>).into();
```

Now, if we run the same test as before:

```rust
#[test]
fn spaceballs() {
    assert_eq!(nobol4::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
}
```

we get the following error output:

```
thread 'spaceballs' panicked at 'assertion failed: `(left == right)`
  left: `Err(UnrecognizedToken { token: (0, Token(2, "z "), 2), expected: ["r#\"[x-z]*\"#"] })`,
 right: `Ok(Stmt(Var('z'), Lit("x")))`', doc/nobol/src/main.rs:58:5
```

Our attempt to generalize what strings can contain has caused problems for
how the *rest* of the input is tokenized.


#### The right way to do this

Let us revisit the original rule in the grammar for string literals, from our
first version:

```lalrpop
pub Lit: Lit = "\"" <r"[a-c]*"> "\"" => <>.into();
```

The heart of our problem is that we have implicitly specified distinct tokens
for the string delimiter (`"\""`) versus the string content (in this case,
`r"[a-c]*"`).

The problem is that our intuition is that we only want to tokenize string
content when we are in the process of reading a string. In other words, we only
want to apply the `r"[a-c]*"` rule immediately after reading a `"\""`. But the
generated lexer does not infer this from our rules; it just blindly looks for
something matching the string content regular expression *anywhere*
in the input.

You could solve this with a custom lexer (treated in the next section).

But the simplest thing to do in this scenario is to read the string delimiters and the string content as a *single token*, like so:

```lalrpop
pub Var: Var = <r"[a-z]"> => <>.chars().next().unwrap().into();

pub Lit: Lit = <l:r#""[a-z ]*""#> => l[1..l.len()-1].into();

pub Stmt: Stmt = <Var> "=" <Lit> => (<>).into();
```

(Note that this form of the grammar does not require any `match` statement;
there is no longer any ambiguity between the different regular expressions that
drive the tokenizer.)

With this definition of the grammar, all of these tests pass:

```
#[test]
fn homerun() {
    assert_eq!(nobol5::VarParser::new().parse("x"), Ok('x'.into()));
    assert_eq!(nobol5::LitParser::new().parse(r#""abc""#), Ok("abc".into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"x = "a""#), Ok(('x', "a").into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"y = "bc""#), Ok(('y', "bc").into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"z = "xyz""#), Ok(('z', "xyz").into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"z = "x y z""#), Ok(('z', "x y z").into()));
}
```

Furthermore, we can now remove other artificial limits in our language.
For example, we can make our identifiers more than one character:

```lalrpop
pub Var: Var = <r"[a-z]+"> => <>.into()
```

which, with suitable changes to the library code, works out fine.

#### Escape sequences

Our current string literals are allowed to hold a small subset of the full space of characters.

If we wanted to generalize it to be able to hold arbitrary characters, we would
need some way to denote the delimiter character `"` in ths string content.

The usual way to do this is via an escape sequence: `\"`, which is understood by
the lexical analyzer as *not* ending the string content.

We can generalize the regular expression in our new `Lit` rule to handle this:

```lalrpop
pub Lit: Lit = <l:r#""(\\\\|\\"|[^"\\])*""#> => l[1..l.len()-1].into();
```

However, depending on your data model, this is not quite right. In particular:
the string is produced still has the escaping backslashes embedded in it.


As a concrete example, with the above definition for `Lit`, this test:

```rust
#[test]
fn popfly() {
    assert_eq!(nobol6::StmtParser::new().parse(r#"z = "\"\\""#), Ok(('z', "\"\\").into()));
}
```

yields this output:

```
thread 'popfly' panicked at 'assertion failed: `(left == right)`
  left: `Ok(Stmt(Var('z'), Lit("\\\"\\\\")))`,
 right: `Ok(Stmt(Var('z'), Lit("\"\\")))`', doc/nobol/src/main.rs:91:5
```

This can be readily addressed by adding some code to post-process the token to remove the
backslashes:

```lalrpop
pub Lit: Lit = <l:r#""(\\\\|\\"|[^"\\])*""#> => Lit(apply_string_escapes(&l[1..l.len()-1]).into());
```

where `apply_string_escapes` is a helper routine that searches for backslashes in the content and performs the corresponding replacement with the character denoted by the escape sequence.
