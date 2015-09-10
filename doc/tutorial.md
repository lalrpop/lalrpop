# LALRPOP

LALRPOP is a parser generator, similar in principle to YACC, Menhir,
and other such programs. In general, it has the grand ambition of
being the most usable parser generator ever. This ambition is most
certainly not fully realized: right now, it's fairly standard, maybe
even a bit subpar in some areas. But hey, it's young. For the most
part, this README is intended to describe the current behavior of
LALRPOP, but in some places it includes notes for planned future
changes.

LALRPOP is looking for contributions as well! If you're interested in
helping out, please come find `nmatsakis` on IRC.

## Quick start: the calculator project

Before diving into the details, let's develop a quick "first project"
with LALRPOP. To start, we'll develop a little calculator for
expressions like "3 - 5 * 2" and so forth. After this, we'll do some
more examples that use some of LALRPOP's more advanced features, like
macros and constructing parse trees. You can find the source code in
the [calculator][] directory.

LALRPOP works as a preprocessor that is integrated with cargo. When
LALRPOP is invoked, it will search your source directory for files
with the extension `lalrpop` and create corresponding `rs` files. So,
for example, if we have a file `calculator.lalrpop`, the preprocessor
will create a Rust file `calculator.rs`. As an aside, the syntax of
LALRPOP intentionally hews fairly close to Rust, so it should be
possible to use the Rust plugin to edit lalrpop files as well, as long
as it's not too picky (the emacs rust-mode, in particular, works just
fine).

### Creating the calculator project and invoking LALRPOP

To start, let's use `cargo new` to make a new project. We'll call it
`calculator1` for now:

```
> cargo new --bin calculator1
```

We now have to edit the generated `calculator1/Cargo.toml` file to
invoke the LALRPOP preprocessor. The resulting file should look
something like:

```
[package]
name = "calculator1"
version = "0.1.0"
authors = ["Niko Matsakis <niko@alum.mit.edu>"]
build = "build.rs" # <-- We added this and everything after!

[build-dependencies]
lalrpop = "0.5.0"
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
nonterminal. Hhere is a simple test that we've added to our
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

### calculator3: Full-featured expressions

Now we are ready to extend our calculator to cover the full range of
arithmetic expressions (well, at least the ones you learned in
elementary school). Here is
[the final calculator example, calculator3][calculator3]:

```rust
use std::str::FromStr;

grammar;

pub Expr: i32 = {
    <l:Factor> "+" <r:Factor> => l+r,
    <l:Factor> "-" <r:Factor> => l-r,
};

Factor: i32 = {
    <l:Term> "*" <r:Term> => l*r,
    <l:Term> "/" <r:Term> => l/r,
};

Term: i32 = {
    Num,
    "(" <Expr> ")"
};

Num: i32 = {
    r"[0-9]+" => i32::from_str(<>).unwrap()
};
```
 
[main]: ./calculator/main.rs
[calculator]: ./calculator/
[calculator1]: ./calculator/src/calculator1.lalrpop
[calculator2]: ./calculator/src/calculator2.lalrpop
[calculator3]: ./calculator/src/calculator3.lalrpop

