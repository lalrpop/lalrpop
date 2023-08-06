### Customizing the Build Process

When you setup LALRPOP, you create a `build.rs` file that looks something
like this:

```rust
fn main() {
    lalrpop::process_root().unwrap();
}
```

This `process_root()` call simply applies the default configuration:
so it will transform `.lalrpop` files into `.rs` files *in-place* (in
your `src` directory), and it will only do so if the `.lalrpop` file
has actually changed. But you can also use the
[`Configuration`][config] struct to get more detailed control.

[config]: https://docs.rs/lalrpop/*/lalrpop/struct.Configuration.html

For example, to **force** the use of colors in the output (ignoring
the TTY settings), you might make your `build.rs` file look like so:

```rust
fn main() {
    lalrpop::Configuration::new()
        .always_use_colors()
        .process_current_dir();
}
```

### Using the Legacy LALR Parser

By default, LALRPOP uses the [lane table][]
algorithm which is LR(1) but creates much smaller tables. There is no longer
any clear benefit to using the previous LALR implementation but it is still
available.

[lane table]: https://github.com/lalrpop/lalrpop/blob/master/lalrpop/src/lr1/lane_table/README.md

To enable it, build with the `LALRPOP_LANE_TABLE=disabled` environment
variable by setting `std::env::set_var` in your `build.rs` and add the
`#[lalr]` attribute above the `grammar;` declaration in your lalrpop grammar
file.
