LALRPOP support conditional compilation of public non-terminal declarations via `#[cfg(feature = "FEATURE")]` attributes.
If run in a build script LALRPOP will automatically pickup the features from `cargo` and use those. Alternatively an explicit set of features can be set using the `Configuration` type.

```rust
#[cfg(feature = "FEATURE")]
pub MyRule : () = {
    ...
};
```
