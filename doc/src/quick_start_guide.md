For getting started with LALRPOP, it's probably best if you read
[the tutorial](tutorial/index.md), which will introduce you
to the syntax of LALRPOP files and so forth.

But if you've done this before, or you're just the impatient sort,
here is a quick 'cheat sheet' for setting up your project. First, add
the following lines to your `Cargo.toml`:

```toml
# The generated code depends on lalrpop-util.
[dependencies]
lalrpop-util = "0.20.2"

# Add a build-time dependency on the lalrpop library:
[build-dependencies]
lalrpop = "0.20.2"
# If you are supplying your own external lexer you can disable default features so that the
# built-in lexer feature is not included
# lalrpop = { version = "0.20.2", default-features = false }
```

Next create a [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html) file
that looks like:

```rust
fn main() {
    lalrpop::process_root().unwrap();
}
```

(If you already have a `build.rs` file, you should be able to just
call `process_root` in addition to whatever else that file is doing.)

In this case, `process_root` simply uses the default settings, which takes
files in `src/` ending with the `.lalrpop` extension, and generates
corresponding Rust source files with the same name in `OUT_DIR`. If you want to
configure how LALRPOP executes, see the [advanced setup](advanced_setup.md)
section.

The [`lalrpop_mod!`][lalrpop_mod] macro generates a wrapper module in your
crate so that you can use the generated parser from your code. For example,
if the source grammar is located in `grammar.lalrpop`, adding the following line
to `lib.rs` will create a corresponding `grammar` submodule (note that you can
also add this line to a `foo.rs` module definition instead, which will then
create a submodule `foo::grammar`):

```rust
lalrpop_mod!(grammar);
```

[lalrpop_mod]: https://docs.rs/lalrpop-util/latest/lalrpop_util/macro.lalrpop_mod.html

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
