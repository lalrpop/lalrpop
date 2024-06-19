# Generating files in the source tree

Up to version 0.15, LALRPOP was generating its files in the same directory
of the input files. Since 0.16, files are generated in the Cargo's
**output directory**.

If you want to keep the previous behaviour, you can use `generate_in_source_tree`
in your configuration:

```rust
fn main() {
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process().unwrap();
}
```

For each `foo.lalrpop` file you can simply have `mod foo;` in your source tree.
The `lalrpop_mod` macro is not useful in this mode.
