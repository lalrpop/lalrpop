# Adding LALRPOP to your `Cargo.toml` file

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

We now have to edit the generated [`calculator/Cargo.toml`][calculator-Cargo.toml]
file to invoke the LALRPOP preprocessor. The resulting file should
look something like:

[calculator-Cargo.toml]: https://github.com/lalrpop/lalrpop/blob/master/doc/calculator/Cargo.toml

```
[package]
name = "calculator"
version = "0.1.0"
authors = ["Niko Matsakis <niko@alum.mit.edu>"]
edition = "2021"

[build-dependencies] # <-- We added this and everything after!
lalrpop = "0.20.0"

[dependencies]
lalrpop-util = { version = "0.20.0", features = ["lexer"] }
```

Cargo can run [build scripts] as a pre-processing step,
named `build.rs` by default. The `[build-dependencies]`
section specifies the dependencies for build scripts -- in this
case, just LALRPOP.

[build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

The `[dependencies]` section describes the dependencies that LALRPOP
needs at runtime. All LALRPOP parsers require at least the
`lalrpop-util` crate. In addition, if you don't want to write the
lexer by hand, you need to add a dependency on the regex crate. (If
you don't know what a lexer is, don't worry, it's not important just
now, though we will cover it in [the next section]; if you *do*
know what a lexer is, and you want to know how to write a lexer by
hand and use it with LALRPOP, then check out the [lexer tutorial].)

[the next section]: 002_paren_numbers.md
[lexer tutorial]: ../lexer_tutorial/index.md

Next we have to add `build.rs` itself. For those unfamiliar with [this feature], the `build.rs` file
should be placed next to your `Cargo.toml` file and not inside the `src` folder with the rest of
your Rust code. This should just look like the following:

[this feature]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

```rust
fn main() {
    lalrpop::process_root().unwrap();
}
```

The function `process_root` processes your `src` directory, converting
all `lalrpop` files into `rs` files. It is smart enough to check
timestamps and do nothing if the `rs` file is newer than the `lalrpop`
file, and to mark the generated `rs` file as read-only. It returns an
`io::Result<()>`, so the `unwrap()` call just asserts that no
file-system errors occurred.

*NOTE:* On Windows, the necessary APIs are not yet stable, so
timestamp checking is disabled.
