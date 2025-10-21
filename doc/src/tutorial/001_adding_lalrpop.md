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

```console
cargo new --bin calculator
```

We now have to edit the generated [`calculator/Cargo.toml`][calculator-Cargo.toml]
file to invoke the LALRPOP preprocessor. The resulting file should
look something like:

[calculator-Cargo.toml]: https://github.com/lalrpop/lalrpop/blob/master/doc/calculator/Cargo.toml

```toml
[package]
name = "calculator"
version = "0.1.0"
authors = ["Niko Matsakis <niko@alum.mit.edu>"]
edition = "2024"

[build-dependencies] # <-- We added this and everything after!
lalrpop = "0.23.0"

[dependencies]
lalrpop-util = { version = "0.21.0", features = ["lexer", "unicode"] }
```

Cargo can run [build scripts] as a pre-processing step,
named `build.rs` by default. The `[build-dependencies]`
section specifies the dependencies for build scripts -- in this
case, just LALRPOP.

[build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

The `[dependencies]` section describes the dependencies that LALRPOP
needs at runtime. All LALRPOP parsers require at least the
`lalrpop-util` crate.

Next we have to add `build.rs` itself. For those unfamiliar with
[this feature], the `build.rs` file should be placed next to your `Cargo.toml`
file and not inside the `src` folder with the rest of your Rust code. This
should just look like the following:

[this feature]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

```rust
fn main() {
    lalrpop::process_root().unwrap();
}
```

The function `process_root` processes your `src` directory, converting
all `lalrpop` files into `rs` files, and saving them to `OUT_DIR`. It is smart
enough to check timestamps and do nothing if the `rs` file is newer than the
`lalrpop` file, and to mark the generated `rs` file as read-only. It returns an
`io::Result<()>`, so the `unwrap()` call just asserts that no
file-system errors occurred.

The [`lalrpop_mod!`][lalrpop_mod] macro generates a wrapper module in your
crate so that you can use the generated parser from your code. For example,
if the source grammar is located in `grammar.lalrpop`, adding the following line
to `lib.rs` will create a corresponding `grammar` submodule (note that you can
also add this line to a `foo.rs` module definition instead, which will then
create a submodule `foo::grammar`):

```rust
lalrpop_mod!(grammar);
```

## Attributes

The code that LALRPOP generates sometimes falls afoul of code linters for
cosmetic reasons which don't apply to macro generated code. Currently, there is
not a blanket `#[automatically_derived]` attribute in the generated parser
because action code provided by the user may be worth linting on. Users can
provide any number of attributes which they would like applied to the generated
module. This is particularly useful for suppressing noisy lints.

```rust
lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    parser
);
```

[lalrpop_mod]: https://docs.rs/lalrpop-util/latest/lalrpop_util/macro.lalrpop_mod.html
