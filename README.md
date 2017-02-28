# LALRPOP

[![Join the chat at https://gitter.im/lalrpop/Lobby](https://badges.gitter.im/lalrpop/Lobby.svg)](https://gitter.im/lalrpop/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

![Build status](https://travis-ci.org/nikomatsakis/lalrpop.svg?branch=master)

LALRPOP is a Rust parser generator framework with *usability* as its
primary goal. You should be able to write compact, DRY, readable
grammars. To this end, LALRPOP offers a number of nifty features:

0. Nice error messages in case parser constructor fails. 
1. Macros that let you extract common parts of your grammar. This
   means you can go beyond simple repetition like `Id*` and define
   things like `Comma<Id>` for a comma-separated list of identifiers.
2. Macros can also create subsets, so that you easily do something
   like `Expr<"all">` to represent the full range of expressions, but
   `Expr<"if">` to represent the subset of expressions that can appear
   in an `if` expression.
3. Builtin support for operators like `*` and `?`.
4. Compact defaults so that you can avoid writing action code much of the
   time.
5. Type inference so you can often omit the types of nonterminals.   

Despite its name, LALRPOP in fact uses LR(1) by default (though you
can opt for LALR(1)), and really I hope to eventually move to
something general that can handle all CFGs (like GLL, GLR, LL(\*),
etc).

### Documentation

There is a [tutorial available here](doc/tutorial.md) that covers a
fair bit of the features of LALRPOP. For the more advanced things are
not yet covered, it also points you to tests that may help give you
the idea. I plan eventually to build up a reference manual in the
Wiki, but that's not even started.

### Obligatory disclaimer

LALRPOP is still in its relatively early days. Not all the features I
want are there, and the error messages are sometimes a bit opaque. But
it's quite powerful already. It's also [self-hosting], which is fun.

[self-hosting]: https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop/src/parser/lrgrammar.lalrpop

### Using LALRPOP

#### Configuring cargo

There are two ways to use LALRPOP. The recommended way is to
configure Cargo to automatically change all `.lalrpop` files
into `.rs` files by adding a `build.rs` file. Here is a "cheat sheet"
for how to do that.

This section is for if you already know what you're doing and just
want to copy-and-paste some code for adding LALRPOP to your Cargo
project. To enable LALRPOP, add the following lines to your
`Cargo.toml`:

```
[package]
...
build = "build.rs" # LALRPOP preprocessing

# Add a dependency on the regex crate; this is not
# needed if you are writing your own tokenizer by
# hand (or if you are already using the regex crate)
[dependencies.regex]
version = "0.12.5"

# Add a dependency on the LALRPOP runtime library:
[dependencies.lalrpop-util]
version = "0.12.5"

[build-dependencies.lalrpop]
version = "0.12.5"
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

#### More advanced configuration

The `process_root` applies the default configuration: so it will
transform `.lalrpop` files into `.rs` files *in-place* (in your `src`
directory), and it will only do so if the `.lalrpop` file has actually
changed. But you can also use the [`Configuration`][config] struct to
get more detailed control.

[config]: https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop/src/api/mod.rs

#### Running manually

If you prefer, you can also run the `lalrpop` crate as an
executable. Simply run `cargo install lalrpop` and then you will get a
`lalrpop` binary you can execute, like so:

```
lalrpop file.lalrpop
```

This will generate `file.rs` for you. Note that it only executes if
`file.lalrpop` is newer than `file.rs`; if you'd prefer to execute
unconditionally, pass `-f` (also try `--help` for other options).

