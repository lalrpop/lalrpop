# LALRPop

LALRPop is a parser generator framework. Despite its name, it does not
implement the LALR(1) algorithm, but rather LR(1). In fact, it has
grand ambitions of eventually supporting all manner of parser
generation algorithms (including more general variants like LL(*),
GLR, etc), but those may or may not ever come to fruitition. :)

LALRPop has *usability* as a primary goal. You should be able to write
compact, DRY, readable grammars. You shouldn't have to stress about
writing action code or types if you don't want to. To this end,
LALRPop has a number of nifty features:

1. Macros that let you extract common parts of your grammar. This
   means you can go beyond simple repetition like `Id*` and define
   things like `Comma<Id>` for a comma-separated list of identifiers.
2. Macros can also create subsets, so that you easily do something
   like `Expr<"all">` to represent the full range of expressions, but
   `Expr<"if">` to represent the subset of expressions that can appear
   in an `if` expression.
3. Compact defaults so that you can avoid writing action code much of the
   time.
4. Type inference so you can often omit the types of nonterminals etc.   
   
To be clear, LALRPop is still early days. It's kind of spare time
project. But it's coming along pretty quickly, now that a lot of the
tricky stuff is out of the way. I'll update this README more with
better instructions soon.

## Documentation

There is an [in-progress tutorial available here](doc/tutorial.md).
You might also want to look at the examples in the `lalrpop-test`
directory.

## Cargo cheat sheet

This section is for if you already know what you're doing and just
want to copy-and-paste some code for adding LALRPOP to your Cargo
project. To enable LALRPOP, add the following lines to your
`Cargo.toml`:

```
[package]
...
build = "build.rs" # LALRPOP preprocessing

# Add a dependency on the LALRPOP runtime library:
[dependencies.lalrpop-util]
version = "0.4.1"

[build-dependencies.lalrpop]
version = "0.4.1"
```

And create a `build.rs` file that looks like:

```rust
extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
```

(If you already have a `build.rs` file, you should be able to just
call `process_root` in addition to whatever else that file is doing.)

That's it!
