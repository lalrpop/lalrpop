# LALRPOP

LALRPOP is a parser generator, similar in principle to [YACC], [ANTLR], [Menhir],
and other such programs. In general, it has the grand ambition of
being the most usable parser generator ever. This ambition is most
certainly not fully realized: right now, it's fairly standard, maybe
even a bit subpar in some areas. But hey, it's young. For the most
part, this README is intended to describe the current behavior of
LALRPOP, but in some places it includes notes for planned future
changes.

LALRPOP is looking for contributions as well! If you're interested in
helping out, please come find `nmatsakis` on
[Mozilla's IRC server][IRC].

[YACC]: http://dinosaur.compilertools.net/yacc/
[ANTLR]: http://www.antlr.org/
[Menhir]: http://gallium.inria.fr/~fpottier/menhir/
[IRC]: https://wiki.mozilla.org/IRC

## Table of contents

Here is a brief listing of the examples and what they cover. If you
are already familiar with the basics of parser generators, you may
want to skip ahead, or just look at the LALRPOP sources:

- Crash course on grammars and parser generators
  ([read](#crash-course)).
- Adding LALRPOP to your [`Cargo.toml`][cargotoml] file
  ([read](#adding-lalrpop)).
- calculator1: Parsing parenthesized numbers like `22` and `(22)` and `((22))`
  ([source][calculator1], [read](#calculator1))
- calculator2: LALRPOP shorthands and type inference
  ([source][calculator2], [read](#calculator2))
- calculator3: Handling full-featured expressions like `22+44*66`
  ([source][calculator3], [read](#calculator3))
- calculator4: Building ASTs
  ([source][calculator4], [read](#calculator4))
- calculator5: Macros
  ([source][calculator5], [read](#calculator5))

This tutorial is still incomplete. Here are some topics that I aim to
cover when I get time to write about them:

- Advice for resolving shift-reduce and reduce-reduce conflicts
- Passing state and type/lifetime parameters to your action code (see e.g. [this test](https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop-test/src/expr_arena.lalrpop) invoked [from here][]).
- Location tracking with `@L` and `@R` (see e.g. [this test](https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop-test/src/intern_tok.lalrpop)).
- Integrating with external tokenizers (see e.g. [this test](https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop-test/src/expr.lalrpop) invoked [from here][]).
- Conditional macros (no good test to point you at yet, sorry)
- Fallible action code that produces a `Result` (see e.g. [this test](https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop-test/src/error.lalrpop) invoked [from here][]).
- Converting to use `LALR(1)` instead of `LR(1)` (see e.g. [this test](https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop-test/src/expr_lalr.lalrpop) invoked [from here][]).
- Plans for future features

[from here]: https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop-test/src/main.rs

<a id="crash-course"></a>
### Crash course on grammars and parser generators

If you've never worked with a parser generator before, or aren't
really familiar with context-free grammars, this section is just a
*very brief* introduction into the basic idea. Basically a grammar is
a nice way of writing out what kinds of inputs are legal.  In our
example, we want to support parenthesized numbers, so things like
`123`, `(123)`, etc. We can express this with a simple grammar like:

```
Term = Num | "(" Term ")"
```

Here we say we are trying to parse a *term*, and a term can either be
a number (`Num`) or some other term enclosing in parentheses (here I
did not define what a number is, but in the real LALRPOP example we'll
do that with a regular expression).  Now imagine a potential input
like `((123))`. We can show how this would be parsed by writing out
something called a "parse tree":

```
(  (  1  2  3  )  )
|  |  |     |  |  |
|  |  +-Num-+  |  |
|  |     |     |  |
|  |   Term    |  |
|  |     |     |  |
|  +---Term----+  |
|        |        |
+------Term-------+
```

Here you can see that we parsed `((123))` by finding a `Num` in the
middle, calling that `Num` a `Term`, and matching up the parentheses
to form two more terms on top of that.

Nnote that this parse tree is not a data structure but more a
visualization of the parse. I mean, you *can* build up a parse tree as
a data structure, but typically you don't want to: it is more detailed
than you need. For example, you may not be that interested in the
no-op conversion from a `Num` to a `Term`. The other weird thing about
a parse tree is that it is intimately tied to your grammar, but often
you have some existing data structures you would like to parse into --
so if you built up a parse tree, you'd then have to convert from the
parse tree into those data structures, and that might be annoying.

Therefore, what a parser generator usually does, is instead let you
choose how to represent each node in the parse tree, and how to do the
conversions. You give each nonterminal a type, which can be any Rust
type, and you write code that will execute each time a new node in the
parse tree would have been constructed. In fact, in the examples that follow, we'll
eventually build up something like a parse tree, but in the beginning, we won't
do that at all. Instead, we'll represent each number and term as an `i32`,
and we'll propagate this value around.

To make this a bit more concrete, here's a version of the grammar above
written in LALRPOP notation (we'll revisit this again in more detail of course).
You can see that the `Term` nonterminal has been given the type `i32`,
and that each of the definitions has some code that follows a `=>` symbol.
This is the code that will execute to convert from the thing that was matched
(like a number, or a parenthesized term) into an `i32`:

```rust
Term: i32 = {
    Num => /* ... number code ... */,
    "(" Term ")" => /* ... parenthesized code ... */,
};
```

OK, that's enough background, let's do this for real!

<a id="adding-lalrpop"></a>
### Adding LALRPOP to your Cargo project

LALRPOP works as a preprocessor that is integrated with cargo. When
LALRPOP is invoked, it will search your source directory for files
with the extension `lalrpop` and create corresponding `rs` files. So,
for example, if we have a file `calculator.lalrpop`, the preprocessor
will create a Rust file `calculator.rs`. As an aside, the syntax of
LALRPOP intentionally hews fairly close to Rust, so it should be
possible to use the Rust plugin to edit lalrpop files as well, as long
as it's not too picky (the emacs rust-mode, in particular, works just
fine).

To start, let's use `cargo new` to make a new project. We'll call it
`calculator`:

```
> cargo new --bin calculator
```

We now have to edit the generated [`calculator/Cargo.toml`][cargotoml]
file to invoke the LALRPOP preprocessor. The resulting file should
look something like:

```
[package]
name = "calculator"
version = "0.1.0"
authors = ["Niko Matsakis <niko@alum.mit.edu>"]
build = "build.rs" # <-- We added this and everything after!

[build-dependencies]
lalrpop = "0.5.0"

[dependencies]
lalrpop-util = "0.5.0"
```

Adding a `build` directive to the `[package]` section tells Cargo to
run `build.rs` as a pre-processing step. The `[build-dependencies]`
section that specifies the dependencies for `build.rs` -- in this
case, just LALRPOP.

Next we have to add `build.rs` itself. This should just look like the
following:

```rust
extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
```

The function `process_root` processes your `src` directory, converting
all `lalrpop` files into `rs` files. It is smart enough to check
timestamps and do nothing if the `rs` file is newer than the `lalrpop`
file, and to mark the geneated `rs` file as read-only. It returns an
`io::Result<()>`, so the `unwrap()` call just asserts that no
file-system errors occurred.

*NOTE:* On Windows, the necessary APIs are not yet stable, so
timestamp checking is disabled.

<a id="calculator1"></a>
### calculator1: Parsing parenthesized numbers

OK, now we're all set to start making a LALRPOP grammar. Before we
tackle full expressions, let's start with something simple -- really
simple. Let's just start with parenthesized integers, like `123` or
`(123)` or even (hold on to your hats) `(((123)))`. Wow.

To handle this, we'll need a to add a
[`calculator1.lalrpop`][calculator1] as shown below. Note: to make
explaining things easier, this version is maximally explicit; the next
section will make it shorter by employing some shorthands that LALRPOP
offers.

```rust
use std::str::FromStr;

grammar;

pub Term: i32 = {
    <n:Num> => n,
    "(" <t:Term> ")" => t,
};

Num: i32 = <s:r"[0-9]+"> => i32::from_str(s).unwrap();
```

Let's look at this bit by bit. The first part of the file is the `use`
statement and the `grammar` declaration. You'll find these at the top
of every LALRPOP grammar. Just as in Rust, the `use` statement just
brings names in scope: in fact, these `use` statements are just copied
verbatim into the generated Rust code as needed.

*A note about underscores and hygiene:* LALRPOP generates its own
names that begin with at least two leading underscores. To avoid
conflicts, it will insert more underscores if it sees that you use
identifiers that also have two underscores. But if you use glob
imports that bring in names beginning with `__`, you may find you have
invisible conflicts. To avoid this, don't use a glob (or define some
other name with two underscores somewhere else).

**Nonterminal declarations.** After the `grammar` declaration comes a
series of *nonterminal declarations*.  This grammar has two
nonterminals, `Term` and `Num`. A nonterminal is just a name that we
give to something which can be parsed. Each nonterminal is then
defined in terms of other things.

Let's start with `Num`, at the end of the file, which is declared
as follows:

```rust
Num: i32 =
    <s:r"[0-9]+"> => i32::from_str(s).unwrap();
```

This declaration says that the type of `Num` is `i32`. This means that
when we parse a `Num` from the input text, we will produce a value of
type `i32`. The definition of `Num` is `<s:r"[0-9]+">`.  Let's look at
this from the inside out. The notation `r"[0-9]+"` is a regex literal
-- this is the same as a Rust raw string. (And, just as in Rust, you
can use hashes if you need to embed quotes, like `r#"..."..."#`.)  It
will match against a string of characters that matches the regular
expression: in this case, some number of digits. The result of this
match will be a slice `&'input str` into the input text that we are
parsing (no copies are made).

This regular expression is wrapped in angle brackets and labeled:
`<s:r"[0-9+]">`. In general, angle brackets are used in LALRPOP to
indicate the values that will be used by the *action code* -- that is,
the code that executes when a `Num` is parsed.  In this case, the
string that matches the regular expression is bound to the name `s`,
and the action code `i32::from_str(s).unwrap()` parses that string and
creates an `i32`. Hence the result of parsing a `Num` is an `i32`.

OK, now let's look at the nonterminal `Term`:

```rust
pub Term: i32 = {
    <n:Num> => n,
    "(" <t:Term> ")" => t,
};
```

First, this nonterminal is declared as `pub`. That means that LALRPOP
will generate a public function (named, as we will see, `parse_Term`)
that you can use to parse strings as `Term`. Private nonterminals
(like `Num`) can only be used within the grammar itself, not from
outside.

The `Term` nonterminal has two alternative definitions, which is
indicated by writing `{ alternative1, alternative2 }`. In this case,
the first alternative is `<n:Num>`, meaning that a term can be just a
number; so `22` is a term. The second alternative is `"(" <t:Term>
")"`, which indicates that a term can also be a parenthesized term; so
`(22)` is a term, as is `((22))`, `((((((22))))))`, and so on.

**Invoking the parser.** OK, so we wrote our parser, how do we use it?
For every nonterminal `Foo` declared as `pub`, LALRPOP will export a
`parse_Foo` fn that you can call to parse a string as that
nonterminal. Here is a simple test that we've added to our
[`main.rs`][main] file which uses this function to test our `Term`
nonterminal:

```rust
pub mod calculator1; // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::parse_Term("22").is_ok());
    assert!(calculator1::parse_Term("(22)").is_ok());
    assert!(calculator1::parse_Term("((((22))))").is_ok());
    assert!(calculator1::parse_Term("((22)").is_err());
}
```

The full signature of the parse function looks like this:

```rust
fn parse_Term<'input>(input: &'input str)
                     -> Result<i32, ParseError<usize,(usize, &'input str),()>>
                     //        ~~~  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                     //         |                       |
                     // Result upon success             |
                     //                                 |
                     //             Error enum defined in the lalrpop_util crate
{
    ...
}
```

<a id="calculator2"></a>
### calculator2: Employing shorthands and type-inference

OK, now that we understand [the calculator1 example][calculator1], let's
look at some of the shorthands that LALRPOP offers to make it more concise.
This code is found in [the calculator2 demo][calculator2].

To start, let's look at the definition of `Term` we saw before:

```rust
pub Term: i32 = {
    <n:Num> => n,
    "(" <t:Term> ")" => t,
};
```

The action code here is somewhat interesting. In both cases, it's not
doing any new work, it's just selecting a value that was produced by
another nonterminal. This turns out to be pretty common. So common,
in fact, that LALRPOP offers some shorthand notation for it. Here is
the definition of `Term` from the calculator2 demo:

```rust
pub Term = { Num, "(" <Term> ")" };
```

Here, we have no action code at all. If there is no action code,
LALRPOP synthesizes action code which just takes the value of the
things being matched. In the case of the first alternative, `Num`,
there is only one thing being matched, so that means that `Term` will
produce the same value as the `Num` we parsed, whatever that was.

In the case of the second alternative, `"(" <Term> ")"`, there are
three things being matched. Here we use the angle brackets to select
which item(s) we want to take the value of --- we selected only one,
so the result is that we take the value of the `Term` we parsed. If we
selected more than one, the result would be a tuple of all the
selected items.  If we did not select any (i.e., `"(" Term ")"`), the
result would be a tuple of all the items, and hence the result would
be of type `(&'static str, i32, &'static str)`.

Speaking of types, you may have noticed that `Term` has no type
annotation. Since we didn't write out own action code, we can omit the
type annotation and let LALRPOP infer it for us. In this case, LALRPOP
can see that `Term` must have the same type as `Num`, and hence that
the type must be `i32`.

OK, let's look at the definition of `Num` we saw before from calculator1:

```rust
Num: i32 = <s:r"[0-9]+"> => i32::from_str(s).unwrap();
```

This definition too can be made somewhat shorter. In calculator2, you will
find:

```rust
Num: i32 = r"[0-9]+" => i32::from_str(<>).unwrap();
```

Here, instead of giving the regular expression a name `s`, we modified
the action code to use the funky expression `<>`. This is a shorthand
that says "synthesize names for the matched values and insert a
comma-separated list here". In this case, there is only one matched
value, `r"[0-9]+"`, and it produces a `&'input str`, so LALRPOP will
insert a synthetic variable for that value. Note that we still have
custom action code, so we still need a type annotation.

To control what values are selected when you use the `<>` expression
in your action code, you can use angle brackets as we saw before.
Here are some examples of alternatives and how they are expanded to
give you the idea:

| Alternative          | Equivalent to              |
| -----------          | -------------              |
| `A => bar(<>)`       | `<a:A> => bar(a)`          |
| `A B => bar(<>)`     | `<a:A> <b:B> => bar(a, b)` |
| `<A> B => bar(<>)`   | `<a:A> B => bar(a)`        |
| `<A> <B> => bar(<>)` | `<a:A> <b:B> => bar(a, b)` |

<a id="calculator3"></a>
### calculator3: Full-featured expressions

Now we are ready to extend our calculator to cover the full range of
arithmetic expressions (well, at least the ones you learned in
elementary school). Here is
[the next calculator example, calculator3][calculator3]:

```rust
use std::str::FromStr;

grammar;

pub Expr: i32 = {
    <l:Expr> "+" <r:Factor> => l+r,
    <l:Expr> "-" <r:Factor> => l-r,
    Factor,
};

Factor: i32 = {
    <l:Factor> "*" <r:Term> => l*r,
    <l:Factor> "/" <r:Term> => l/r,
    Term,
};

Term: i32 = {
    Num,
    "(" <Expr> ")"
};

Num: i32 = {
    r"[0-9]+" => i32::from_str(<>).unwrap()
};
```

Perhaps the most interesting thing about this example is the way it
encodes precedence. The idea of precedence of course is that in an
expression like `2+3*4`, we want to do the multiplication first, and
then the addition. LALRPOP doesn't have any built-in features for
giving precedence to operators, mostly because I consider those to be
creepy, but it's pretty straightforward to express precedence in your
grammar by structuring it in tiers -- for example, here we have the
nonterminal `Expr`, which covers all expressions. It consists of a series
of factors that are added or subtracted from one another. A `Factor`
is then a series of terms that are multiplied or divided. Finally, a
`Term` is either a single number or, using parenthesis, an entire expr.

Abstracting from this example, the typical pattern for encoding
precedence is to have one nonterminal per precedence level, where you
begin with the operators of lowest precedence (`+`, `-`), add in the
next highest precedence level (`*`, `/`), and finish with the bare
"atomic" expressions like `Num`. Finally, you add in a parenthesized
version of your top-level as an atomic expression, which lets people
reset.

To see why this works, consider the two parse possible parse trees
for something like `2+3*4`:

```
2 + 3   *    4      2   +  3   *    4
| | |   |    |      |   |  |   |    |
| | +-Factor-+  OR  +-Expr-+   |    |
| |     |               |      |    |
+-Expr -+               +----Factor-+
```

In the first one, we give multiplication higher precedence, and in the
second one, we (incorrectly) give addition higher precedence. If you
look at the grammar now, you can see that the second one is
impossible: a `Factor` cannot have an `Expr` as its left-hand side.
This is the purpose of the tiers: to force the parser into the
precedence you want.

<a id="calculator4"></a>
### calculator4: Building up an AST

Of course, most of the time, when you're parsing you don't want to
compute a value, you want to build up some kind of data structure.
Here's a quick example to show how that is done in LALRPOP.  First, we
need to *define* the data structure we will build. We're going to use
a very simple `enum`:

```rust
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}
```

We put this code into [an `ast.rs` module][astrs] in our project,
along with some `Debug` impls so that things pretty-print nicely. Now
we will create the [calculator4] example, which will build up this
tree. To start, let's just look at the `Expr` nonterminal, which will
show you most everything of how it is done (the most interesting lines
have been flagged with comments):

```rust
use std::str::FromStr;
use ast::{Expr, Opcode}; // (0)

grammar;

pub Expr: Box<Expr> = { // (1)
    Expr ExprOp Factor => Box::new(Expr::Op(<>)) // (2)
    Factor,
};

ExprOp: Opcode = { // (3)
    "+" => Opcode::Add,
    "-" => Opcode::Sub,
};
```

First off, we have to import these new names into our file by adding a
`use` statement (0). Next, we want to produce `Box<Expr>` values, so
we change the type of `Expr` (and `Factor` and `Term`) to `Box<Expr>`
(1). The action code changes accordingly in (2); here we've used the
`<>` expansion to supply three arguments to `Expr::Op`. Finally, just
for concision, we introduced an `ExprOp` nonterminal (3) to cover the
two opcodes, which now trigger the same action code (before they
triggered different action code, so we could do an addition vs a
subtraction).

The definition of `Factor` is transformed in a similar way:

```rust
Factor: Box<Expr> = {
    Factor FactorOp Term => Box::new(Expr::Op(<>)),
    Term,
};

FactorOp: Opcode = {
    "*" => Opcode::Mul,
    "/" => Opcode::Div,
};
```

And finally we adjust the definitions of `Term` and `Num`. Here, we
convert from a raw `i32` into a `Box<Expr>` when we transition from
`Num` to `Term` (4):

```rust
Term: Box<Expr> = {
    Num => Box::new(Expr::Number(<>)), // (4)
    "(" <Expr> ")"
};

Num: i32 = {
    r"[0-9]+" => i32::from_str(<>).unwrap()
};
```

And that's it! Now we can test it by adding some code to our
[main.rs][main] file that parses an expression and formats it using
the `Debug` impl:

```rust
pub mod calculator4;
pub mod ast;

#[test]
fn calculator4() {
    assert_eq!(&format!("{:?}", calculator4::parse_Expr("22 * 44 + 66").unwrap()),
               "((22 * 44) + 66)");
}
```

<a id="calculator5"></a>
### calculator5: Macros

Frequently when writing grammars we encounter repetitive constructs
that we would like to copy-and-paste. A common example is defining
something like a "comma-separated list". Imagine we wanted to parse a
comma-separated list of expressions (with an optional trailing comma,
of course).  If we had to write this out in full, it would look
something like:

```rust
Exprs: Vec<Box<Expr>> = {
    Exprs "," Expr => ...,
    Expr => vec![<>],
}
```

Of course, this doesn't handle trailing commas, and I've omitted the
action code. If we added those, it would get a bit more
complicated. So far, this is fine, but then what happens when we later
want a comma-separated list of terms? Do we just copy-and-paste
everything?

LALRPOP offers a better option. You can define macros. In fact,
LALRPOP comes with four macros builtin: `*`, `?`, `+`, and `(...)`. So
you can write something like `Expr?` to mean "an optional
`Expr`". This will have type `Option<Box<Expr>>` (since `Expr` alone
has type `Box<Expr>`).  Similarly, you can write `Expr*` or `Expr+` to
get a `Vec<Expr>` (with minimum length 0 and 1 respectively). The
final macro is parentheses, which is a shorthand for creating a new
nonterminal.  This lets you write things like `(<Expr> ",")?` to mean
an "optionally parse an `Expr` followed by a comma". Note the angle
brackets around `Expr`: these ensures that the value of the `(<Expr>
",")` is the value of the expression, and not a tuple of the
expression and the comma. This means that `(<Expr> ",")?` would have
the type type `Option<Box<Expr>>` (and not `Option<(Box<Expr>, &'input
str)>`).

Using these operations we can define `Exprs` in terms of a macro
`Comma<T>` that creates a comma-separated list of `T`, whatever `T` is
(this definition appears in [calculator5]):

```rust
pub Exprs = Comma<Expr>; // (0)

Comma<T>: Vec<T> = { // (1)
    <v:(<T> ",")*> <e:T?> => match e { // (2)
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
};
```

The definition of `Exprs` on line (0) is fairly obvious, I think. It
just uses a macro `Comma<Expr>`. Let's take a look then at the
definition of `Comma<T>` on line (1). This is sort of dense, so let's
unpack it. First, `T` is some terminal or nonterminal, but note that
we can also use it as a type: when the macro is expanded, the `T` in
the type will be replaced with "whatever the type of `T` is".

Next, on (2), we parse `<v:(<T> ",")*> <e:T?>`.  That's a lot of
symbols, so let's first remove all the angle brackets, which just
serve to tell LALRPOP what values you want to propagate and which you
want to discard. In that case, we have: `(T ",")* e?`. Hopefully you
can see that this matches a comma-separated list with an optional
trailing comma. Now let's add those angle-brackets back in. In the
parentheses, we get `(<T> ",")*` -- this just means that we keep the
value of the `T` but discard the value of the comma when we build our
vector. Then we capture that vector and call it `v`: `<v:(<T> ",")*>`.
Finally, we capture the optional trailing element `e`: `<e:T?>`. This
means the Rust code has two variables available to it, `v: Vec<T>` and
`e: Option<T>`. The action code itself should then be fairly clear --
if `e` is `Some`, it appends it to the vector and returns the result.

As another example of using macros, you may recall the precedence
tiers we saw in [calculator4] (`Expr`, `Factor`, etc), which had a
sort of repetitive structure. You could factor that out using a
macro. In this case, it's a recursive macro:

```rust
Tier<Op,NextTier>: Box<Expr> = {
    Tier<Op,NextTier> Op NextTier => Box::new(Expr::Op(<>)),
    NextTier
};

Expr = Tier<ExprOp, Factor>;
Factor = Tier<FactorOp, Term>;

ExprOp: Opcode = { // (3)
    "+" => Opcode::Add,
    "-" => Opcode::Sub,
};

FactorOp: Opcode = {
    "*" => Opcode::Mul,
    "/" => Opcode::Div,
};
```

And, of course, we have to add some tests to [main.rs file][main]:

```rust
pub mod calculator5;

#[test]
fn calculator5() {
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("").unwrap()),
               "[]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("22 * 44 + 66").unwrap()),
               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("22 * 44 + 66,").unwrap()),
               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("22 * 44 + 66, 13*3").unwrap()),
               "[((22 * 44) + 66), (13 * 3)]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("22 * 44 + 66, 13*3,").unwrap()),
               "[((22 * 44) + 66), (13 * 3)]");
}
```

[main]: ./calculator/main.rs
[calculator]: ./calculator/
[cargotoml]: ./calculator/Cargo.toml
[calculator1]: ./calculator/src/calculator1.lalrpop
[calculator2]: ./calculator/src/calculator2.lalrpop
[calculator3]: ./calculator/src/calculator3.lalrpop
[calculator4]: ./calculator/src/calculator4.lalrpop
[calculator5]: ./calculator/src/calculator5.lalrpop
[calculator6]: ./calculator/src/calculator6.lalrpop
[astrs]: ./calculator/src/ast.rs
